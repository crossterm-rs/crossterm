use std::{io, mem};

use super::cvt;
use super::libc;

#[repr(C)]
struct TermSize {
    row: libc::c_ushort,
    col: libc::c_ushort,
    _x: libc::c_ushort,
    _y: libc::c_ushort,
}

pub fn terminal_size() -> Option<(u16, u16)> {
    unsafe {
        if libc::isatty(libc::STDOUT_FILENO) != 1 {
            return None;
        }

        let mut winsize: libc::winsize = mem::zeroed();
        libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &mut winsize);
        if winsize.ws_row > 0 && winsize.ws_col > 0 {
            Some((winsize.ws_col as u16, winsize.ws_row as u16))
        } else {
            None
        }
    }
}
