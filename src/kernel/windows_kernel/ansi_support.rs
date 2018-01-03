extern crate kernel32;
extern crate winapi;

use self::winapi::HANDLE;

use super::handle;

/// Enables ansi for windows terminals. 
pub fn enable_ansi_support() -> Result<(), u64>
{
    const ENABLE_ANSI_CODES: u32 = 7;

    let std_out_handle = handle::get_output_handle();

    match std_out_handle
    {
        Some(handle) => { set_console_mode(handle, ENABLE_ANSI_CODES); Ok(()) },
        None => return Err(0)
    }
}

/// Set the console mode of the windows terminal.
fn set_console_mode(handle: HANDLE, enable_ansi_code: u32)
{
    unsafe
    {
        kernel32::SetConsoleMode(handle, enable_ansi_code);
    }
}
