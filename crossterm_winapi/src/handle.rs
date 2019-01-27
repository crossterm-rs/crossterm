//! This module contains some logic for working with the console handle.

use winapi::um::{
    fileapi::{CreateFileW, OPEN_EXISTING},
    handleapi::INVALID_HANDLE_VALUE,
    processenv::GetStdHandle,
    winbase::{STD_INPUT_HANDLE, STD_OUTPUT_HANDLE},
    winnt::{FILE_SHARE_READ, FILE_SHARE_WRITE, GENERIC_READ, GENERIC_WRITE, HANDLE},
};

use std::io::{self, Result};
use std::ops::Deref;
use std::ptr::null_mut;

/// This enum represents the different handles that could be requested.
///
/// Some more details could be found [here](https://docs.microsoft.com/en-us/windows/console/getstdhandle#parameters)
pub enum HandleType {
    /// This represents the `STD_OUTPUT_HANDLE`
    OutputHandle,
    /// This represents the `STD_INPUT_HANDLE`
    InputHandle,
    /// This represents the `CONOUT$` file handle
    /// When using multiple screen buffers this will always point to the to the current screen output buffer.
    CurrentOutputHandle,
    /// This represents the `CONIN$` file handle.
    /// When using multiple screen buffers this will always point to the to the current screen input buffer.
    CurrentInputHandle,
}

/// This abstracts away some WinaApi calls to set and get some console handles.
///
// Wraps the underlying WinApi type: [HANDLE]
pub struct Handle {
    handle: HANDLE,
}

impl Handle {
    pub fn new(handle: HandleType) -> Result<Handle> {
        let handle = match handle {
            HandleType::OutputHandle => Handle::output_handle(),
            HandleType::InputHandle => Handle::input_handle(),
            HandleType::CurrentOutputHandle => Handle::current_out_handle(),
            HandleType::CurrentInputHandle => Handle::current_in_handle(),
        }?;

        Ok(Handle { handle })
    }

    /// Get the handle of the active screen buffer.
    /// When using multiple screen buffers this will always point to the to the current screen output buffer.
    ///
    /// On success this function returns the `HANDLE` to `STD_OUTPUT_HANDLE`.
    ///
    /// This function uses `CONOUT$` to create a file handle to the current output buffer.
    ///
    /// Wraps the underlying function call: [CreateFileW]
    /// link: [https://docs.microsoft.com/en-us/windows/desktop/api/fileapi/nf-fileapi-createfilew]
    pub fn current_out_handle() -> Result<HANDLE> {
        let utf16: Vec<u16> = "CONOUT$\0".encode_utf16().collect();
        let utf16_ptr: *const u16 = utf16.as_ptr();

        let handle = unsafe {
            CreateFileW(
                utf16_ptr,
                GENERIC_READ | GENERIC_WRITE,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                null_mut(),
                OPEN_EXISTING,
                0,
                null_mut(),
            )
        };

        if !Handle::is_valid_handle(&handle) {
            println!("invalid!!");
            return Err(io::Error::last_os_error());
        }

        Ok(handle)
    }

    /// Get the handle of the active input screen buffer.
    /// When using multiple screen buffers this will always point to the to the current screen input buffer.
    ///
    /// On success this function returns the `HANDLE` to `STD_INPUT_HANDLE`.
    ///
    /// This function uses `CONIN$` to create a file handle to the current input buffer.
    ///
    /// Wraps the underlying function call: [CreateFileW]
    /// link: [https://docs.microsoft.com/en-us/windows/desktop/api/fileapi/nf-fileapi-createfilew]
    pub fn current_in_handle() -> Result<HANDLE> {
        let utf16: Vec<u16> = "CONIN$\0".encode_utf16().collect();
        let utf16_ptr: *const u16 = utf16.as_ptr();

        let handle = unsafe {
            CreateFileW(
                utf16_ptr,
                GENERIC_READ | GENERIC_WRITE,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                null_mut(),
                OPEN_EXISTING,
                0,
                null_mut(),
            )
        };

        if !Handle::is_valid_handle(&handle) {
            return Err(io::Error::last_os_error());
        }

        Ok(handle)
    }

    /// Get the handle of the output screen buffer.
    ///
    /// On success this function returns the `HANDLE` to `STD_OUTPUT_HANDLE`.
    ///
    /// Wraps the underlying function call: [GetStdHandle] whit argument `STD_OUTPUT_HANDLE`
    /// link: [https://docs.microsoft.com/en-us/windows/console/getstdhandle]
    pub fn output_handle() -> Result<HANDLE> {
        unsafe {
            let handle = GetStdHandle(STD_OUTPUT_HANDLE);

            if !Handle::is_valid_handle(&handle) {
                return Err(io::Error::last_os_error());
            }

            Ok(handle)
        }
    }

    /// Get the handle of the input screen buffer.
    ///
    /// On success this function returns the `HANDLE` to `STD_INPUT_HANDLE`.
    ///
    /// Wraps the underlying function call: [GetStdHandle] whit argument `STD_INPUT_HANDLE`
    /// link: [https://docs.microsoft.com/en-us/windows/console/getstdhandle]
    pub fn input_handle() -> Result<HANDLE> {
        unsafe {
            let handle = GetStdHandle(STD_INPUT_HANDLE);

            if !Handle::is_valid_handle(&handle) {
                return Err(io::Error::last_os_error());
            }

            Ok(handle)
        }
    }

    /// Checks if the console handle is an invalid handle value.
    ///
    /// This is done by checking if the passed `HANDLE` is equal to `INVALID_HANDLE_VALUE`
    pub fn is_valid_handle(handle: &HANDLE) -> bool {
        if *handle == INVALID_HANDLE_VALUE {
            false
        } else {
            true
        }
    }
}

impl Deref for Handle {
    type Target = HANDLE;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.handle
    }
}

impl From<HANDLE> for Handle {
    fn from(handle: HANDLE) -> Self {
        Handle { handle }
    }
}

#[cfg(test)]
mod test {
    use super::{Handle, HandleType};

    #[test]
    fn get_handle() {
        let out_put_handle = Handle::new(HandleType::OutputHandle).unwrap();
        let out_put_handle = Handle::new(HandleType::InputHandle).unwrap();
        let curr_out_put_handle = Handle::new(HandleType::CurrentOutputHandle).unwrap();
        let curr_out_put_handle = Handle::new(HandleType::CurrentInputHandle).unwrap();
    }
}
