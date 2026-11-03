//! Stubs for targets that are neither unix nor windows (e.g. wasm): the crate
//! compiles, and terminal control reports `Unsupported` at runtime.

use std::io;

use crate::terminal::WindowSize;

pub(crate) fn is_raw_mode_enabled() -> io::Result<bool> {
    Err(unsupported())
}

pub(crate) fn enable_raw_mode() -> io::Result<()> {
    Err(unsupported())
}

pub(crate) fn disable_raw_mode() -> io::Result<()> {
    Err(unsupported())
}

pub(crate) fn size() -> io::Result<(u16, u16)> {
    Err(unsupported())
}

pub(crate) fn window_size() -> io::Result<WindowSize> {
    Err(unsupported())
}

/// There is no terminal to probe on these targets.
#[cfg(feature = "events")]
pub fn supports_keyboard_enhancement() -> io::Result<bool> {
    Ok(false)
}

fn unsupported() -> io::Error {
    io::Error::new(
        io::ErrorKind::Unsupported,
        "terminal control is not supported on this platform",
    )
}
