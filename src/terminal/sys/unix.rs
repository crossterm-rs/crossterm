use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};

use crate::utils::Result;

pub(crate) fn exit() {
    ::std::process::exit(0);
}

pub(crate) fn get_terminal_size() -> Result<(u16, u16)> {
    // http://rosettacode.org/wiki/Terminal_control/Dimensions#Library:_BSD_libc
    let mut size = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    let r = unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ.into(), &mut size) };

    if r == 0 {
        Ok((size.ws_col, size.ws_row))
    } else {
        Err(std::io::Error::last_os_error().into())
    }
}
