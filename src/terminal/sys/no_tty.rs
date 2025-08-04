//! Non-terminal related logic for terminal manipulation.

use crate::event::internal_no_tty::NoTtyEvent;
#[cfg(feature = "events")]
use crate::event::KeyboardEnhancementFlags;
use crate::terminal::WindowSize;
use std::io;

pub(crate) fn is_raw_mode_enabled() -> bool {
    true
}

pub(crate) fn window_size(event: &NoTtyEvent) -> io::Result<WindowSize> {
    let win = event.window_size.lock();
    let size = WindowSize {
        rows: win.rows,
        columns: win.columns,
        width: win.width,
        height: win.height,
    };
    Ok(size)
}

#[allow(clippy::useless_conversion)]
pub(crate) fn size(event: &NoTtyEvent) -> io::Result<(u16, u16)> {
    match window_size(event) {
        Ok(window_size) => Ok((window_size.columns, window_size.rows)),
        Err(e) => Err(e),
    }
}

pub(crate) fn enable_raw_mode() -> io::Result<()> {
    Ok(())
}

/// Reset the raw mode.
///
/// More precisely, reset the whole termios mode to what it was before the first call
/// to [enable_raw_mode]. If you don't mess with termios outside of crossterm, it's
/// effectively disabling the raw mode and doing nothing else.
pub(crate) fn disable_raw_mode() -> io::Result<()> {
    Ok(())
}

/// Queries the terminal's support for progressive keyboard enhancement.
///
/// On unix systems, this function will block and possibly time out while
/// [`crossterm::event::read`](crate::event::read) or [`crossterm::event::poll`](crate::event::poll) are being called.
#[cfg(feature = "events")]
pub fn supports_keyboard_enhancement(event: &NoTtyEvent) -> io::Result<bool> {
    query_keyboard_enhancement_flags(event).map(|flags| flags.is_some())
}

/// Queries the terminal's currently active keyboard enhancement flags.
///
/// On unix systems, this function will block and possibly time out while
/// [`crossterm::event::read`](crate::event::read) or [`crossterm::event::poll`](crate::event::poll) are being called.
#[cfg(feature = "events")]
pub fn query_keyboard_enhancement_flags(
    event: &NoTtyEvent,
) -> io::Result<Option<KeyboardEnhancementFlags>> {
    if is_raw_mode_enabled() {
        query_keyboard_enhancement_flags_raw(event)
    } else {
        query_keyboard_enhancement_flags_nonraw(event)
    }
}

#[cfg(feature = "events")]
fn query_keyboard_enhancement_flags_nonraw(
    event: &NoTtyEvent,
) -> io::Result<Option<KeyboardEnhancementFlags>> {
    enable_raw_mode()?;
    let flags = query_keyboard_enhancement_flags_raw(event);
    disable_raw_mode()?;
    flags
}

#[cfg(feature = "events")]
fn query_keyboard_enhancement_flags_raw(
    event: &NoTtyEvent,
) -> io::Result<Option<KeyboardEnhancementFlags>> {
    use crate::event::{
        filter::{KeyboardEnhancementFlagsFilter, PrimaryDeviceAttributesFilter},
        internal::InternalEvent,
    };
    use std::time::Duration;

    // This is the recommended method for testing support for the keyboard enhancement protocol.
    // We send a query for the flags supported by the terminal and then the primary device attributes
    // query. If we receive the primary device attributes response but not the keyboard enhancement
    // flags, none of the flags are supported.
    //
    // See <https://sw.kovidgoyal.net/kitty/keyboard-protocol/#detection-of-support-for-this-protocol>

    // ESC [ ? u        Query progressive keyboard enhancement flags (kitty protocol).
    // ESC [ c          Query primary device attributes.
    const QUERY: &[u8] = b"\x1B[?u\x1B[c";

    event
        .send
        .send_timeout(QUERY.into(), Duration::from_secs(1))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    loop {
        match event.poll(
            Some(Duration::from_millis(2000)),
            &KeyboardEnhancementFlagsFilter,
        ) {
            Ok(true) => {
                match event.read(&KeyboardEnhancementFlagsFilter) {
                    Ok(InternalEvent::KeyboardEnhancementFlags(current_flags)) => {
                        // Flush the PrimaryDeviceAttributes out of the event queue.
                        event.read(&PrimaryDeviceAttributesFilter).ok();
                        return Ok(Some(current_flags));
                    }
                    _ => return Ok(None),
                }
            }
            Ok(false) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "The keyboard enhancement status could not be read within a normal duration",
                ));
            }
            Err(_) => {}
        }
    }
}
