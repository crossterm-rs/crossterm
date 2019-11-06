//! UNIX related logic for terminal manipulation.
use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};
use std::process;

use crate::utils::sys::unix::wrap_with_result;
use crate::utils::Result;

/// Exits the current application.
pub fn exit() {
    ::std::process::exit(0);
}

/// execute tput with the given argument and parse
/// the output as a u16.
///
/// The arg should be "cols" or "lines"
fn tput_value(arg: &str) -> Option<u16> {
    match process::Command::new("tput").arg(arg).output() {
        Ok(process::Output { stdout, .. }) => {
            let value = stdout
                .iter()
                .map(|&b| b as u16)
                .take_while(|&b| b >= 48 && b <= 58)
                .fold(0, |v, b| v * 10 + (b - 48));
            if value > 0 {
                Some(value)
            } else {
                None
            }
        }
        _ => None,
    }
}

/// return the size of the screen as determined by tput
///
/// This alternate way of computing the size is useful
///  when in a subshell.
fn tput_size() -> Option<(u16, u16)> {
    match (tput_value("cols"), tput_value("lines")) {
        (Some(w), Some(h)) => Some((w, h)),
        _ => None,
    }
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

    if let Ok(true) = wrap_with_result(unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ.into(), &mut size) })
    {
        return Ok((size.ws_col, size.ws_row));
    } else {
        tput_size().ok_or_else(|| std::io::Error::last_os_error().into())
    }
}
