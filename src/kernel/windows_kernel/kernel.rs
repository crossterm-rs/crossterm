//! This module is the core of all the `WINAPI` actions. All unsafe `WINAPI` function call are done here.
//! I am planing to refactor this a little since a lot of code could be handled safer.

use std::rc::Rc;
use Context;

use winapi::shared::minwindef::{FALSE, TRUE};
use winapi::shared::ntdef::NULL;
use winapi::um::consoleapi::WriteConsoleW;
use winapi::um::consoleapi::{GetConsoleMode, SetConsoleMode};
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::processenv::GetStdHandle;
use winapi::um::winbase::{STD_INPUT_HANDLE, STD_OUTPUT_HANDLE};
use winapi::um::wincon;
use winapi::um::wincon::{
    CreateConsoleScreenBuffer, FillConsoleOutputAttribute, FillConsoleOutputCharacterA,
    GetConsoleScreenBufferInfo, GetLargestConsoleWindowSize, SetConsoleActiveScreenBuffer,
    SetConsoleCursorInfo, SetConsoleCursorPosition, SetConsoleScreenBufferSize,
    SetConsoleTextAttribute, SetConsoleWindowInfo, WriteConsoleOutputAttribute,
    WriteConsoleOutputCharacterA, WriteConsoleOutputCharacterW, WriteConsoleOutputW, CHAR_INFO,
    CONSOLE_CURSOR_INFO, CONSOLE_SCREEN_BUFFER_INFO, COORD, ENABLE_PROCESSED_INPUT, PSMALL_RECT,
    SMALL_RECT,
};
use winapi::um::winnt::HANDLE;

use super::Empty;
static mut CONSOLE_OUTPUT_HANDLE: Option<HANDLE> = None;
static mut CONSOLE_INPUT_HANDLE: Option<HANDLE> = None;

use super::super::super::manager::{ScreenManager, WinApiScreenManager};
use std::sync::Mutex;

/// Get the global stored handle.
pub fn get_current_handle(screen_manager: &Rc<Mutex<ScreenManager>>) -> HANDLE {
    let mut mx_guard = screen_manager;

    let handle: HANDLE;

    let mut screen_manager = mx_guard.lock().unwrap();
    {
        let winapi_screen_manager: &mut WinApiScreenManager = match screen_manager
            .as_any()
            .downcast_mut::<WinApiScreenManager>()
            {
                Some(win_api) => win_api,
                None => panic!(""),
            };

        handle = *winapi_screen_manager.get_handle();
    }

    return handle;
}

/// Get the std_output_handle of the console
pub fn get_output_handle() -> HANDLE {
    unsafe {
        if let Some(handle) = CONSOLE_OUTPUT_HANDLE {
            handle
        } else {
            let handle = GetStdHandle(STD_OUTPUT_HANDLE);

            if !is_valid_handle(&handle) {
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

            if !is_valid_handle(&handle) {
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
pub fn get_console_screen_buffer_info(
    screen_manager: &Rc<Mutex<ScreenManager>>,
) -> CONSOLE_SCREEN_BUFFER_INFO {

    let mut csbi = CONSOLE_SCREEN_BUFFER_INFO::empty();
    let success;

    unsafe { success = GetConsoleScreenBufferInfo(get_current_handle(screen_manager), &mut csbi) }

    if success == 0 {
        panic!("Cannot get console screen buffer info");
    }

    csbi
}

/// Create a new console screen buffer info struct.
pub fn get_std_console_screen_buffer_info() -> CONSOLE_SCREEN_BUFFER_INFO {

    let mut csbi = CONSOLE_SCREEN_BUFFER_INFO::empty();
    let success;

    unsafe { success = GetConsoleScreenBufferInfo(get_output_handle(), &mut csbi) }

    if success == 0 {
        panic!("Cannot get console screen buffer info");
    }

    csbi
}

/// Get buffer info and handle of the current screen.
pub fn get_buffer_info_and_hande(screen_manager: &Rc<Mutex<ScreenManager>>) -> (CONSOLE_SCREEN_BUFFER_INFO, HANDLE)
{
   let handle = get_current_handle(screen_manager);
    let csbi = get_console_screen_buffer_info_from_handle(&handle);

    return (csbi, handle)
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

/// Get the largest console window size possible.
pub fn get_largest_console_window_size() -> COORD {
    let output_handle = get_output_handle();

    unsafe { GetLargestConsoleWindowSize(output_handle) }
}

/// Get the original color of the terminal.
pub fn get_original_console_color(screen_manager: &Rc<Mutex<ScreenManager>>) -> u16 {
    let console_buffer_info = get_console_screen_buffer_info(screen_manager);
    console_buffer_info.wAttributes as u16
}

/// Set the console mode to the given console mode.
pub fn set_console_mode(handle: &HANDLE, console_mode: u32) -> bool {
    unsafe {
        let success = SetConsoleMode(*handle, console_mode);
        return is_true(success);
    }
}

/// Get the console mode.
pub fn get_console_mode(handle: &HANDLE, current_mode: &mut u32) -> bool {
    unsafe {
        let success = GetConsoleMode(*handle, &mut *current_mode);
        return is_true(success);
    }
}

/// Set the cursor position to the given x and y. Note that this is 0 based.
pub fn set_console_cursor_position(x: i16, y: i16, screen_manager: &Rc<Mutex<ScreenManager>>) {
    if x < 0 || x >= <i16>::max_value() {
        panic!("X: {}, Argument Out of Range Exception", x);
    }

    if y < 0 || y >= <i16>::max_value() {
        panic!("Y: {}, Argument Out of Range Exception", y);
    }

    let handle = get_current_handle(screen_manager);

    let position = COORD { X: x, Y: y };

    unsafe {
        let success = SetConsoleCursorPosition(handle, position);

        if success == 0 {
            panic!("Argument out of range.");
        }
    }
}

/// change the cursor visibility.
pub fn cursor_visibility(visable: bool, screen_manager: &Rc<Mutex<ScreenManager>>) {
    let handle = get_current_handle(screen_manager);

    let cursor_info = CONSOLE_CURSOR_INFO {
        dwSize: 100,
        bVisible: if visable { TRUE } else { FALSE },
    };

    unsafe {
        SetConsoleCursorInfo(handle, &cursor_info);
    }
}

/// Change the console text attribute.
pub fn set_console_text_attribute(value: u16, screen_manager: &Rc<Mutex<ScreenManager>>) {
    let handle = get_current_handle(screen_manager);

    unsafe {
        SetConsoleTextAttribute(handle, value);
    }
}

/// Change console info.
pub fn set_console_info(
    absolute: bool,
    rect: &SMALL_RECT,
    screen_manager: &Rc<Mutex<ScreenManager>>,
) -> bool {
    let handle = get_current_handle(screen_manager);

    let absolute = match absolute {
        true => 1,
        false => 0,
    };
    unsafe {
        let success = SetConsoleWindowInfo(handle, absolute, rect);
        is_true(success)
    }
}

/// Set the console screen buffer size
pub fn set_console_screen_buffer_size(
    size: COORD,
    screen_manager: &Rc<Mutex<ScreenManager>>,
) -> bool {

    let handle = get_current_handle(screen_manager);

    unsafe {
        let success = SetConsoleScreenBufferSize(handle, size);
        is_true(success)
    }
}



/// Create new console screen buffer. This can be used for alternate screen.
pub fn create_console_screen_buffer() -> HANDLE {
    use std::mem::size_of;
    use winapi::shared::ntdef::NULL;
    use winapi::um::minwinbase::SECURITY_ATTRIBUTES;
    use winapi::um::wincon::CONSOLE_TEXTMODE_BUFFER;
    use winapi::um::winnt::{FILE_SHARE_READ, FILE_SHARE_WRITE, GENERIC_READ, GENERIC_WRITE};

    unsafe {
        let mut security_attr: SECURITY_ATTRIBUTES = SECURITY_ATTRIBUTES {
            nLength: size_of::<SECURITY_ATTRIBUTES>() as u32,
            lpSecurityDescriptor: NULL,
            bInheritHandle: TRUE,
        };

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
pub fn set_active_screen_buffer(new_buffer: HANDLE) {
    unsafe {
        if !is_true(SetConsoleActiveScreenBuffer(new_buffer)) {
            panic!("Cannot set active screen buffer");
        }
    }
}


/// Parse integer to an bool
pub fn is_true(value: i32) -> bool {
    if value == 0 {
        return false;
    } else {
        return true;
    }
}
