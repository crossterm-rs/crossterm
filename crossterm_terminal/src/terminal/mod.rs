//! A module that contains all the actions related to the terminal. like clearing, resizing, pausing and scrolling the terminal.
#[cfg(test)]
mod test;

mod terminal;

mod ansi_terminal;
#[cfg(windows)]
mod winapi_terminal;

use self::ansi_terminal::AnsiTerminal;
#[cfg(windows)]
use self::winapi_terminal::WinApiTerminal;

pub use self::terminal::{terminal, Clear, ScrollDown, ScrollUp, SetSize, Terminal};

use crossterm_utils::Result;

/// Enum that specifies ways of clearing the terminal.
#[derive(Clone)]
pub enum ClearType {
    /// clear all cells in terminal.
    All,
    /// clear all cells from the cursor position downwards in terminal.
    FromCursorDown,
    /// clear all cells from the cursor position upwards in terminal.
    FromCursorUp,
    /// clear current line cells in terminal.
    CurrentLine,
    /// clear all cells from cursor position until new line in terminal.
    UntilNewLine,
}

/// This trait defines the actions that can be performed with the terminal color.
/// This trait can be implemented so that an concrete implementation of the ITerminalColor can fulfill.
/// the wishes to work on an specific platform.
///
/// ## For example:
///
/// This trait is implemented for `WinApi` (Windows specific) and `ANSI` (Unix specific),
/// so that terminal related actions can be performed on both Unix and Windows systems.
trait ITerminal {
    /// Clear the current cursor by specifying the clear type
    fn clear(&self, clear_type: ClearType) -> Result<()>;
    /// Get the terminal size (x,y)
    fn terminal_size(&self) -> (u16, u16);
    /// Scroll `n` lines up in the current terminal.
    fn scroll_up(&self, count: i16) -> Result<()>;
    /// Scroll `n` lines down in the current terminal.
    fn scroll_down(&self, count: i16) -> Result<()>;
    /// Resize terminal to the given width and height.
    fn set_size(&self, width: i16, height: i16) -> Result<()>;
}
