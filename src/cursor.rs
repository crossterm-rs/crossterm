//! With this module you can perform actions that are cursor related.
//! Like changing and display the position of the cursor in terminal.
//!
//! Note that positions of the cursor are 0 -based witch means that the coordinates (cells) starts counting from 0

use super::*;

use modules::cursor::AnsiCursor;

use std::fmt::Display;
use std::io::Write;

use std::sync::Arc;

/// Struct that stores an specific platform implementation for cursor related actions.
///
/// Check `/examples/cursor` in the library for more specific examples.
///
/// ```rust
/// extern crate crossterm;
/// use self::crossterm::cursor::cursor;
/// use self::crossterm::Screen;
///
/// let screen = Screen::default();
/// let mut cursor = cursor(&screen);
///
/// // Get cursor and goto pos X: 5, Y: 10
/// cursor.goto(5,10);
///
/// cursor.show();
/// cursor.hide();
/// cursor.blink(true);
/// cursor.move_left(2);
/// ```

pub type TerminalCursor = Box<ITerminalCursor>;

pub fn cursor() -> TerminalCursor {
    #[cfg(target_os = "windows")]
    let r = functions::get_module::<Box<ITerminalCursor>>(WinApiCursor::new(), AnsiCursor::new())
        .unwrap();

    #[cfg(not(target_os = "windows"))]
    let r = AnsiCursor::new() as Box<ITerminalCursor>;
    r
}

