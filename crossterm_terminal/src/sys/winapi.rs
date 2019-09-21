use crossterm_utils::Result;
use crossterm_winapi::ScreenBuffer;

/// Exit the current process.
pub fn exit() {
    ::std::process::exit(256);
}

#[cfg(windows)]
pub fn get_terminal_size() -> Result<(u16, u16)> {
    let terminal_size = ScreenBuffer::current()?.info()?.terminal_size();
    // windows starts counting at 0, unix at 1, add one to replicated unix behaviour.
    Ok(((terminal_size.width + 1) as u16, (terminal_size.height + 1) as u16))
}
