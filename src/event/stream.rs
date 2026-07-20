use std::{
    io,
    pin::Pin,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self, SyncSender},
        Arc, Mutex,
    },
    task::{Context, Poll},
    thread,
    time::Duration,
};

use futures_core::stream::Stream;

use crate::event::{
    filter::EventFilter,
    internal::{self, InternalEvent},
    sys::Waker,
    Event,
};

/// A stream of `Result<Event>`.
///
/// **This type is not available by default. You have to use the `event-stream` feature flag
/// to make it available.**
///
/// It implements the [Stream](futures_core::stream::Stream)
/// trait and allows you to receive [`Event`]s with [`async-std`](https://crates.io/crates/async-std)
/// or [`tokio`](https://crates.io/crates/tokio) crates.
///
/// Check the [examples](https://github.com/crossterm-rs/crossterm/tree/master/examples) folder to see how to use
/// it (`event-stream-*`).
#[derive(Debug)]
pub struct EventStream {
    poll_internal_waker: Waker,
    stream_wake_task_executed: Arc<AtomicBool>,
    stream_wake_task_should_shutdown: Arc<AtomicBool>,
    stream_wake_task_error: Arc<Mutex<Option<io::Error>>>,
    task_sender: SyncSender<Task>,
}

impl Default for EventStream {
    fn default() -> Self {
        let (task_sender, receiver) = mpsc::sync_channel::<Task>(1);

        thread::spawn(move || {
            while let Ok(task) = receiver.recv() {
                run_stream_wake_task(task, || internal::poll(None, &EventFilter));
            }
        });

        EventStream {
            poll_internal_waker: internal::lock_event_reader().waker(),
            stream_wake_task_executed: Arc::new(AtomicBool::new(false)),
            stream_wake_task_should_shutdown: Arc::new(AtomicBool::new(false)),
            stream_wake_task_error: Arc::new(Mutex::new(None)),
            task_sender,
        }
    }
}

fn wait_until_ready_or_error(
    should_shutdown: &AtomicBool,
    mut poll: impl FnMut() -> io::Result<bool>,
) -> Option<io::Error> {
    while !should_shutdown.load(Ordering::SeqCst) {
        let result = poll();
        if should_shutdown.load(Ordering::SeqCst) {
            return None;
        }

        match result {
            Ok(false) => {}
            Ok(true) => return None,
            Err(error) => return Some(error),
        }
    }
    None
}

fn run_stream_wake_task(task: Task, poll: impl FnMut() -> io::Result<bool>) {
    if let Some(error) = wait_until_ready_or_error(&task.stream_wake_task_should_shutdown, poll) {
        *task.stream_wake_task_error.lock().unwrap() = Some(error);
    }
    task.stream_wake_task_executed
        .store(false, Ordering::SeqCst);
    task.stream_waker.wake();
}

impl EventStream {
    /// Constructs a new instance of `EventStream`.
    pub fn new() -> EventStream {
        EventStream::default()
    }
}

struct Task {
    stream_waker: std::task::Waker,
    stream_wake_task_executed: Arc<AtomicBool>,
    stream_wake_task_should_shutdown: Arc<AtomicBool>,
    stream_wake_task_error: Arc<Mutex<Option<io::Error>>>,
}

// Note to future me
//
// We need two wakers in order to implement EventStream correctly.
//
// 1. futures::Stream waker
//
// Stream::poll_next can return Poll::Pending which means that there's no
// event available. We are going to spawn a thread with the
// poll_internal(None, &EventFilter) call. This call blocks until an
// event is available and then we have to wake up the executor with notification
// that the task can be resumed.
//
// 2. poll_internal waker
//
// There's no event available, Poll::Pending was returned, stream waker thread
// is up and sitting in the poll_internal. User wants to drop the EventStream.
// We have to wake up the poll_internal (force it to return Ok(false)) and quit
// the thread before we drop.
impl Stream for EventStream {
    type Item = io::Result<Event>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if let Some(error) = self.stream_wake_task_error.lock().unwrap().take() {
            return Poll::Ready(Some(Err(error)));
        }

        let result = match internal::poll(Some(Duration::from_secs(0)), &EventFilter) {
            Ok(true) => match internal::read(&EventFilter) {
                Ok(InternalEvent::Event(event)) => Poll::Ready(Some(Ok(event))),
                Err(e) => Poll::Ready(Some(Err(e))),
                #[cfg(unix)]
                _ => unreachable!(),
            },
            Ok(false) => {
                if !self
                    .stream_wake_task_executed
                    .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
                    // https://github.com/rust-lang/rust/issues/80486#issuecomment-752244166
                    .unwrap_or_else(|x| x)
                {
                    let stream_waker = cx.waker().clone();
                    let stream_wake_task_executed = self.stream_wake_task_executed.clone();
                    let stream_wake_task_should_shutdown =
                        self.stream_wake_task_should_shutdown.clone();
                    let stream_wake_task_error = self.stream_wake_task_error.clone();

                    stream_wake_task_should_shutdown.store(false, Ordering::SeqCst);

                    let _ = self.task_sender.send(Task {
                        stream_waker,
                        stream_wake_task_executed,
                        stream_wake_task_should_shutdown,
                        stream_wake_task_error,
                    });
                }
                Poll::Pending
            }
            Err(e) => Poll::Ready(Some(Err(e))),
        };
        result
    }
}

impl Drop for EventStream {
    fn drop(&mut self) {
        self.stream_wake_task_should_shutdown
            .store(true, Ordering::SeqCst);
        let _ = self.poll_internal_waker.wake();
    }
}

#[cfg(test)]
mod tests {
    use std::{
        cell::Cell,
        sync::atomic::{AtomicUsize, Ordering},
        task::Wake,
    };

    use super::*;

    #[derive(Default)]
    struct WakeCounter(AtomicUsize);

    impl Wake for WakeCounter {
        fn wake(self: Arc<Self>) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn shutdown_does_not_start_another_poll() {
        let should_shutdown = AtomicBool::new(true);
        let polls = Cell::new(0);

        let error = wait_until_ready_or_error(&should_shutdown, || {
            polls.set(polls.get() + 1);
            Ok(false)
        });

        assert!(error.is_none());
        assert_eq!(polls.get(), 0);
    }

    #[test]
    fn shutdown_during_poll_suppresses_the_poll_error() {
        let should_shutdown = AtomicBool::new(false);

        let error = wait_until_ready_or_error(&should_shutdown, || {
            should_shutdown.store(true, Ordering::SeqCst);
            Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "poll was woken for shutdown",
            ))
        });

        assert!(error.is_none());
    }

    #[test]
    fn wake_task_delivers_poll_error_to_stream_consumer() {
        let wake_counter = Arc::new(WakeCounter::default());
        let polls = Cell::new(0);
        let stream_wake_task_executed = Arc::new(AtomicBool::new(true));
        let stream_wake_task_should_shutdown = Arc::new(AtomicBool::new(false));
        let stream_wake_task_error = Arc::new(Mutex::new(None));

        run_stream_wake_task(
            Task {
                stream_waker: wake_counter.clone().into(),
                stream_wake_task_executed: stream_wake_task_executed.clone(),
                stream_wake_task_should_shutdown,
                stream_wake_task_error: stream_wake_task_error.clone(),
            },
            || {
                polls.set(polls.get() + 1);
                Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "terminal is gone",
                ))
            },
        );

        assert!(!stream_wake_task_executed.load(Ordering::SeqCst));
        assert_eq!(polls.get(), 1);
        assert_eq!(wake_counter.0.load(Ordering::SeqCst), 1);

        let error = stream_wake_task_error
            .lock()
            .unwrap()
            .take()
            .expect("the poll error should be available to poll_next");
        assert_eq!(error.kind(), io::ErrorKind::UnexpectedEof);
        assert_eq!(error.to_string(), "terminal is gone");
    }
}
