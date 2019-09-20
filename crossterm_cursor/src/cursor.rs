//! A module that contains all the actions related to cursor movement in the terminal.
//! Like: moving the cursor position; saving and resetting the cursor position; hiding showing and control
//! the blinking of the cursor.
//!
//! Note that positions of the cursor are 0 -based witch means that the coordinates (cells) starts counting from 0

use crossterm_utils::Result;

pub use self::cursor::{
    cursor, BlinkOff, BlinkOn, Down, Goto, Hide, Left, ResetPos, Right, SavePos, Show,
    TerminalCursor, Up,
};

mod cursor;

mod ansi_cursor;
#[cfg(windows)]
mod winapi_cursor;

///! This trait defines the actions that can be performed with the terminal cursor.
///! This trait can be implemented so that a concrete implementation of the ITerminalCursor can fulfill
///! the wishes to work on a specific platform.
///!
///! ## For example:
///!
///! This trait is implemented for `WinApi` (Windows specific) and `ANSI` (Unix specific),
///! so that cursor related actions can be performed on both UNIX and Windows systems.
trait ITerminalCursor: Sync + Send {
    /// Goto location (`x`, `y`) in the current terminal window.
    fn goto(&self, x: u16, y: u16) -> Result<()>;
    /// Get the cursor location `(x, y)` in the current terminal window.
    fn pos(&self) -> Result<(u16, u16)>;
    /// Move cursor `n` times up
    fn move_up(&self, count: u16) -> Result<()>;
    /// Move the cursor `n` times to the right.
    fn move_right(&self, count: u16) -> Result<()>;
    /// Move the cursor `n` times down.
    fn move_down(&self, count: u16) -> Result<()>;
    /// Move the cursor `n` times left.
    fn move_left(&self, count: u16) -> Result<()>;
    /// Save cursor position so that its saved position can be recalled later. Note that this position
    /// is stored program based not per instance of the cursor struct.
    fn save_position(&self) -> Result<()>;
    /// Return to saved cursor position
    fn restore_position(&self) -> Result<()>;
    /// Hide the terminal cursor.
    fn hide(&self) -> Result<()>;
    /// Show the terminal cursor
    fn show(&self) -> Result<()>;
    /// Enable or disable the blinking of the cursor.
    fn blink(&self, blink: bool) -> Result<()>;
}
