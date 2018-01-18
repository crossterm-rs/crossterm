use winapi::um::winnt;
use winapi::um::winbase;
use winapi::um::handleapi;

use winapi::um::processenv::{GetStdHandle};

// static mut CONSOLE_INPUT_HANDLE:Option<winnt::HANDLE> = None;
static mut CONSOLE_OUTPUT_HANDLE: Option<winnt::HANDLE> = None;

/// Get the std_output_handle of the console
pub fn get_output_handle() -> winnt::HANDLE {
    unsafe {
        if let Some(handle) = CONSOLE_OUTPUT_HANDLE {
            handle
        } else {
            let handle = GetStdHandle(winbase::STD_OUTPUT_HANDLE);
            CONSOLE_OUTPUT_HANDLE = Some(handle);
            handle
        }
    }
}

/// Checks if the console handle is an invalid handle value.
pub fn is_valid_handle(handle: &winnt::HANDLE) -> bool {
    if *handle == handleapi::INVALID_HANDLE_VALUE {
        true
    } else {
        false
    }
}

