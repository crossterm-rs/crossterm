//! Module that contains all the actions related to the terminal. like clearing, resizing and scrolling the terminal.

pub mod terminal;

#[cfg(target_os = "windows")]
mod winapi_terminal;
mod ansi_terminal;

#[cfg(target_os = "windows")]
use self::winapi_terminal::WinApiTerminal;
use self::ansi_terminal::AnsiTerminal;

pub use self::terminal::Terminal;
use super::{ ScreenManager, functions };

/// Enum that specifies a way of clearing.
pub enum ClearType {
    All,
    FromCursorDown,
    FromCursorUp,
    CurrentLine,
    UntilNewLine,
}

/// This trait defines the actions that can be preformed with the terminal color.
/// This trait can be implemented so that an concrete implementation of the ITerminalColor can forfill
/// the wishes to work on an specific platform.
///
/// ## For example:
///
/// This trait is implemented for `WINAPI` (Windows specific) and `ANSI` (Unix specific),
/// so that color related actions can be preformed on both unix and windows systems.
pub trait ITerminal {
    /// Clear the current cursor by specifying the clear type
    fn clear(&self, clear_type: ClearType, screen_manager: &ScreenManager);
    /// Get the terminal size (x,y)
    fn terminal_size(&self, screen_manager: &ScreenManager) -> (u16, u16);
    /// Scroll `n` lines up in the current terminal.
    fn scroll_up(&self, count: i16, screen_manager: &ScreenManager);
    /// Scroll `n` lines down in the current terminal.
    fn scroll_down(&self, count: i16,screen_manager: &ScreenManager);
    /// Resize terminal to the given width and height.
    fn set_size(&self, width: i16, height: i16, screen_manager: &ScreenManager);
    /// Close the current terminal
    fn exit(&self);
}
