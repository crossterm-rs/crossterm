use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use async_stream::try_stream;
use futures_core::stream::Stream;

use crate::Result;

use super::{
    filter::EventFilter, lock_internal_event_reader, poll_internal, read_internal, sys::Waker,
    Event, InternalEvent,
};
struct ReadEventFuture {
    poll_internal_waker: Waker,
}

impl Default for ReadEventFuture {
    fn default() -> Self {
        Self {
            poll_internal_waker: lock_internal_event_reader().waker(),
        }
    }
}

impl Future for ReadEventFuture {
    type Output = Result<Event>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let result = match poll_internal(Some(Duration::from_secs(0)), &EventFilter) {
            Ok(true) => match read_internal(&EventFilter) {
                Ok(InternalEvent::Event(event)) => Poll::Ready(Ok(event)),
                Err(e) => Poll::Ready(Err(e)),
                #[cfg(unix)]
                _ => unreachable!(),
            },
            Ok(false) => {
                cx.waker().clone().wake();
                Poll::Pending
            }
            Err(e) => Poll::Ready(Err(e)),
        };
        result
    }
}

impl Drop for ReadEventFuture {
    fn drop(&mut self) {
        let _ = self.poll_internal_waker.wake();
    }
}

/// Returns a stream of `Result<Event>`.
/// See [`EventStream`] for more details.
pub fn event_stream() -> impl Stream<Item = Result<Event>> {
    try_stream! {
        loop {
            let e = ReadEventFuture::default().await?;
            yield e;
        }
    }
}

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
pub struct EventStream(Pin<Box<dyn Stream<Item = Result<Event>>>>);

impl Default for EventStream {
    fn default() -> Self {
        Self(Box::pin(event_stream()))
    }
}

impl EventStream {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Stream for EventStream {
    type Item = Result<Event>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.0.as_mut().poll_next(cx)
    }
}
