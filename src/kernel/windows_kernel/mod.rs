//! This module contains the `windows` (unsafe) logic.

pub mod ansi_support;
mod cursor;
#[allow(unused)]
mod reading;
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
