use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};

pub fn exit() {
    ::std::process::exit(0);
}

/// Get the current terminal size.
pub fn get_terminal_size() -> (u16, u16) {
    // http://rosettacode.org/wiki/Terminal_control/Dimensions#Library:_BSD_libc
    let mut size = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    let r = unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ.into(), &mut size) };

    if r == 0 {
        // because crossterm works starts counting at 0
        // and unix terminal starts at cell 1
        // you have subtract one to get 0-based results.
        (size.ws_col - 1, size.ws_row - 1)
    } else {
        (0, 0)
    }
}
