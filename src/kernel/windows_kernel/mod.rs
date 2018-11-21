//! This module contains the `windows` (unsafe) logic.

pub mod ansi_support;
pub mod csbi;
pub mod cursor;
pub mod handle;
pub mod kernel;
pub mod terminal;
pub mod writing;

use winapi::um::wincon::{CONSOLE_SCREEN_BUFFER_INFO, COORD, SMALL_RECT};
use winapi::um::winnt::HANDLE;

use TerminalOutput;

use common::traits::Empty;

impl Empty for COORD {
    fn empty() -> COORD {
        COORD { X: 0, Y: 0 }
    }
}

impl Empty for SMALL_RECT {
    fn empty() -> SMALL_RECT {
        SMALL_RECT {
            Top: 0,
            Right: 0,
            Bottom: 0,
            Left: 0,
        }
    }
}

impl Empty for CONSOLE_SCREEN_BUFFER_INFO {
    fn empty() -> CONSOLE_SCREEN_BUFFER_INFO {
        CONSOLE_SCREEN_BUFFER_INFO {
            dwSize: COORD::empty(),
            dwCursorPosition: COORD::empty(),
            wAttributes: 0,
            srWindow: SMALL_RECT::empty(),
            dwMaximumWindowSize: COORD::empty(),
        }
    }
}
