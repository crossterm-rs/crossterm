//! This module provides a type that represents some rectangle.
//! For example, in WinAPi we have `SMALL_RECT` to represent a window size but this is a little inconvenient.
//! This module provides some trait implementations who will make parsing and working whit `COORD` easier.

use winapi::um::wincon::{CONSOLE_SCREEN_BUFFER_INFO, SMALL_RECT};

/// This is a wrapper for the locations of a rectangle.
///
/// It has left, right, bottom, top attributes.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct WindowPositions {
    pub left: i16,
    pub right: i16,
    pub bottom: i16,
    pub top: i16,
}

impl From<CONSOLE_SCREEN_BUFFER_INFO> for WindowPositions {
    fn from(csbi: CONSOLE_SCREEN_BUFFER_INFO) -> Self {
        WindowPositions {
            left: csbi.srWindow.Left,
            right: csbi.srWindow.Right,
            bottom: csbi.srWindow.Bottom,
            top: csbi.srWindow.Top,
        }
    }
}

impl From<WindowPositions> for SMALL_RECT {
    fn from(positions: WindowPositions) -> Self {
        SMALL_RECT {
            Top: positions.top,
            Right: positions.right,
            Bottom: positions.bottom,
            Left: positions.left,
        }
    }
}
