use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use lazy_static::lazy_static;

use crate::{EventSource, Result};
use crate::EventStream;
#[cfg(unix)]
use crate::input::event_source::tty::TTYEventSource;
#[cfg(windows)]
use crate::input::event_source::winapi::WinApiEventSource;
use crate::input::spmc::EventChannel;

lazy_static! {
    /// Static event pool that can be used to read input events.
    pub static ref INPUT: RwLock<EventPool> = { RwLock::new(EventPool::new()) };
}

/// Returns a event stream that can be used to read input events with.
pub fn event_stream() -> EventStream {
    let lock = EventPool::get();
    lock.pool().event_stream()
}

/// Polls for input from the underlying input source.
///
/// An input event will be replicated to all consumers aka streams if an input event has occurred.
/// This poll function will block read for a single key press.
pub fn poll_event() -> Result<()> {
    let mut lock = EventPool::get_mut();
    lock.pool().poll()
}

/// An event pool is a pool that takes care of polling for new input.
/// Before you are able to use the input pool, you have to acquire a lock for it.
/// That prevents race conditions while reading input from certain sources.
pub struct EventPool {
    pub(crate) event_channel: EventChannel,
    event_source: Box<dyn EventSource>,
}

impl EventPool {
    pub(crate) fn new() -> EventPool {
        #[cfg(windows)]
        let input = WinApiEventSource::new();
        #[cfg(unix)]
        let input = TTYEventSource::new();

        EventPool {
            event_source: Box::new(input) as Box<dyn EventSource + Sync + Send>,
            event_channel: EventChannel::channel(shrev::EventChannel::new()),
        }
    }

    /// Returns a event stream that can be used to read input events with.
    pub fn event_stream(&self) -> EventStream {
        EventStream::new(self.event_channel.new_consumer())
    }

    /// Acquires the `InputPool`, this can be used when you want mutable access to this pool.
    pub fn get_mut<'a>() -> EventPoolWriteLock<'a> {
        EventPoolWriteLock::from_lock_result(INPUT.write().unwrap_or_else(|e| e.into_inner()))
    }

    /// Acquires the `InputPool`, this can be used when you want mutable access to this pool.
    pub fn get<'a>() -> EventPoolReadLock<'a> {
        EventPoolReadLock::from_lock_result(INPUT.read().unwrap_or_else(|e| e.into_inner()))
    }

    /// Changes the default input source to the given input source.
    pub fn set_event_source(&mut self, event_source: Box<dyn EventSource>) {
        self.event_source = event_source;
    }

    /// Polls for input from the underlying input source.
    ///
    /// An input event will be replicated to all consumers aka streams if an input event has occurred.
    /// This poll function will block read for a single key press.
    pub fn poll(&mut self) -> Result<()> {
        // poll for occurred input events
        if let Some(event) = self.event_source.read_event()? {
            // produce the input event for the consumers
            self.event_channel.producer().produce_input_event(event);
        }

        Ok(())
    }

    pub fn enable_mouse_events() {}

    pub fn disable_mouse_events() {}
}

/// An acquired write lock to the event channel producer.
pub struct EventPoolReadLock<'a> {
    read_guard: RwLockReadGuard<'a, EventPool>,
}

impl<'a> EventPoolReadLock<'a> {
    pub(crate) fn from_lock_result(
        read_guard: RwLockReadGuard<'a, EventPool>,
    ) -> EventPoolReadLock<'a> {
        EventPoolReadLock { read_guard }
    }

    pub fn pool(&self) -> &RwLockReadGuard<'a, EventPool> {
        &self.read_guard
    }
}

/// An acquired write lock to the event channel producer.
pub struct EventPoolWriteLock<'a> {
    write_guard: RwLockWriteGuard<'a, EventPool>,
}

impl<'a> EventPoolWriteLock<'a> {
    pub(crate) fn from_lock_result(
        write_guard: RwLockWriteGuard<'a, EventPool>,
    ) -> EventPoolWriteLock<'a> {
        EventPoolWriteLock { write_guard }
    }

    pub fn pool(&mut self) -> &mut RwLockWriteGuard<'a, EventPool> {
        &mut self.write_guard
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc::channel;

    use crate::{event_stream, EventPool, poll_event};
    use crate::input::event_source::fake::FakeEventSource;
    use crate::InputEvent;

    #[test]
    pub fn test_read_input_multiple_consumers() {
        let mut lock = EventPool::get_mut();
        let mut pool = lock.pool();

        // sender can be used to send fake data, receiver is used to provide the fake input source with input events.
        let (input_sender, input_receiver) = channel();

        // set input source, and sent fake input
        pool.set_event_source(Box::new(FakeEventSource::new(input_receiver)));
        input_sender.send(InputEvent::Unknown).unwrap();

        // drop write lock
        drop(lock);

        // acquire consumers
        let mut stream1 = event_stream();
        let mut stream2 = event_stream();

        // poll for input
        poll_event().unwrap();

        assert_eq!(stream1.events().next(), Some(InputEvent::Unknown));
        assert_eq!(stream2.events().next(), Some(InputEvent::Unknown));
    }
}
