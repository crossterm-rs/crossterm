//! UNIX related logic for terminal manipulation.

use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};

use crate::utils::sys::unix::wrap_with_result;
use crate::utils::Result;

/// Exits the current application.
pub fn exit() {
    ::std::process::exit(0);
}

/// Returns the terminal size `(columns, rows)`.
///
/// The top left cell is represented `1,1`.
pub fn size() -> Result<(u16, u16)> {
    // http://rosettacode.org/wiki/Terminal_control/Dimensions#Library:_BSD_libc
    let mut size = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };

    if let Ok(()) =
        wrap_with_result(unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ.into(), &mut size) }).unwrap_or()
    {
        return Ok((size.ws_col, size.ws_row));
    } else {
        Err(std::io::Error::last_os_error().into())
    }
}
