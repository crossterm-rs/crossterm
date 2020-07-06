use std::{
    pin::Pin,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use futures_util::{
    stream::Stream,
    task::{Context, Poll},
};

use crate::Result;

use super::{
    filter::EventFilter, poll_internal, read_internal, sys::Waker, Event, InternalEvent,
    INTERNAL_EVENT_READER,
};

/// A stream of `Result<Event>`.
///
/// **This type is not available by default. You have to use the `event-stream` feature flag
/// to make it available.**
///
/// It implements the [`futures::stream::Stream`](https://docs.rs/futures/0.3.1/futures/stream/trait.Stream.html)
/// trait and allows you to receive `Event`s with [`async-std`](https://crates.io/crates/async-std)
/// or [`tokio`](https://crates.io/crates/tokio) crates.
///
/// Check the [examples](https://github.com/crossterm-rs/crossterm/tree/master/examples) folder to see how to use
/// it (`event-stream-*`).
#[derive(Debug)]
pub struct EventStream {
    poll_internal_waker: Waker,
    stream_wake_thread_spawned: Arc<AtomicBool>,
    stream_wake_thread_should_shutdown: Arc<AtomicBool>,
}

impl Default for EventStream {
    fn default() -> Self {
        EventStream {
            poll_internal_waker: INTERNAL_EVENT_READER.write().waker(),
            stream_wake_thread_spawned: Arc::new(AtomicBool::new(false)),
            stream_wake_thread_should_shutdown: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl EventStream {
    /// Constructs a new instance of `EventStream`.
    pub fn new() -> EventStream {
        EventStream::default()
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
    type Item = Result<Event>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let result = match poll_internal(Some(Duration::from_secs(0)), &EventFilter) {
            Ok(true) => match read_internal(&EventFilter) {
                Ok(InternalEvent::Event(event)) => Poll::Ready(Some(Ok(event))),
                Err(e) => Poll::Ready(Some(Err(e))),
                #[cfg(unix)]
                _ => unreachable!(),
            },
            Ok(false) => {
                if !self
                    .stream_wake_thread_spawned
                    .compare_and_swap(false, true, Ordering::SeqCst)
                {
                    let stream_waker = cx.waker().clone();
                    let stream_wake_thread_spawned = self.stream_wake_thread_spawned.clone();
                    let stream_wake_thread_should_shutdown =
                        self.stream_wake_thread_should_shutdown.clone();

                    stream_wake_thread_should_shutdown.store(false, Ordering::SeqCst);

                    thread::spawn(move || {
                        loop {
                            if let Ok(true) = poll_internal(None, &EventFilter) {
                                break;
                            }

                            if stream_wake_thread_should_shutdown.load(Ordering::SeqCst) {
                                break;
                            }
                        }
                        stream_wake_thread_spawned.store(false, Ordering::SeqCst);
                        stream_waker.wake();
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
        self.stream_wake_thread_should_shutdown
            .store(true, Ordering::SeqCst);
        let _ = self.poll_internal_waker.wake();
    }
}
