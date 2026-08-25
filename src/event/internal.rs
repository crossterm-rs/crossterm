use std::time::Duration;

use parking_lot::{MappedMutexGuard, Mutex, MutexGuard};

#[cfg(unix)]
use crate::event::KeyboardEnhancementFlags;
use crate::event::{filter::Filter, read::InternalEventReader, timeout::PollTimeout, Event};

/// Static instance of `InternalEventReader`.
/// This needs to be static because there can be one event reader.
static EVENT_READER: Mutex<Option<InternalEventReader>> = parking_lot::const_mutex(None);

pub(crate) fn lock_event_reader() -> MappedMutexGuard<'static, InternalEventReader> {
    MutexGuard::map(EVENT_READER.lock(), |reader| {
        reader.get_or_insert_with(InternalEventReader::default)
    })
}

fn try_lock_event_reader_for(
    duration: Duration,
) -> Option<MappedMutexGuard<'static, InternalEventReader>> {
    Some(MutexGuard::map(
        EVENT_READER.try_lock_for(duration)?,
        |reader| reader.get_or_insert_with(InternalEventReader::default),
    ))
}

/// Polls to check if there are any `InternalEvent`s that can be read within the given duration.
pub(crate) fn poll<F>(timeout: Option<Duration>, filter: &F) -> std::io::Result<bool>
where
    F: Filter,
{
    let (mut reader, timeout) = if let Some(timeout) = timeout {
        let poll_timeout = PollTimeout::new(Some(timeout));
        if let Some(reader) = try_lock_event_reader_for(timeout) {
            (reader, poll_timeout.leftover())
        } else {
            return Ok(false);
        }
    } else {
        (lock_event_reader(), None)
    };
    reader.poll(timeout, filter)
}

/// Reads a single `InternalEvent`.
pub(crate) fn read<F>(filter: &F) -> std::io::Result<InternalEvent>
where
    F: Filter,
{
    let mut reader = lock_event_reader();
    reader.read(filter)
}

/// Reads a single `InternalEvent`. Non-blocking.
pub(crate) fn try_read<F>(filter: &F) -> Option<InternalEvent>
where
    F: Filter,
{
    let mut reader = lock_event_reader();
    reader.try_read(filter)
}

/// An internal event.
///
/// Encapsulates publicly available `Event` with additional internal
/// events that shouldn't be publicly available to the crate users.
#[derive(Debug, PartialOrd, PartialEq, Hash, Clone, Eq)]
pub(crate) enum InternalEvent {
    /// An event.
    Event(Event),
    /// A cursor position (`col`, `row`).
    #[cfg(unix)]
    CursorPosition(u16, u16),
    /// The progressive keyboard enhancement flags enabled by the terminal.
    #[cfg(unix)]
    KeyboardEnhancementFlags(KeyboardEnhancementFlags),
    /// Attributes and architectural class of the terminal.
    #[cfg(unix)]
    PrimaryDeviceAttributes,
}
