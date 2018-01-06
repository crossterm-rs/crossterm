use winapi;
use kernel32;

// static mut CONSOLE_INPUT_HANDLE:Option<winapi::HANDLE> = None;
static mut CONSOLE_OUTPUT_HANDLE: Option<winapi::HANDLE> = None;

/// Get the std_output_handle of the console
pub fn get_output_handle() -> winapi::HANDLE {
    unsafe {
        if let Some(handle) = CONSOLE_OUTPUT_HANDLE {
            handle
        } else {
            let handle = kernel32::GetStdHandle(winapi::STD_OUTPUT_HANDLE);
            CONSOLE_OUTPUT_HANDLE = Some(handle);
            handle
        }
    }
}

/// Checks if the console handle is an invalid handle value.
pub fn is_valid_handle(handle: &winapi::HANDLE) -> bool {
    if *handle == winapi::INVALID_HANDLE_VALUE {
        true
    } else {
        false
    }
}
