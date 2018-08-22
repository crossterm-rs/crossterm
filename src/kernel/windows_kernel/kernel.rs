//! This module contains some basic winapi calls.

use winapi::um::consoleapi::{GetConsoleMode, SetConsoleMode};
use winapi::um::wincon::{
    GetLargestConsoleWindowSize, SetConsoleTextAttribute, SetConsoleWindowInfo, COORD, SMALL_RECT,
};

use super::*;
use std::sync::Arc;

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
pub fn set_console_text_attribute(value: u16) -> bool {
    let handle = handle::get_current_handle().unwrap();

    unsafe {
        return is_true(SetConsoleTextAttribute(handle, value));
    }
}

/// Change console info.
pub fn set_console_info(absolute: bool, rect: &SMALL_RECT) -> bool {
    let handle = handle::get_current_handle().unwrap();

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
