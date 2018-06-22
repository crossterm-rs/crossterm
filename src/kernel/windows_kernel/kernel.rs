//! This module is the core of all the `WINAPI` actions. All unsafe `WINAPI` function call are done here.
use winapi::um::winnt::HANDLE;
use winapi::um::winbase::{STD_OUTPUT_HANDLE, STD_INPUT_HANDLE };
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::processenv::{GetStdHandle};
use winapi::um::consoleapi::{SetConsoleMode,GetConsoleMode, };
use winapi::shared::minwindef::{TRUE, FALSE};
use winapi::um::wincon;
use winapi::um::wincon::
{
    SetConsoleWindowInfo, SetConsoleCursorPosition, SetConsoleTextAttribute, SetConsoleScreenBufferSize, CreateConsoleScreenBuffer,SetConsoleActiveScreenBuffer, SetConsoleCursorInfo,
    GetLargestConsoleWindowSize, GetConsoleScreenBufferInfo,
    FillConsoleOutputCharacterA, FillConsoleOutputAttribute,WriteConsoleOutputCharacterA,WriteConsoleOutputAttribute,
    CONSOLE_SCREEN_BUFFER_INFO, SMALL_RECT, COORD, CHAR_INFO, PSMALL_RECT, CONSOLE_CURSOR_INFO
};

use super::{Empty};
static mut CONSOLE_OUTPUT_HANDLE: Option<HANDLE> = None;
static mut CONSOLE_INPUT_HANDLE: Option<HANDLE> = None;

/// Get the std_output_handle of the console
pub fn get_output_handle() -> HANDLE {
    unsafe {
        if let Some(handle) = CONSOLE_OUTPUT_HANDLE {
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
fn is_valid_handle(handle: &HANDLE) -> bool {
    if *handle == INVALID_HANDLE_VALUE {
        false
    } else {
        true
    }
}

pub fn get_console_screen_buffer_info() -> CONSOLE_SCREEN_BUFFER_INFO {
    let output_handle = get_output_handle();
    let mut csbi = CONSOLE_SCREEN_BUFFER_INFO::empty();
    let success;

    unsafe { success = GetConsoleScreenBufferInfo(output_handle, &mut csbi) }

    if success == 0 {
        panic!("Cannot get console screen buffer info");
    }

    csbi
}

pub fn get_largest_console_window_size() -> COORD
{
    let output_handle = get_output_handle();

    unsafe {
        GetLargestConsoleWindowSize(output_handle)
    }
}

pub fn get_original_console_color() -> u16 {
    let console_buffer_info = get_console_screen_buffer_info();
    console_buffer_info.wAttributes as u16
}

pub fn set_console_mode(handle: &HANDLE, console_mode: u32) -> bool
{
    unsafe {
        let success = SetConsoleMode(*handle, console_mode);
        return is_true(success);
    }
}

pub fn get_console_mode(handle: &HANDLE, current_mode: &mut u32) -> bool
{
    unsafe {
        let success = GetConsoleMode(*handle, &mut *current_mode);
        return is_true(success);
    }
}

pub fn set_console_cursor_position(x: i16, y: i16)
{
    if x < 0 || x >= <i16>::max_value() {
        panic!("X: {}, Argument Out of Range Exception", x);
    }

    if y < 0 || y >= <i16>::max_value() {
        panic!("Y: {}, Argument Out of Range Exception", y);
    }

    let output_handle = get_output_handle();
    let position = COORD { X: x, Y: y };

    unsafe {
        let success = SetConsoleCursorPosition(output_handle, position);

        if success == 0 {
            panic!("Argument out of range.");
        }
    }
}

pub fn cursor_visibility(visable: bool)
{
    let handle = get_output_handle();

    let cursor_info = CONSOLE_CURSOR_INFO
    {
        dwSize: 100,
        bVisible: if visable { FALSE } else {TRUE}
    };

    unsafe
    {
        SetConsoleCursorInfo(handle, &cursor_info);
    }
}

pub fn set_console_text_attribute(value: u16)
{
    let output_handle = get_output_handle();

    unsafe {
        SetConsoleTextAttribute(output_handle, value);
    }
}

pub fn set_console_info(absolute: bool, rect: &SMALL_RECT) -> bool
{

    let output_handle = get_output_handle();

    let absolute = match absolute {  true => 1, false => 0, };
    unsafe
    {
        let success = SetConsoleWindowInfo(output_handle,absolute ,rect);
        is_true(success)
    }
}

pub fn set_console_screen_buffer_size(  size: COORD) -> bool
{
    let output_handle = get_output_handle();

    unsafe
        {
            let success = SetConsoleScreenBufferSize(output_handle, size);
            is_true(success)
        }
}

pub fn fill_console_output_character(cells_written: &mut u32, start_location: COORD, cells_to_write: u32) -> bool
{
    let output_handle = get_output_handle();

    unsafe {
        // fill the cells in console with blanks
        let success = FillConsoleOutputCharacterA (
            output_handle,
            ' ' as i8,
            cells_to_write,
            start_location,
            cells_written,
        );
        is_true(success)
    }
}

pub fn fill_console_output_attribute(cells_written: &mut u32, start_location: COORD, cells_to_write: u32) -> bool
{
    // Get the position of the current console window
    let csbi = get_console_screen_buffer_info();
    let output_handle = get_output_handle();

    let success;

    unsafe {
        success = FillConsoleOutputAttribute (
            output_handle,
            csbi.wAttributes,
            cells_to_write,
            start_location,
            cells_written,
        );
    }

    is_true(success)
}

pub fn create_console_screen_buffer() -> HANDLE
{
    use winapi::shared::ntdef::NULL;
    use winapi::um::wincon::CONSOLE_TEXTMODE_BUFFER;
    use winapi::um::winnt::{GENERIC_READ, GENERIC_WRITE, FILE_SHARE_READ, FILE_SHARE_WRITE};
    use winapi::um::minwinbase::SECURITY_ATTRIBUTES;
    use std::mem::size_of;

    unsafe
    {
        let mut security_attr: SECURITY_ATTRIBUTES = SECURITY_ATTRIBUTES
        {
            nLength: size_of::<SECURITY_ATTRIBUTES>() as u32,
            lpSecurityDescriptor: NULL,
            bInheritHandle: TRUE
        };

        let new_screen_buffer = CreateConsoleScreenBuffer(
            GENERIC_READ |           // read/write access
                GENERIC_WRITE,
            FILE_SHARE_READ |
                FILE_SHARE_WRITE,        // shared
            &mut security_attr,                    // default security attributes
            CONSOLE_TEXTMODE_BUFFER, // must be TEXTMODE
            NULL
        );
        new_screen_buffer
    }
}

pub fn set_active_screen_buffer(new_buffer: HANDLE)
{
    unsafe
    {
        if !is_true(SetConsoleActiveScreenBuffer(new_buffer))
        {
            panic!("Cannot set active screen buffer");
        }
    }
}

pub fn read_console_output(read_buffer: &HANDLE, copy_buffer: &mut [CHAR_INFO;160], buffer_size: COORD, buffer_coord: COORD, source_buffer: PSMALL_RECT)
{
    use self::wincon::ReadConsoleOutputA;

    unsafe
    {
        if !is_true(ReadConsoleOutputA(
            *read_buffer,   // screen buffer to read from
            copy_buffer.as_mut_ptr(),    // buffer to copy into
            buffer_size,    // col-row size of chiBuffer
            buffer_coord,  // top left dest. cell in chiBuffer
            source_buffer) // screen buffer source rectangle
        ){

            panic!("Cannot read console output");
        }
    }
}

pub fn write_console_output(write_buffer: &HANDLE, copy_buffer: &mut [CHAR_INFO;160], buffer_size: COORD, buffer_coord: COORD, source_buffer: PSMALL_RECT)
{
    use self::wincon::WriteConsoleOutputA;

    unsafe
        {
            if !is_true(WriteConsoleOutputA(
                *write_buffer,        // screen buffer to write to
                copy_buffer.as_mut_ptr(),      // buffer to copy into
                buffer_size,   // col-row size of chiBuffer
                buffer_coord,  // top left dest. cell in chiBuffer
                source_buffer)// screen buffer source rectangle
            ){

                panic!("Cannot write to console output");
            }
        }
}

pub fn write_char_buffer(handle: HANDLE, buf: &[u8])
{
    use std::ffi::CString;
    use std::str;

    // get buffer info
    let csbi = get_console_screen_buffer_info();

    // get string from u8[] and parse it to an c_str
    let mut data = str::from_utf8(buf).unwrap();
    let c_str = CString::new(data).unwrap();
    let ptr: *const u16 = c_str.as_ptr() as *const u16;
    panic!("{:?}", ptr);
    // get current position
    let current_pos = COORD {X: csbi.dwCursorPosition.X, Y: csbi.dwCursorPosition.Y};

    let mut cells_written: u32 = 0;

    // write to console
    unsafe
    {

        WriteConsoleOutputAttribute(handle, ptr, data.len() as u32, current_pos, &mut cells_written);
    }
}

/// Parse integer to an bool
fn is_true(value: i32) -> bool
{
    if value == 0{
        return false;
    }
    else{
        return true;
    }
}
