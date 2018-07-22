use std::rc::Rc;
use std::sync::Mutex;
use ScreenManager;

use super::csbi;

/// Get the terminal size
pub fn terminal_size(screen_manager: &Rc<Mutex<ScreenManager>>) -> (u16, u16) {
    if let Ok(csbi) = csbi::get_csbi(screen_manager) {
        (
            (csbi.srWindow.Right - csbi.srWindow.Left) as u16,
            (csbi.srWindow.Bottom - csbi.srWindow.Top) as u16,
        )
    } else {
        return (0, 0);
    }
}

/// Exit the current process.
pub fn exit() {
    ::std::process::exit(256);
}
