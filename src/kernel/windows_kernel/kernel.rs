//! This module is the core of all the `WINAPI` actions. All unsafe `WINAPI` function call are done here.
use Context;
use std::rc::Rc;

use winapi::um::winnt::HANDLE;
use winapi::um::winbase::{STD_OUTPUT_HANDLE, STD_INPUT_HANDLE };
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::processenv::{GetStdHandle};
use winapi::um::consoleapi::{SetConsoleMode,GetConsoleMode, };
use winapi::shared::ntdef::{NULL};
use winapi::shared::minwindef::{TRUE, FALSE};
use winapi::um::wincon;
use winapi::um::consoleapi::WriteConsoleW;
use winapi::um::wincon::
{
    ENABLE_PROCESSED_INPUT,
    WriteConsoleOutputW,
    WriteConsoleOutputCharacterA,
    WriteConsoleOutputCharacterW,
    SetConsoleWindowInfo, SetConsoleCursorPosition, SetConsoleTextAttribute, SetConsoleScreenBufferSize, CreateConsoleScreenBuffer,SetConsoleActiveScreenBuffer, SetConsoleCursorInfo,
    GetLargestConsoleWindowSize, GetConsoleScreenBufferInfo,
    FillConsoleOutputCharacterA, FillConsoleOutputAttribute,WriteConsoleOutputAttribute,
    CONSOLE_SCREEN_BUFFER_INFO, SMALL_RECT, COORD, CHAR_INFO, PSMALL_RECT, CONSOLE_CURSOR_INFO
};

use super::{Empty};
static mut CONSOLE_OUTPUT_HANDLE: Option<HANDLE> = None;
static mut CONSOLE_INPUT_HANDLE: Option<HANDLE> = None;

use super::super::super::manager::{ScreenManager, WinApiScreenManager};
use std::sync::Mutex;

/// Get the global stored handle.
pub fn get_current_handle(screen_manager: &mut WinApiScreenManager) -> &HANDLE
{
    return screen_manager.get_handle();
}

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
/// Create a new console screen buffer info struct.
pub fn get_console_screen_buffer_info(screen_manager: &Rc<Mutex<ScreenManager>>) -> CONSOLE_SCREEN_BUFFER_INFO {

    let mut screen_manager = screen_manager.lock().unwrap();
    let winapi_screen_manager: &mut WinApiScreenManager = match screen_manager.as_any().downcast_mut::<WinApiScreenManager>() {
        Some(win_api) => { win_api },
        None => panic!("")
    };

    let output_handle = get_current_handle(winapi_screen_manager);
    let mut csbi = CONSOLE_SCREEN_BUFFER_INFO::empty();
    let success;

    unsafe { success = GetConsoleScreenBufferInfo(*output_handle, &mut csbi) }

    if success == 0 {
        panic!("Cannot get console screen buffer info");
    }

    csbi
}

/// Create a new console screen buffer info struct.
pub fn get_console_screen_buffer_info_from_handle(handle: &HANDLE) -> CONSOLE_SCREEN_BUFFER_INFO {

    let mut csbi = CONSOLE_SCREEN_BUFFER_INFO::empty();
    let success;

    unsafe { success = GetConsoleScreenBufferInfo(*handle, &mut csbi) }

    if success == 0 {
        panic!("Cannot get console screen buffer info");
    }

    csbi
}

/// Get the larged console window size posible.
pub fn get_largest_console_window_size() -> COORD
{
    let output_handle = get_output_handle();

    unsafe {
        GetLargestConsoleWindowSize(output_handle)
    }
}

/// Get the original color of the terminal.
pub fn get_original_console_color(screen_manager: &Rc<Mutex<ScreenManager>>) -> u16 {
    let console_buffer_info = get_console_screen_buffer_info(screen_manager);
    console_buffer_info.wAttributes as u16
}

/// Set the console mode to the given console mode.
pub fn set_console_mode(handle: &HANDLE, console_mode: u32) -> bool
{
    unsafe {
        let success = SetConsoleMode(*handle, console_mode);
        return is_true(success);
    }
}

/// Get the console mode.
pub fn get_console_mode(handle: &HANDLE, current_mode: &mut u32) -> bool
{
    unsafe {
        let success = GetConsoleMode(*handle, &mut *current_mode);
        return is_true(success);
    }
}

/// Set the cursor position to the given x and y. Note that this is 0 based.
pub fn set_console_cursor_position(x: i16, y: i16, screen_manager: &Rc<Mutex<ScreenManager>>)
{
    if x < 0 || x >= <i16>::max_value() {
        panic!("X: {}, Argument Out of Range Exception", x);
    }

    if y < 0 || y >= <i16>::max_value() {
        panic!("Y: {}, Argument Out of Range Exception", y);
    }

    let mut screen_manager = screen_manager.lock().unwrap();
    let winapi_screen_manager: &mut WinApiScreenManager = match screen_manager.as_any().downcast_mut::<WinApiScreenManager>() {
        Some(win_api) => { win_api },
        None => panic!("")
    };

    let handle = get_current_handle(winapi_screen_manager);

    let position = COORD { X: x, Y: y };

    unsafe {
        let success = SetConsoleCursorPosition(*handle, position);

        if success == 0 {
            panic!("Argument out of range.");
        }
    }
}

/// change the cursor visibility.
pub fn cursor_visibility(visable: bool, screen_manager: &Rc<Mutex<ScreenManager>>)
{
    let mut screen_manager = screen_manager.lock().unwrap();
    let winapi_screen_manager: &mut WinApiScreenManager = match screen_manager.as_any().downcast_mut::<WinApiScreenManager>() {
        Some(win_api) => { win_api },
        None => panic!("")
    };

    let handle = get_current_handle(winapi_screen_manager);

    let cursor_info = CONSOLE_CURSOR_INFO
    {
        dwSize: 100,
        bVisible: if visable { TRUE } else {FALSE}
    };

    unsafe
    {
        SetConsoleCursorInfo(*handle, &cursor_info);
    }
}

/// Change the console text attribute.
pub fn set_console_text_attribute(value: u16, screen_manager: &Rc<Mutex<ScreenManager>>)
{
    let output_handle = get_output_handle();

    unsafe {
        SetConsoleTextAttribute(output_handle, value);
    }
}

/// Change console info.
pub fn set_console_info(absolute: bool, rect: &SMALL_RECT, screen_manager: &Rc<Mutex<ScreenManager>>) -> bool
{
    let mut screen_manager = screen_manager.lock().unwrap();
    let winapi_screen_manager: &mut WinApiScreenManager = match screen_manager.as_any().downcast_mut::<WinApiScreenManager>() {
        Some(win_api) => { win_api },
        None => panic!("")
    };

    let handle = get_current_handle(winapi_screen_manager);

    let absolute = match absolute {  true => 1, false => 0, };
    unsafe
    {
        let success = SetConsoleWindowInfo(*handle,absolute ,rect);
        is_true(success)
    }
}

/// Set the console screen buffer size
pub fn set_console_screen_buffer_size(  size: COORD, screen_manager: &Rc<Mutex<ScreenManager>>) -> bool
{
    let mut screen_manager = screen_manager.lock().unwrap();
    let winapi_screen_manager: &mut WinApiScreenManager = match screen_manager.as_any().downcast_mut::<WinApiScreenManager>() {
        Some(win_api) => { win_api },
        None => panic!("")
    };

    let handle = get_current_handle(winapi_screen_manager);

    unsafe
    {
        let success = SetConsoleScreenBufferSize(*handle, size);
        is_true(success)
    }
}

/// Fill a certain block with characters.
pub fn fill_console_output_character(cells_written: &mut u32, start_location: COORD, cells_to_write: u32, screen_manager: &Rc<Mutex<ScreenManager>>) -> bool
{
    let mut screen_manager = screen_manager.lock().unwrap();
    let winapi_screen_manager: &mut WinApiScreenManager = match screen_manager.as_any().downcast_mut::<WinApiScreenManager>() {
        Some(win_api) => { win_api },
        None => panic!("")
    };

    let handle = get_current_handle(winapi_screen_manager);

    unsafe {
        // fill the cells in console with blanks
        let success = FillConsoleOutputCharacterA (
            *handle,
            ' ' as i8,
            cells_to_write,
            start_location,
            cells_written,
        );
        is_true(success)
    }
}

/// Set console ouput attribute for certain block.
pub fn fill_console_output_attribute(cells_written: &mut u32, start_location: COORD, cells_to_write: u32,screen_manager: &Rc<Mutex<ScreenManager>>) -> bool
{

    // Get the position of the current console window
    let csbi = get_console_screen_buffer_info(screen_manager);

    let mut screen_manager = screen_manager.lock().unwrap();
    let winapi_screen_manager: &mut WinApiScreenManager = match screen_manager.as_any().downcast_mut::<WinApiScreenManager>() {
        Some(win_api) => { win_api },
        None => panic!("")
    };

    let handle = get_current_handle(winapi_screen_manager);

    let success;

    unsafe {
        success = FillConsoleOutputAttribute (
            *handle,
            csbi.wAttributes,
            cells_to_write,
            start_location,
            cells_written,
        );
    }

    is_true(success)
}

/// Create new console screen buffer. This can be used for alternate screen.
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

/// Set the active screen buffer to the given handle. This can be used for alternate screen.
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

/// Read the console outptut.
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

/// Write console output.
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

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::iter::once;

fn win32_string( value : &str ) -> Vec<u16> {
    OsStr::new( value ).encode_wide().chain( once( 0 ) ).collect()
}

//use std::os::raw::c_void;
use winapi::ctypes::c_void;
use std::mem::transmute;
use std::str;

/// Write utf8 buffer to console.
pub fn write_char_buffer(handle: HANDLE, buf: &[u8])
{
    // get string from u8[] and parse it to an c_str
    let mut utf8 = match str::from_utf8(buf)
        {
            Ok(string) => string,
            Err(_) => "123",
        };

    let utf16: Vec<u16> = utf8.encode_utf16().collect();
    let utf16_ptr: *const c_void = utf16.as_ptr() as *const _ as *const c_void;

    // get buffer info
    let csbi = get_console_screen_buffer_info_from_handle(&handle);

    // get current position
    let current_pos = COORD {X: csbi.dwCursorPosition.X, Y: csbi.dwCursorPosition.Y};

    let mut cells_written: u32 = 0;

    // write to console
    unsafe
    {
        WriteConsoleW(handle, utf16_ptr, utf16.len() as u32, &mut cells_written, NULL);
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
