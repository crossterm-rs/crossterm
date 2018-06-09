use winapi::um::winnt::HANDLE;
use std::mem::zeroed;
use winapi::um::processenv::{GetStdHandle};
use winapi::um::winbase::{STD_OUTPUT_HANDLE, STD_INPUT_HANDLE };

static mut ALTERNATEHANDLE: Option<HANDLE> = None;
static mut CONSOLE_OUTPUT_HANDLE: Option<HANDLE> = None;
static mut CONSOLE_INPUT_HANDLE: Option<HANDLE> = None;


pub fn register_new_alternate_handle(handle: Handle) -> HANDLE
{
    unsafe
    {
        ALTERNATEHANDLE = handle;
    }

    ALTERNATEHANDLE.unwrap();
}

pub fn clear_alternate_screen()
{
    unsafe
    {
        ALTERNATEHANDLE = None;
    }
}

/// Get the std_output_handle of the console
pub fn get_output_handle() -> HANDLE {
    unsafe {
        if let Some(alternate_handle) = ALTERNATEHANDLE
        {
            alternate_handle
        }
        else if let Some(handle) = CONSOLE_OUTPUT_HANDLE {
            handle
        } else {
            let handle = GetStdHandle(STD_OUTPUT_HANDLE);

            if !is_valid_handle(&handle)
                {
                    panic!("Cannot get output handle")
                }

            CONSOLE_OUTPUT_HANDLE = Some(handle);
            handle
        }
    }
}

/// Get the std_input_handle of the console
pub fn get_input_handle() -> HANDLE {
    unsafe {
        if let Some(handle) = CONSOLE_INPUT_HANDLE {
            handle
        } else {
            let handle = GetStdHandle(STD_INPUT_HANDLE);

            if !is_valid_handle(&handle)
                {
                    panic!("Cannot get input handle")
                }

            CONSOLE_INPUT_HANDLE = Some(handle);
            handle
        }
    }
}

/// Checks if the console handle is an invalid handle value.
pub fn is_valid_handle(handle: &HANDLE) -> bool {
    if *handle == INVALID_HANDLE_VALUE {
        false
    } else {
        true
    }
}