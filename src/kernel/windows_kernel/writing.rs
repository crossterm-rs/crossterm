//! This module contains the logic to write to the terminal.

use winapi::ctypes::c_void;
use winapi::shared::ntdef::NULL;
use winapi::um::consoleapi::WriteConsoleW;
use winapi::um::wincon::{
    self, FillConsoleOutputAttribute, FillConsoleOutputCharacterA, WriteConsoleOutputA, CHAR_INFO,
    COORD, PSMALL_RECT,
};

use super::{csbi, handle, kernel, TerminalOutput, HANDLE};

use std::io::{self, ErrorKind, Result};
use std::str;
use std::sync::Arc;

/// Fill a certain block with characters.
pub fn fill_console_output_character(
    cells_written: &mut u32,
    start_location: COORD,
    cells_to_write: u32,
    screen_manager: &Arc<TerminalOutput>,
) -> bool {
    let handle = handle::get_current_handle(screen_manager).unwrap();

    unsafe {
        // fill the cells in console with blanks
        let success = FillConsoleOutputCharacterA(
            handle,
            ' ' as i8,
            cells_to_write,
            start_location,
            cells_written,
        );
        kernel::is_true(success)
    }
}

/// Set console ouput attribute for certain block.
pub fn fill_console_output_attribute(
    cells_written: &mut u32,
    start_location: COORD,
    cells_to_write: u32,
    screen_manager: &Arc<TerminalOutput>,
) -> bool {
    // Get the position of the current console window

    let (csbi, mut handle) = csbi::get_csbi_and_handle(screen_manager).unwrap();

    let success;

    unsafe {
        success = FillConsoleOutputAttribute(
            handle,
            csbi.wAttributes,
            cells_to_write,
            start_location,
            cells_written,
        );
    }

    kernel::is_true(success)
}

/// Write console output.
pub fn write_console_output(
    write_buffer: &HANDLE,
    copy_buffer: &mut [CHAR_INFO; 160],
    buffer_size: COORD,
    buffer_coord: COORD,
    source_buffer: PSMALL_RECT,
) -> Result<()> {
    unsafe {
        if !kernel::is_true(
            WriteConsoleOutputA(
                *write_buffer,            // screen buffer to write to
                copy_buffer.as_mut_ptr(), // buffer to copy into
                buffer_size,              // col-row size of chiBuffer
                buffer_coord,             // top left dest. cell in chiBuffer
                source_buffer,
            ), // screen buffer source rectangle
        ) {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Could not write to terminal",
            ));
        }
    }

    Ok(())
}

/// Write utf8 buffer to console.
pub fn write_char_buffer(handle: &HANDLE, buf: &[u8]) -> ::std::io::Result<usize> {
    // get string from u8[] and parse it to an c_str
    let mut utf8 = match str::from_utf8(buf) {
        Ok(string) => string,
        Err(_) => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Could not parse input to utf8 string.",
            ))
        }
    };

    let utf16: Vec<u16> = utf8.encode_utf16().collect();
    let utf16_ptr: *const c_void = utf16.as_ptr() as *const _ as *const c_void;

    // get buffer info
    let csbi = csbi::get_csbi_by_handle(handle)?;

    // get current position
    let current_pos = COORD {
        X: csbi.dwCursorPosition.X,
        Y: csbi.dwCursorPosition.Y,
    };

    let mut cells_written: u32 = 0;
    let mut success = false;
    // write to console
    unsafe {
        success = kernel::is_true(WriteConsoleW(
            *handle,
            utf16_ptr,
            utf16.len() as u32,
            &mut cells_written,
            NULL,
        ));
    }

    match success {
        // think this is wrong could be done better!
        true => Ok(utf8.as_bytes().len()),
        false => Ok(0),
    }
}
