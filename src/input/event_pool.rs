use std::{
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
    time::Duration,
};

use lazy_static::lazy_static;

use crate::{
    input::{
        event_poll::EventPoll,
        event_reader::{EventReader, InternalEventReader},
        event_source::EventSource,
        events::InternalEvent,
        Event,
    },
    Result,
};

lazy_static! {
    /// Static instance of `EventPool`.
    /// This needs to be static because there can be one event reader.
    pub (crate) static ref EVENT_POOL: RwLock<EventPool> = { RwLock::new(EventPool::new()) };
}

/// Polls during an given duration for ready events.
///
/// This function takes in an optional duration.
/// * `None`: will block indefinitely until an event is read.
/// * `Some(duration)`: will block for the given duration.
///
/// The following value can be returned:
/// * `Ok(true)`: in case an event is ready.
/// * `Ok(false)`: in case the given duration is elapsed.
/// * `Err(err)`: in case of an error.
///
/// An ready event can be read with [read](LINK)
/// ```no_run
/// use std::time::Duration;
/// use crossterm::{Result, input::poll};
///
/// fn main() -> Result<()> {
///     // poll maximal 1 second
///     if poll(Some(Duration::from_millis(1000)))? {  /* logic */  }
///
///     // poll indefinitely
///     if poll(None)? { /* logic */  }
///
///     Ok(())
/// }
/// ```
pub fn poll(timeout: Option<Duration>) -> Result<bool> {
    let mut lock = EventPool::get_mut();
    lock.pool().poll(timeout)
}

/// Reads a single event.
///
/// This function will block until an event is received.
/// Use [poll](LINK) to check for ready events.
///
/// ```no_run
/// use crossterm::{Result, input::{read, poll, Event}};
/// use std::time::Duration;
///
/// fn main() -> Result<()> {
///     // poll maximal 1 second for an ready event.
///     if poll(Some(Duration::from_millis(1000)))? {
///         // read the ready event.
///         match read() {
///             Ok(Event(event)) => { println!("{:?}", event) }
///             _ => { }
///         }
///      }
///     Ok(())
/// }
/// ```
pub fn read() -> Result<Event> {
    let mut lock = EventPool::get_mut();
    lock.pool().read()
}

/// Changes the default `EventSource` to the given `EventSource`.
///
/// This might be usefull for testing.
/// See [FakeEventSource](LINK) for more information.
pub fn swap_event_source(new: Box<dyn EventSource>) {
    let mut lock = EventPool::get_mut();
    lock.pool().swap_event_source(new);
}

/// Wrapper for event readers which exposes an reading API for those.
///
/// There should be one and only one instance of this type,
/// because We can only have on source that is polling from the system for events.
pub(crate) struct EventPool {
    event_reader: EventReader,
    internal_event_reader: InternalEventReader,
}

impl EventPool {
    /// Construct an new instance of `EventPool`.
    pub(crate) fn new() -> EventPool {
        EventPool {
            event_reader: EventReader::new(),
            internal_event_reader: InternalEventReader::new(),
        }
    }

    /// Acquires an write lock to `EventPool`.
    pub(crate) fn get_mut<'a>() -> EventPoolWriteLock<'a> {
        EventPoolWriteLock::from_lock_result(EVENT_POOL.write().unwrap_or_else(|e| e.into_inner()))
    }

    /// Changes the default `EventSource` to the given `EventSource`.
    pub(crate) fn swap_event_source(&mut self, new: Box<dyn EventSource>) {
        self.internal_event_reader.swap_event_source(new)
    }

    /// Polls to check if there are any `Event`s that can be read withing the given duration.
    pub(crate) fn poll(&mut self, timeout: Option<Duration>) -> Result<bool> {
        self.event_reader.poll(timeout)
    }

    /// Reads a single `Event`.
    pub(crate) fn read(&mut self) -> Result<Event> {
        self.event_reader.read()
    }

    /// Polls to check if there are any `InternalEvent`s that can be read withing the given duration.
    pub(crate) fn poll_internal(&mut self, timeout: Option<Duration>) -> Result<bool> {
        self.internal_event_reader.poll(timeout)
    }

    /// Reads a single `InternalEvent`.
    pub(crate) fn read_internal(&mut self) -> Result<InternalEvent> {
        self.internal_event_reader.read()
    }
}

/// An acquired write lock to the event pool.
pub(crate) struct EventPoolWriteLock<'a> {
    write_guard: RwLockWriteGuard<'a, EventPool>,
}

impl<'a> EventPoolWriteLock<'a> {
    /// Constructs the write lock from the given `EventPool` write lock.
    pub(crate) fn from_lock_result(
        write_guard: RwLockWriteGuard<'a, EventPool>,
    ) -> EventPoolWriteLock<'a> {
        EventPoolWriteLock { write_guard }
    }

    /// Returns the obtained write lock to the pool.
    pub(crate) fn pool(&mut self) -> &mut RwLockWriteGuard<'a, EventPool> {
        &mut self.write_guard
    }
}

#[cfg(test)]
mod tests {}
