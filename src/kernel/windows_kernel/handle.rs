//! This module contains some logic for working with the console handle.

use super::*;
use winapi::shared::minwindef::DWORD;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::fileapi::{CreateFileW, OPEN_EXISTING};
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::processenv::GetStdHandle;
use winapi::um::winbase::{STD_INPUT_HANDLE, STD_OUTPUT_HANDLE};
use winapi::um::winnt::{FILE_SHARE_WRITE, FILE_SHARE_READ, GENERIC_READ, GENERIC_WRITE};

use std::io::{self, Result};
use std::ptr::null_mut;

/// Get the handle of the active screen.
pub fn get_current_out_handle() -> Result<HANDLE> {
    let utf16: Vec<u16> = "CONOUT$\0".encode_utf16().collect();
    let utf16_ptr: *const u16 = utf16.as_ptr();

    let handle = unsafe { CreateFileW(
        utf16_ptr, GENERIC_READ | GENERIC_WRITE,
        FILE_SHARE_READ | FILE_SHARE_WRITE, null_mut(), OPEN_EXISTING,
        0, null_mut(),
    )};

    if !is_valid_handle(&handle) {
        return Err(io::Error::last_os_error());
    }

    Ok(handle)
}

/// Get the handle of the active screen.
pub fn get_current_in_handle() -> Result<HANDLE> {
    let utf16: Vec<u16> = "CONIN$\0".encode_utf16().collect();
    let utf16_ptr: *const u16 = utf16.as_ptr();

    let handle = unsafe { CreateFileW(
        utf16_ptr, GENERIC_READ | GENERIC_WRITE,
        FILE_SHARE_READ | FILE_SHARE_WRITE, null_mut(), OPEN_EXISTING,
        0, null_mut(),
    )};

    if !is_valid_handle(&handle) {
        return Err(io::Error::last_os_error());
    }

    Ok(handle)
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
