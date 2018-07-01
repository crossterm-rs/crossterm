use ScreenManager;
use std::sync::Mutex;
use std::rc::Rc;


/// Get the terminal size
pub fn terminal_size(screen_manager: &Rc<Mutex<ScreenManager>>) -> (u16, u16) {
    let csbi = super::kernel::get_console_screen_buffer_info(screen_manager);
    (
        (csbi.srWindow.Right - csbi.srWindow.Left) as u16,
        (csbi.srWindow.Bottom - csbi.srWindow.Top) as u16,
    )
}

/// Exit the current process.
pub fn exit()
{
    ::std::process::exit(256);
}