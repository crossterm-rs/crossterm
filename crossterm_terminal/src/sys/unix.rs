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
        (size.ws_co, size.ws_row)
    } else {
        (0, 0)
    }
}
