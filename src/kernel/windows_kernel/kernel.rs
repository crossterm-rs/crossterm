use winapi::shared::minwindef::DWORD;
use winapi::um::winnt::HANDLE;
use winapi::um::winbase::{STD_OUTPUT_HANDLE, STD_INPUT_HANDLE };
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::processenv::{GetStdHandle};
use winapi::um::consoleapi::{SetConsoleMode,GetConsoleMode};
use winapi::um::wincon::{ SetConsoleWindowInfo, SetConsoleCursorPosition, SetConsoleTextAttribute, SetConsoleScreenBufferSize,
                          GetLargestConsoleWindowSize, GetConsoleScreenBufferInfo,
                          FillConsoleOutputCharacterA, FillConsoleOutputAttribute, ENABLE_VIRTUAL_TERMINAL_PROCESSING,ENABLE_VIRTUAL_TERMINAL_INPUT,
                          CONSOLE_SCREEN_BUFFER_INFO, SMALL_RECT, COORD, DISABLE_NEWLINE_AUTO_RETURN
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
pub fn is_valid_handle(handle: &HANDLE) -> bool {
    if *handle == INVALID_HANDLE_VALUE {
        false
    } else {
        true
    }
}

/// Get console screen buffer info.
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

/// Enables ansi for windows terminals.
pub fn try_enable_ansi_support() -> bool {

    let output_handle = get_output_handle();

    let mut dw_mode: DWORD = 0;
    if !get_console_mode(&output_handle, &mut dw_mode)
    {
       return false;
    }

    dw_mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;
    if !set_console_mode(&output_handle, dw_mode)
    {
        return false;
    }

    return true;
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
