use std::{
    sync::{RwLock, RwLockWriteGuard},
    time::Duration,
};

use lazy_static::lazy_static;

use crate::{input::Event, Result};

use super::{
    events::InternalEvent,
    poll::EventPoll,
    reader::{EventReader, InternalEventReader},
};

lazy_static! {
    /// Static instance of `EventPool`.
    /// This needs to be static because there can be one event reader.
    static ref EVENT_POOL: RwLock<EventPool> = { RwLock::new(EventPool::new()) };
}

/// Wrapper for event readers.
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

    /// Enqueues an `InternalEvent` into the internal event reader.
    pub(crate) fn enqueue_internal(&mut self, event: InternalEvent) {
        self.internal_event_reader.enqueue(event);
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
