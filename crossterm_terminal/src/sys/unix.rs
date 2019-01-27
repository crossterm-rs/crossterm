use libc::{c_ushort, ioctl, STDOUT_FILENO, TCSADRAIN, TIOCGWINSZ};

pub fn exit() {
    ::std::process::exit(0);
}

/// A representation of the size of the current terminal.
#[repr(C)]
#[derive(Debug)]
pub struct UnixSize {
    /// number of rows
    pub rows: c_ushort,
    /// number of columns
    pub cols: c_ushort,
    pub ws_xpixel: c_ushort,
    pub ws_ypixel: c_ushort,
}

/// Get the current terminal size.
pub fn get_terminal_size() -> (u16, u16) {
    // http://rosettacode.org/wiki/Terminal_control/Dimensions#Library:_BSD_libc
    let us = UnixSize {
        rows: 0,
        cols: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };

    let r = unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &us) };

    if r == 0 {
        // because crossterm works starts counting at 0 and unix terminal starts at cell 1 you have subtract one to get 0-based results.
        (us.cols, us.rows)
    } else {
        (0, 0)
    }
}
