extern crate winapi;
extern crate kernel32;

use super::handle;
use self::winapi::CONSOLE_SCREEN_BUFFER_INFO;

use std::mem;

/// Get console screen buffer info.
pub fn get_console_screen_buffer_info() -> Option<CONSOLE_SCREEN_BUFFER_INFO>
{
    let handle = handle::get_output_handle();
    let mut csbi: CONSOLE_SCREEN_BUFFER_INFO = unsafe { mem::zeroed() };
    
    unsafe 
    {
        if let Some(x) = handle
        {
            match kernel32::GetConsoleScreenBufferInfo(x, &mut csbi)
            {
                 0 => None,
                  _ => Some(csbi),
            }
        }
        else {
            None
        }
    }
}

/// Get the current console colors.
pub fn get_original_console_color() -> u16
{
    let console_buffer_info = get_console_screen_buffer_info();

    if let Some(buffer_info) = console_buffer_info
    {
        buffer_info.wAttributes as u16
    }    
    else{
        300
    }       
}
