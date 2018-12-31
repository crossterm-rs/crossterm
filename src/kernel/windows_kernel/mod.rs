//! This module contains the `windows` (unsafe) logic.

pub mod ansi_support;
mod cursor;
pub mod reading;
pub mod writing;

use winapi::um::{
    wincon::{CONSOLE_SCREEN_BUFFER_INFO, COORD, SMALL_RECT},
    winnt::HANDLE,
};

pub use self::cursor::Cursor;
pub use crossterm_winapi::{
    Console, ConsoleMode, Coord, Handle, HandleType, ScreenBuffer, ScreenBufferInfo, Size,
    WindowPositions,
};

/// Exit the current process.
pub fn exit() {
    ::std::process::exit(256);
}
