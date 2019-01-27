use crossterm_winapi::ScreenBuffer;

/// Exit the current process.
pub fn exit() {
    ::std::process::exit(256);
}

#[cfg(windows)]
pub fn get_terminal_size() -> (u16, u16) {
    if let Ok(buffer) = ScreenBuffer::current() {
        let size = buffer.info().unwrap().terminal_size();
        (size.width as u16, size.height as u16)
    } else {
        (0, 0)
    }
}
