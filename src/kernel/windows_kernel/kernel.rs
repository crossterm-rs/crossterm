//! This module is the core of all the `WINAPI` actions. All unsafe `WINAPI` function call are done here.
//! I am planing to refactor this a little since a lot of code could be handled safer.

use winapi::um::consoleapi::{GetConsoleMode, SetConsoleMode};
use winapi::um::winnt::HANDLE;
use winapi::um::wincon::{
    SetConsoleTextAttribute, SetConsoleWindowInfo, GetLargestConsoleWindowSize,
    COORD, SMALL_RECT
};

use super::{handle, Empty};
use super::super::super::manager::ScreenManager;

use std::io::{ErrorKind, Result};
use std::sync::Mutex;
use std::rc::Rc;

/// Get the largest console window size possible.
pub fn get_largest_console_window_size() -> COORD {
    let output_handle = handle::get_output_handle().unwrap();

    unsafe { GetLargestConsoleWindowSize(output_handle) }
}

/// Set the console mode to the given console mode.
pub fn set_console_mode(handle: &HANDLE, console_mode: u32) -> bool {
    unsafe {
        return is_true(SetConsoleMode(*handle, console_mode));
    }
}

/// Get the console mode.
pub fn get_console_mode(handle: &HANDLE, current_mode: &mut u32) -> bool {
    unsafe {
        return is_true(GetConsoleMode(*handle, &mut *current_mode));
    }
}

/// Change the console text attribute.
pub fn set_console_text_attribute(value: u16, screen_manager: &Rc<Mutex<ScreenManager>>) -> bool {
    let handle = handle::get_current_handle(screen_manager).unwrap();

    unsafe {
        return is_true(SetConsoleTextAttribute(handle, value));
    }
}

/// Change console info.
pub fn set_console_info(
    absolute: bool,
    rect: &SMALL_RECT,
    screen_manager: &Rc<Mutex<ScreenManager>>,
) -> bool {
    let handle = handle::get_current_handle(screen_manager).unwrap();

    let absolute = match absolute {
        true => 1,
        false => 0,
    };
    unsafe {
        return is_true(SetConsoleWindowInfo(handle, absolute, rect));
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

///// Get the original color of the terminal.
//pub fn get_original_console_color(screen_manager: &Rc<Mutex<ScreenManager>>) -> u16 {
//    let console_buffer_info = csbi::get_console_screen_buffer_info(screen_manager);
//    console_buffer_info.wAttributes as u16
//}
