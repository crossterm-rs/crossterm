use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::time::Duration;

use lazy_static::lazy_static;

#[cfg(unix)]
use crate::input::event_source::tty::TTYEventSource;
#[cfg(windows)]
use crate::input::event_source::winapi::WinApiEventSource;
use crate::input::events::InternalEvent;
use crate::{Event, EventSource, Result};

lazy_static! {
    /// Static instance of `EventPool`.
    /// This needs to be static because there can be one event reader.
    pub static ref EVENT_POOL: RwLock<EventPool> = { RwLock::new(EventPool::new()) };
}

/// Return true if there are events to be read.
///
/// This function will block the current thread until the given timeout is elapsed.
/// It will wait indefinitely if no timeout is given.
pub fn poll(timeout: Option<Duration>) -> Result<bool> {
    let mut lock = EventPool::get_mut();
    lock.pool().poll(timeout)
}

/// Reads a single event.
///
/// This function will block until an event is received.
pub fn read() -> Result<Event> {
    let mut lock = EventPool::get_mut();
    lock.pool().read()
}

/// Produces events to consumers.
///
/// There should one and only one instance of this type.
///
/// The `EventPool` is responsible for:
/// - creating `EventStreams`
/// - passing events listening `EventStreams`
/// - manage the producer
///
/// You can get an instance to this pool by acuring either the read-only or write lock for it:
///
/// ```no_run
/// use crossterm::EventPool;
///
/// let read_only = EventPool::get().pool();
/// let read_only = EventPool::get_mut().pool();
/// ```
///
/// Not that one can obtain only one writer and multiple readers.
pub struct EventPool {
    event_source: Box<dyn EventSource>,
}

impl EventPool {
    /// Construct an new instance of `EventPool`.
    pub(crate) fn new() -> EventPool {
        #[cfg(windows)]
        let input = WinApiEventSource::new();
        #[cfg(unix)]
        let input = TTYEventSource::new();

        EventPool {
            event_source: Box::new(input) as Box<dyn EventSource + Sync + Send>,
        }
    }

    /// Acquires an write lock to `EventPool`.
    pub fn get_mut<'a>() -> EventPoolWriteLock<'a> {
        EventPoolWriteLock::from_lock_result(EVENT_POOL.write().unwrap_or_else(|e| e.into_inner()))
    }

    /// Acquires an read-only lock to `EventPool`.
    pub fn get<'a>() -> EventPoolReadLock<'a> {
        EventPoolReadLock::from_lock_result(EVENT_POOL.read().unwrap_or_else(|e| e.into_inner()))
    }

    /// Changes the default `EventSource` to the given `EventSource`.
    pub fn set_event_source(&mut self, event_source: Box<dyn EventSource>) {
        self.event_source = event_source;
    }

    /// Polls for event readiness
    pub fn poll(&mut self, timeout: Option<Duration>) -> Result<bool> {
        self.event_source.poll(timeout)
    }

    /// Reads for an event from the underlying event source
    pub fn read(&mut self) -> Result<Event> {
        match self.event_source.read()? {
            Some(InternalEvent::Input(event)) => {
                return Ok(event);
            }
            Some(InternalEvent::CursorPosition(x, y)) => return Ok(Event::CursorPosition(x, y)),
            None => return self.read(),
        }
    }

    /// Enables mouse events to be monitored.
    pub fn enable_mouse_events() {}

    /// Disables mouse events to be monitored.
    pub fn disable_mouse_events() {}
}

/// An acquired read lock to the event channel pool.
pub struct EventPoolReadLock<'a> {
    read_guard: RwLockReadGuard<'a, EventPool>,
}

impl<'a> EventPoolReadLock<'a> {
    pub(crate) fn from_lock_result(
        read_guard: RwLockReadGuard<'a, EventPool>,
    ) -> EventPoolReadLock<'a> {
        EventPoolReadLock { read_guard }
    }

    /// Returns the obtained read lock to the pool.
    pub fn pool(&self) -> &RwLockReadGuard<'a, EventPool> {
        &self.read_guard
    }
}

/// An acquired write lock to the event pool.
pub struct EventPoolWriteLock<'a> {
    write_guard: RwLockWriteGuard<'a, EventPool>,
}

impl<'a> EventPoolWriteLock<'a> {
    pub(crate) fn from_lock_result(
        write_guard: RwLockWriteGuard<'a, EventPool>,
    ) -> EventPoolWriteLock<'a> {
        EventPoolWriteLock { write_guard }
    }

    /// Returns the obtained write lock to the pool.
    pub fn pool(&mut self) -> &mut RwLockWriteGuard<'a, EventPool> {
        &mut self.write_guard
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc::channel;

    use crate::input::event_source::fake::FakeEventSource;
    use crate::Event;
    use crate::{event_stream, poll, EventPool};

    #[test]
    pub fn test_read_input_multiple_consumers() {
        let mut lock = EventPool::get_mut();
        let mut pool = lock.pool();

        // sender can be used to send fake data, receiver is used to provide the fake input source with input events.
        let (input_sender, input_receiver) = channel();

        // set input source, and sent fake input
        pool.set_event_source(Box::new(FakeEventSource::new(input_receiver)));
        input_sender.send(Event::Unknown).unwrap();

        // drop write lock
        drop(lock);

        // acquire consumers
        let mut stream1 = event_stream();
        let mut stream2 = event_stream();

        // poll for input
        poll().unwrap();

        assert_eq!(stream1.events().next(), Some(Event::Unknown));
        assert_eq!(stream2.events().next(), Some(Event::Unknown));
    }
}
