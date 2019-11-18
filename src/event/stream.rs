use futures::{
    task::{Context, Poll},
    Stream,
};
use std::{
    pin::Pin,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use crate::Result;

use super::{
    filter::EventFilter, poll_internal, read_internal, Event, InternalEvent, INTERNAL_EVENT_READER,
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
pub struct EventStream {
    wake_thread_spawned: Arc<AtomicBool>,
    wake_thread_should_shutdown: Arc<AtomicBool>,
}

impl EventStream {
    pub fn new() -> EventStream {
        EventStream {
            wake_thread_spawned: Arc::new(AtomicBool::new(false)),
            wake_thread_should_shutdown: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl Stream for EventStream {
    type Item = Result<Event>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let result = match poll_internal(Some(Duration::from_secs(0)), &EventFilter) {
            Ok(true) => match read_internal(&EventFilter) {
                Ok(InternalEvent::Event(event)) => Poll::Ready(Some(Ok(event))),
                Err(e) => Poll::Ready(Some(Err(e))),
                _ => unreachable!(),
            },
            Ok(false) => {
                if !self
                    .wake_thread_spawned
                    .compare_and_swap(false, true, Ordering::SeqCst)
                {
                    let waker = cx.waker().clone();
                    let wake_thread_spawned = self.wake_thread_spawned.clone();
                    let wake_thread_should_shutdown = self.wake_thread_should_shutdown.clone();

                    wake_thread_should_shutdown.store(false, Ordering::SeqCst);

                    thread::spawn(move || {
                        loop {
                            if let Ok(true) = poll_internal(None, &EventFilter) {
                                break;
                            }

                            if wake_thread_should_shutdown.load(Ordering::SeqCst) {
                                break;
                            }
                        }
                        wake_thread_spawned.store(false, Ordering::SeqCst);
                        waker.wake();
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
        self.wake_thread_should_shutdown
            .store(true, Ordering::SeqCst);
        INTERNAL_EVENT_READER.read().wake();
    }
}
