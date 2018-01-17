use winapi;
use kernel32;
use super::handle;

/// Enables ansi for windows terminals.
pub fn enable_ansi_support() {
    let enable_ansi_code: u32 = 7;
    let output_handle = handle::get_output_handle();
    set_console_mode(output_handle, enable_ansi_code)
}

/// Set the console mode of the windows terminal.
fn set_console_mode(handle: winapi::HANDLE, console_mode: u32) {
    unsafe {
        winapi::SW_HIDE;
        kernel32::SetConsoleMode(handle, console_mode);
    }
}
