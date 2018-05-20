/// Get the terminal size
pub fn terminal_size() -> (u16, u16) {
    let csbi = super::kernel::get_console_screen_buffer_info();
    (
        (csbi.srWindow.Right - csbi.srWindow.Left) as u16,
        (csbi.srWindow.Bottom - csbi.srWindow.Top) as u16,
    )
}