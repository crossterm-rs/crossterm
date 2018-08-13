//! This contains the logic for working with the console buffer.

use winapi::shared::minwindef::{FALSE, TRUE};
use winapi::shared::ntdef::NULL;
use winapi::um::minwinbase::SECURITY_ATTRIBUTES;
use winapi::um::wincon::{
    CreateConsoleScreenBuffer, GetConsoleScreenBufferInfo, SetConsoleActiveScreenBuffer,
    SetConsoleScreenBufferSize, CONSOLE_SCREEN_BUFFER_INFO, CONSOLE_TEXTMODE_BUFFER, COORD,
};
use winapi::um::winnt::HANDLE;
use winapi::um::winnt::{FILE_SHARE_READ, FILE_SHARE_WRITE, GENERIC_READ, GENERIC_WRITE};

use super::{handle, kernel, Empty, Stdout};

use std::sync::{Once, ONCE_INIT};
use std::io::{self, ErrorKind, Result};
use std::mem::size_of;
use std::sync::Arc;

/// Create a new console screen buffer info struct.
pub fn get_csbi(screen_manager: &Arc<Stdout>) -> Result<CONSOLE_SCREEN_BUFFER_INFO> {
    let mut csbi = CONSOLE_SCREEN_BUFFER_INFO::empty();
    let success;

    unsafe {
        success = GetConsoleScreenBufferInfo(handle::get_current_handle(screen_manager)?, &mut csbi)
    }

    if success == 0 {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Could not get console screen buffer info",
        ));
    }

    Ok(csbi)
}

/// Get buffer info and handle of the current screen.
pub fn get_csbi_and_handle(
    screen_manager: &Arc<Stdout>,
) -> Result<(CONSOLE_SCREEN_BUFFER_INFO, HANDLE)> {
    let handle = handle::get_current_handle(screen_manager)?;
    let csbi = get_csbi_by_handle(&handle)?;

    return Ok((csbi, handle));
}

/// Create a new console screen buffer info struct.
pub fn get_csbi_by_handle(handle: &HANDLE) -> Result<CONSOLE_SCREEN_BUFFER_INFO> {
    let mut csbi = CONSOLE_SCREEN_BUFFER_INFO::empty();

    unsafe {
        if !kernel::is_true(GetConsoleScreenBufferInfo(*handle, &mut csbi)) {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Could not get console screen buffer info",
            ));
        }
    }

    Ok(csbi)
}

/// Set the console screen buffer size
pub fn set_console_screen_buffer_size(size: COORD, screen_manager: &Arc<Stdout>) -> bool {
    let handle = handle::get_current_handle(screen_manager).unwrap();

    unsafe {
        if !kernel::is_true(SetConsoleScreenBufferSize(handle, size)) {
            return false;
        } else {
            return true;
        }
    }
}

/// Create new console screen buffer. This can be used for alternate screen.
pub fn create_console_screen_buffer() -> HANDLE {
    let mut security_attr: SECURITY_ATTRIBUTES = SECURITY_ATTRIBUTES {
        nLength: size_of::<SECURITY_ATTRIBUTES>() as u32,
        lpSecurityDescriptor: NULL,
        bInheritHandle: TRUE,
    };

    unsafe {
        let new_screen_buffer = CreateConsoleScreenBuffer(
            GENERIC_READ |           // read/write access
                GENERIC_WRITE,
            FILE_SHARE_READ | FILE_SHARE_WRITE, // shared
            &mut security_attr,                 // default security attributes
            CONSOLE_TEXTMODE_BUFFER,            // must be TEXTMODE
            NULL,
        );
        new_screen_buffer
    }
}

/// Set the active screen buffer to the given handle. This can be used for alternate screen.
pub fn set_active_screen_buffer(new_buffer: HANDLE) -> Result<()> {
    unsafe {
        if !kernel::is_true(SetConsoleActiveScreenBuffer(new_buffer)) {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Could not set the active screen buffer",
            ));
        }
    }
    Ok(())
}

static GET_ORIGINAL_CONSOLE_COLOR: Once = ONCE_INIT;
static mut original_console_color: u16 = 0;

pub fn get_original_console_color() -> u16 {
    GET_ORIGINAL_CONSOLE_COLOR.call_once(|| {
        let handle = handle::get_output_handle().unwrap();
        let csbi = get_csbi_by_handle(&handle).unwrap();
        unsafe { original_console_color = csbi.wAttributes as u16 };
    });
    return unsafe { original_console_color };
}
