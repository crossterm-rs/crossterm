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

use super::{poll, read, Event, INTERNAL_EVENT_READER};

/// Stream that reads events asynchronously.
///
/// When calling `next`, it will try to poll for event readiness and return the ready event.
/// If no event is ready to be read, a thread is spawned that waits for event readiness.
/// Then, if there is an event is ready, it will `wake` the associated task of the `Waker`.
/// This spawned thread will always be closed when the stream drops.
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
        let result = match poll(Some(Duration::from_secs(0))) {
            Ok(true) => Poll::Ready(Some(read())),
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
                            if let Ok(true) = poll(None) {
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
