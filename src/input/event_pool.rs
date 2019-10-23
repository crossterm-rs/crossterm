use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use lazy_static::lazy_static;

#[cfg(unix)]
use crate::input::event_source::tty::TTYEventSource;
#[cfg(windows)]
use crate::input::event_source::winapi::WinApiEventSource;
use crate::input::spmc::EventChannel;
use crate::EventStream;
use crate::{EventSource, Result};

lazy_static! {
    /// Static instance of `EventPool`.
    /// This needs to be static because there can be one event reader.
    pub static ref EVENT_POOL: RwLock<EventPool> = { RwLock::new(EventPool::new()) };
}

/// Returns a `EventStream` that can be used to read input events with.
///
/// Note that in order for the stream to receive events you have to call [`poll_event`](..link) first.
///
/// ```no_run
/// use crossterm::{event_stream, poll_event, RawScreen};
///
/// fn main () {
///     let r = RawScreen::into_raw_mode().unwrap();
///
///     let mut stream = event_stream();
///
///     while true {
///         poll_event();
///
///         for event in stream.key_events() {
///             println!("{:?}", event);
///         }
///     }
/// }
/// ```
pub fn event_stream() -> EventStream {
    let lock = EventPool::get();
    lock.pool().event_stream()
}

/// Polls for occurred events.
///
/// An input event will be replicated to all `EventStreams` when an event has occurred.
/// This function will wait until an event read until an event is risen.
pub fn poll_event() -> Result<()> {
    let mut lock = EventPool::get_mut();
    lock.pool().poll()
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
/// let read_only = EventPool::get_mut().pool;
/// ```
///
/// Not that one can obtain only one writer and multiple readers.
pub struct EventPool {
    pub(crate) event_channel: EventChannel,
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
            event_channel: EventChannel::channel(shrev::EventChannel::new()),
        }
    }

    /// Returns a [`EventStream`](struct.EventStream.html) that will consume events produced by the producer.
    pub(crate) fn event_stream(&self) -> EventStream {
        EventStream::new(self.event_channel.new_consumer())
    }

    /// Acquires an write lock to `EventPool`.
    pub fn get_mut<'a>() -> EventPoolWriteLock<'a> {
        EventPoolWriteLock::from_lock_result(EVENT_POOL.write().unwrap_or_else(|e| e.into_inner()))
    }

    /// Acquires an read-only lock to `EventPool`.
    pub fn get<'a>() -> EventPoolReadLock<'a> {
        EventPoolReadLock::from_lock_result(EVENT_POOL.read().unwrap_or_else(|e| e.into_inner()))
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
    use crate::InputEvent;
    use crate::{event_stream, poll_event, EventPool};

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
