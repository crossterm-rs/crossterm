//! Module that contains all the actions related to the terminal.
//!
//! We can think of:
//! - alternate screen
//! - raw mode
//! - clearing resizing scrolling the terminal.

use {Context, ScreenManager, Terminal};
use self::ansi_terminal::AnsiTerminal;
pub use self::terminal::{ terminal};
#[cfg(target_os = "windows")]
use self::winapi_terminal::WinApiTerminal;
use std::cell::RefCell;
use std::sync::Mutex;

pub mod terminal;

#[cfg(target_os = "windows")]
mod winapi_terminal;
mod ansi_terminal;

pub mod screen;
mod raw;

/// Enum that can be used for the kind of clearing that can be done in the terminal.
pub enum ClearType {
    All,
    FromCursorDown,
    FromCursorUp,
    CurrentLine,
    UntilNewLine,
}

///! This trait defines the actions that can be preformed with the terminal.
///! This trait can be implemented so that an concrete implementation of the ITerminal can forfill
///! the wishes to work on an specific platform.
///!
///! ## For example:
///!
///! This trait is implemented for `WINAPI` (Windows specific) and `ANSI` (Unix specific),
///! so that cursor related actions can be preformed on both unix and windows systems.
pub trait ITerminal {
    /// Clear the current cursor by specifying the clear type
    fn clear(&self, clear_type: ClearType, terminal: &Terminal);
    /// Get the terminal size (x,y)
    fn terminal_size(&self, terminal: &Terminal) -> (u16, u16);
    /// Scroll `n` lines up in the current terminal.
    fn scroll_up(&self, count: i16, terminal: &Terminal);
    /// Scroll `n` lines down in the current terminal.
    fn scroll_down(&self, count: i16, terminal: &Terminal);
    /// Resize terminal to the given width and height.
    fn set_size(&self,width: i16, height: i16, terminal: &Terminal);
}
