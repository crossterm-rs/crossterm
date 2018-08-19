//! This module contains some logic for working with the console handle.

use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::processenv::GetStdHandle;
use winapi::um::winbase::{STD_INPUT_HANDLE, STD_OUTPUT_HANDLE};
use super::*;

use std::sync::Arc;
use std::io::{self,  Result};

/// Get the global stored handle whits provides access to the current screen.
pub fn get_current_handle(stdout: &Arc<TerminalOutput>) -> Result<HANDLE> {
    let handle: Result<HANDLE>;

    let winapi_stdout: &WinApiOutput = match stdout
        .as_any()
        .downcast_ref::<WinApiOutput>()
        {
            Some(win_api) => win_api,
            None => return Err(io::Error::new(io::ErrorKind::Other,"Could not convert to winapi screen write, this could happen when the user has an ANSI screen write and is calling the platform specific operations 'get_cursor_pos' or 'get_terminal_size'"))
        };


    handle = Ok(winapi_stdout.get_handle());

    return handle;
}

/// Get the std_output_handle of the console
pub fn get_output_handle() -> Result<HANDLE> {
    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);

        if !is_valid_handle(&handle) {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Could not get output handle!",
            ));
        }
        Ok(handle)
    }
}

/// Get the std_input_handle of the console
pub fn get_input_handle() -> Result<HANDLE> {
    unsafe {
        let handle = GetStdHandle(STD_INPUT_HANDLE);

        if !is_valid_handle(&handle) {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Could not get input handle",
            ));
        }

        Ok(handle)
    }
}

/// Checks if the console handle is an invalid handle value.
fn is_valid_handle(handle: &HANDLE) -> bool {
    if *handle == INVALID_HANDLE_VALUE {
        false
    } else {
        true
    }
}
