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

/// Terminal color type, used in queries and responses.
///
/// `Palette(n)` uses OSC 4. The remaining variants use OSC 10..=19.
#[derive(Debug, PartialOrd, PartialEq, Hash, Clone, Copy, Eq)]
pub enum ColorType {
    Palette(u8),
    Foreground,
    Background,
    Cursor,
    PointerForeground,
    PointerBackground,
    TektronixForeground,
    TektronixBackground,
    HighlightBackground,
    TektronixCursor,
    HighlightForeground,
}

impl ColorType {
    /// Maps an OSC number (10..=19) to the corresponding `ColorType` variant.
    pub(crate) fn from_osc_number(n: u8) -> Option<Self> {
        match n {
            10 => Some(Self::Foreground),
            11 => Some(Self::Background),
            12 => Some(Self::Cursor),
            13 => Some(Self::PointerForeground),
            14 => Some(Self::PointerBackground),
            15 => Some(Self::TektronixForeground),
            16 => Some(Self::TektronixBackground),
            17 => Some(Self::HighlightBackground),
            18 => Some(Self::TektronixCursor),
            19 => Some(Self::HighlightForeground),
            _ => None,
        }
    }

    /// Returns the OSC number for this color type.
    pub(crate) fn osc_number(&self) -> u8 {
        match self {
            Self::Palette(_) => 4,
            Self::Foreground => 10,
            Self::Background => 11,
            Self::Cursor => 12,
            Self::PointerForeground => 13,
            Self::PointerBackground => 14,
            Self::TektronixForeground => 15,
            Self::TektronixBackground => 16,
            Self::HighlightBackground => 17,
            Self::TektronixCursor => 18,
            Self::HighlightForeground => 19,
        }
    }
}

/// The terminal's color scheme preference (dark or light).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialOrd, Ord, PartialEq, Hash, Clone, Copy, Eq)]
pub enum ColorScheme {
    Dark,
    Light,
}

/// A parsed color response from the terminal.
#[derive(Debug, PartialOrd, PartialEq, Hash, Clone, Eq)]
pub(crate) struct ColorEntry {
    pub color_type: ColorType,
    pub r: u8,
    pub g: u8,
    pub b: u8,
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
    /// A terminal color response (OSC 4 or OSC 10..=19).
    #[cfg(unix)]
    ColorResponse(ColorEntry),
    /// A color scheme response (CSI ? 997 ; 1/2 n).
    #[cfg(unix)]
    ColorSchemeResponse(ColorScheme),
}
