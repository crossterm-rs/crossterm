use std::{
    io,
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc::{self, SyncSender},
    },
    task::{Context, Poll, Waker as TaskWaker},
    thread,
    time::Duration,
};

use futures_core::stream::Stream;
use parking_lot::Mutex;

use crate::event::{
    Event,
    filter::EventFilter,
    internal::{self, InternalEvent},
    sys::Waker,
};

/// A stream of `Result<Event>`.
///
/// **This type is not available by default. You have to use the `event-stream` feature flag
/// to make it available.**
///
/// It implements the [Stream](futures_core::stream::Stream)
/// trait and allows you to receive [`Event`]s with [`smol`](https://crates.io/crates/smol)
/// or [`tokio`](https://crates.io/crates/tokio) crates.
///
/// Check the [examples](https://github.com/crossterm-rs/crossterm/tree/master/examples) folder to see how to use
/// it (`event-stream-*`).
#[derive(Debug)]
pub struct EventStream {
    poll_internal_waker: Waker,
    pending_wake: Arc<PendingWake>,
    stream_wake_task_should_shutdown: Arc<AtomicBool>,
    task_sender: SyncSender<()>,
}

impl Default for EventStream {
    fn default() -> Self {
        let (task_sender, receiver) = mpsc::sync_channel(1);
        let pending_wake = Arc::new(PendingWake::default());
        let stream_wake_task_should_shutdown = Arc::new(AtomicBool::new(false));
        let worker_wake = pending_wake.clone();
        let worker_should_shutdown = stream_wake_task_should_shutdown.clone();

        thread::spawn(move || {
            while let Ok(()) = receiver.recv() {
                loop {
                    if let Ok(true) = internal::poll(None, &EventFilter) {
                        break;
                    }

                    if worker_should_shutdown.load(Ordering::SeqCst) {
                        break;
                    }
                }
                worker_wake.wake();
            }
        });

        EventStream {
            poll_internal_waker: internal::lock_event_reader().waker(),
            pending_wake,
            stream_wake_task_should_shutdown,
            task_sender,
        }
    }
}

impl EventStream {
    /// Constructs a new instance of `EventStream`.
    pub fn new() -> EventStream {
        EventStream::default()
    }
}

#[derive(Debug, Default)]
struct PendingWake(Mutex<Option<TaskWaker>>);

impl PendingWake {
    fn register(&self, waker: &TaskWaker) -> bool {
        let waker = waker.clone();
        let previous = self.0.lock().replace(waker);
        previous.is_none()
    }

    fn wake(&self) {
        let waker = self.0.lock().take();
        if let Some(waker) = waker {
            waker.wake();
        }
    }
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
        match internal::poll(Some(Duration::from_secs(0)), &EventFilter) {
            Ok(true) => match internal::read(&EventFilter) {
                Ok(InternalEvent::Event(event)) => Poll::Ready(Some(Ok(event))),
                Err(e) => Poll::Ready(Some(Err(e))),
                #[cfg(unix)]
                _ => unreachable!(),
            },
            Ok(false) => {
                if self.pending_wake.register(cx.waker()) {
                    let _ = self.task_sender.send(());
                }
                Poll::Pending
            }
            Err(e) => Poll::Ready(Some(Err(e))),
        }
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
    use std::task::Wake;

    use super::*;

    struct Notify(mpsc::Sender<&'static str>, &'static str);

    impl Wake for Notify {
        fn wake(self: Arc<Self>) {
            self.0.send(self.1).unwrap();
        }
    }

    #[test]
    fn pending_wake_uses_latest_waker() {
        let (tx, rx) = mpsc::channel();
        let first = TaskWaker::from(Arc::new(Notify(tx.clone(), "first")));
        let latest = TaskWaker::from(Arc::new(Notify(tx, "latest")));

        let pending_wake = PendingWake::default();
        assert!(pending_wake.register(&first));
        assert!(!pending_wake.register(&latest));
        pending_wake.wake();

        assert_eq!(rx.try_recv(), Ok("latest"));
        assert_eq!(rx.try_recv(), Err(mpsc::TryRecvError::Empty));
        assert!(pending_wake.register(&first));
    }
}
