//! This module contains the logic to write to the terminal.

use winapi::ctypes::c_void;
use winapi::shared::ntdef::NULL;
use winapi::um::consoleapi::WriteConsoleW;
use winapi::um::wincon::{WriteConsoleOutputA, CHAR_INFO, COORD, PSMALL_RECT};
use winapi::um::winnt::HANDLE;

use crossterm_winapi::{is_true, ScreenBuffer};

use std::io::{self, Result};
use std::str;

/// Write console output.
pub fn write_console_output(
    write_buffer: &HANDLE,
    copy_buffer: &mut [CHAR_INFO; 160],
    buffer_size: COORD,
    buffer_coord: COORD,
    source_buffer: PSMALL_RECT,
) -> Result<()> {
    unsafe {
        if !is_true(
            WriteConsoleOutputA(
                *write_buffer,            // screen buffer to write to
                copy_buffer.as_mut_ptr(), // buffer to copy into
                buffer_size,              // col-row size of chiBuffer
                buffer_coord,             // top left dest. cell in chiBuffer
                source_buffer,
            ), // screen buffer source rectangle
        ) {
            return Err(io::Error::last_os_error());
        }
    }

    Ok(())
}

/// Write utf8 buffer to console.
pub fn write_char_buffer(handle: &HANDLE, buf: &[u8]) -> io::Result<usize> {
    // get string from u8[] and parse it to an c_str
    let utf8 = match str::from_utf8(buf) {
        Ok(string) => string,
        Err(_) => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Could not parse to utf8 string",
            ));
        }
    };

    let utf16: Vec<u16> = utf8.encode_utf16().collect();
    let utf16_ptr: *const c_void = utf16.as_ptr() as *const _ as *const c_void;

    // get buffer info
    match ScreenBuffer::from(*handle).info() {
        Ok(csbi) => {
            // get current position
            let _current_pos = COORD {
                X: csbi.cursor_pos().x,
                Y: csbi.cursor_pos().y,
            };

            let mut cells_written: u32 = 0;
            // write to console
            unsafe {
                if !is_true(WriteConsoleW(
                    *handle,
                    utf16_ptr,
                    utf16.len() as u32,
                    &mut cells_written,
                    NULL,
                )) {
                    return Err(io::Error::last_os_error());
                }
            }
            Ok(utf8.as_bytes().len())
        }
        Err(e) => Err(e),
    }
}
