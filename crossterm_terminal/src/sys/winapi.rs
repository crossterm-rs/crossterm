use crossterm_utils::Result;
use crossterm_winapi::ScreenBuffer;

/// Exit the current process.
pub fn exit() {
    ::std::process::exit(256);
}

#[cfg(windows)]
pub fn get_terminal_size() -> Result<(u16, u16)> {
    let buffer = ScreenBuffer::current()?;
    Ok(buffer.info()?.terminal_size().into())
}
