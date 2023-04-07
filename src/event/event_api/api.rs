use std::{ time::Duration};
use parking_lot::{MappedMutexGuard, Mutex, MutexGuard};
use crate::event::{
    filter::{EventFilter, Filter},
    read::InternalEventReader,
    timeout::PollTimeout,
    InternalEvent
};

use super::Event;

/// Static instance of `InternalEventReader`.
/// This needs to be static because there can be one event reader.
static INTERNAL_EVENT_READER: Mutex<Option<InternalEventReader>> = parking_lot::const_mutex(None);

pub (super) fn lock_internal_event_reader() -> MappedMutexGuard<'static, InternalEventReader> {
    MutexGuard::map(INTERNAL_EVENT_READER.lock(), |reader| {
        reader.get_or_insert_with(InternalEventReader::default)
    })
}
fn try_lock_internal_event_reader_for(
    duration: Duration,
) -> Option<MappedMutexGuard<'static, InternalEventReader>> {
    Some(MutexGuard::map(
        INTERNAL_EVENT_READER.try_lock_for(duration)?,
        |reader| reader.get_or_insert_with(InternalEventReader::default),
    ))
}

/// Checks if there is an [`Event`](enum.Event.html) available.
///
/// Returns `Ok(true)` if an [`Event`](enum.Event.html) is available otherwise it returns `Ok(false)`.
///
/// `Ok(true)` guarantees that subsequent call to the [`read`](fn.read.html) function
/// won't block.
///
/// # Arguments
///
/// * `timeout` - maximum waiting time for event availability
///
/// # Examples
///
/// Return immediately:
///
/// ```no_run
/// use std::time::Duration;
///
/// use crossterm::{event::poll, Result};
///
/// fn is_event_available() -> Result<bool> {
///     // Zero duration says that the `poll` function must return immediately
///     // with an `Event` availability information
///     poll(Duration::from_secs(0))
/// }
/// ```
///
/// Wait up to 100ms:
///
/// ```no_run
/// use std::time::Duration;
///
/// use crossterm::{event::poll, Result};
///
/// fn is_event_available() -> Result<bool> {
///     // Wait for an `Event` availability for 100ms. It returns immediately
///     // if an `Event` is/becomes available.
///     poll(Duration::from_millis(100))
/// }
/// ```
pub fn poll(timeout: Duration) -> crate::Result<bool> {
    poll_internal(Some(timeout), &EventFilter)
}

/// Reads a single [`Event`](enum.Event.html).
///
/// This function blocks until an [`Event`](enum.Event.html) is available. Combine it with the
/// [`poll`](fn.poll.html) function to get non-blocking reads.
///
/// # Examples
///
/// Blocking read:
///
/// ```no_run
/// use crossterm::{event::read, Result};
///
/// fn print_events() -> Result<bool> {
///     loop {
///         // Blocks until an `Event` is available
///         println!("{:?}", read()?);
///     }
/// }
/// ```
///
/// Non-blocking read:
///
/// ```no_run
/// use std::time::Duration;
///
/// use crossterm::{event::{read, poll}, Result};
///
/// fn print_events() -> Result<bool> {
///     loop {
///         if poll(Duration::from_millis(100))? {
///             // It's guaranteed that `read` won't block, because `poll` returned
///             // `Ok(true)`.
///             println!("{:?}", read()?);
///         } else {
///             // Timeout expired, no `Event` is available
///         }
///     }
/// }
/// ```
pub fn read() -> crate::Result<Event> {
    match read_internal(&EventFilter)? {
        InternalEvent::Event(event) => Ok(event),
        #[cfg(unix)]
        _ => unreachable!(),
    }
}

/// Polls to check if there are any `InternalEvent`s that can be read within the given duration.
pub(crate) fn poll_internal<F>(timeout: Option<Duration>, filter: &F) -> crate::Result<bool>
where
    F: Filter,
{
    let (mut reader, timeout) = if let Some(timeout) = timeout {
        let poll_timeout = PollTimeout::new(Some(timeout));
        if let Some(reader) = try_lock_internal_event_reader_for(timeout) {
            (reader, poll_timeout.leftover())
        } else {
            return Ok(false);
        }
    } else {
        (lock_internal_event_reader(), None)
    };
    reader.poll(timeout, filter)
}

/// Reads a single `InternalEvent`.
pub(crate) fn read_internal<F>(filter: &F) -> crate::Result<InternalEvent>
where
    F: Filter,
{
    let mut reader = lock_internal_event_reader();
    reader.read(filter)
}
