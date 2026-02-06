//! This is a WINDOWS specific implementation for input related action.

use std::convert::TryFrom;
use std::io;
use std::sync::atomic::{AtomicU64, Ordering};

use crossterm_winapi::{ConsoleMode, Handle};

pub(crate) mod parse;
pub(crate) mod poll;
#[cfg(feature = "event-stream")]
pub(crate) mod waker;

const ENABLE_MOUSE_MODE: u32 = 0x0010 | 0x0080 | 0x0008;

// See https://learn.microsoft.com/en-us/windows/console/setconsolemode
const ENABLE_VIRTUAL_TERMINAL_INPUT: u32 = 0x0200;

/// This is a either `u64::MAX` if it's uninitialized or a valid `u32` that stores the original
/// console mode if it's initialized.
static ORIGINAL_CONSOLE_MODE: AtomicU64 = AtomicU64::new(u64::MAX);

/// Saves the original console mode on first call (uses compare_exchange, so only the
/// first caller wins). Callers that modify the mode (try_enable_vt_input,
/// enable_mouse_capture) must call this **before** modifying the mode to ensure the
/// stored value is the true original. Currently, `try_enable_vt_input` is called
/// first in `WindowsEventSource::new()`, which is correct.
fn init_original_console_mode(original_mode: u32) {
    let _ = ORIGINAL_CONSOLE_MODE.compare_exchange(
        u64::MAX,
        u64::from(original_mode),
        Ordering::Relaxed,
        Ordering::Relaxed,
    );
}

/// Returns the original console color, make sure to call `init_console_color` before calling this function. Otherwise this function will panic.
fn original_console_mode() -> std::io::Result<u32> {
    u32::try_from(ORIGINAL_CONSOLE_MODE.load(Ordering::Relaxed))
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Initial console modes not set"))
}

/// Try to enable virtual terminal input on the console input handle.
/// Returns `Ok(true)` if VT input was successfully enabled, `Ok(false)` if unsupported.
pub(crate) fn try_enable_vt_input() -> io::Result<bool> {
    let mode = ConsoleMode::from(Handle::current_in_handle()?);
    let current = mode.mode()?;
    init_original_console_mode(current);

    // Try to set the VT input flag. If the console doesn't support it
    // (e.g. legacy conhost), set_mode will fail.
    match mode.set_mode(current | ENABLE_VIRTUAL_TERMINAL_INPUT) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
}

pub(crate) fn enable_mouse_capture() -> std::io::Result<()> {
    let mode = ConsoleMode::from(Handle::current_in_handle()?);
    let current = mode.mode()?;
    init_original_console_mode(current);
    // OR the flags to preserve existing mode bits (e.g. VT input)
    mode.set_mode(current | ENABLE_MOUSE_MODE)?;

    Ok(())
}

pub(crate) fn disable_mouse_capture() -> std::io::Result<()> {
    let mode = ConsoleMode::from(Handle::current_in_handle()?);
    mode.set_mode(original_console_mode()?)?;
    Ok(())
}
