//! Stub for targets that are neither unix nor windows (e.g. wasm): there is
//! no terminal to query, so the cursor position is unavailable.

use std::io;

pub fn position() -> io::Result<(u16, u16)> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "cursor position is not available on this platform",
    ))
}
