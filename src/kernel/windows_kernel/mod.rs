//! This module contains the `windows` (unsafe) logic.

#[allow(unused)]
mod reading;
pub mod ansi_support;
mod cursor;
pub mod writing;

pub use self::cursor::Cursor;
pub use crossterm_winapi::{
    Console, ConsoleMode, Coord, Handle, HandleType, ScreenBuffer, ScreenBufferInfo, Size,
    WindowPositions,
};

/// Exit the current process.
pub fn exit() {
    ::std::process::exit(256);
}