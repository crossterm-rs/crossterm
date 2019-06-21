//! A module that contains all the actions related to the terminal.
//! Like clearing and scrolling in the terminal or getting the window size from the terminal.

use super::{AnsiTerminal, ClearType, ITerminal};
use crossterm_utils::Result;

#[cfg(windows)]
use super::WinApiTerminal;
#[cfg(windows)]
use crossterm_utils::supports_ansi;

use std::fmt;
use std::io::Write;

/// Allows you to preform actions on the terminal.
///
/// # Features:
///
/// - Clearing (all lines, current line, from cursor down and up, until new line)
/// - Scrolling (Up, down)
/// - Get the size of the terminal
/// - Set the size of the terminal
/// - Alternate screen
/// - Raw screen
/// - Exit the current process
///
/// Check `/examples/` in the library for more specific examples.
pub struct Terminal {
    #[cfg(windows)]
    terminal: Box<(dyn ITerminal + Sync + Send)>,
    #[cfg(unix)]
    terminal: AnsiTerminal,
}

impl Terminal {
    /// Create new terminal instance whereon terminal related actions can be performed.
    pub fn new() -> Terminal {
        #[cfg(windows)]
        let terminal = if supports_ansi() {
            Box::from(AnsiTerminal::new()) as Box<(dyn ITerminal + Sync + Send)>
        } else {
            WinApiTerminal::new() as Box<(dyn ITerminal + Sync + Send)>
        };

        #[cfg(unix)]
        let terminal = AnsiTerminal::new();

        Terminal { terminal }
    }

    /// Clear the current cursor by specifying the `ClearType`.
    ///
    /// # Example
    /// ```rust
    /// let mut term = terminal();
    ///
    /// // clear all cells in terminal.
    /// term.clear(terminal::ClearType::All);
    /// // clear all cells from the cursor position downwards in terminal.
    /// term.clear(terminal::ClearType::FromCursorDown);
    /// // clear all cells from the cursor position upwards in terminal.
    /// term.clear(terminal::ClearType::FromCursorUp);
    /// // clear current line cells in terminal.
    /// term.clear(terminal::ClearType::CurrentLine);
    /// // clear all cells from cursor position until new line in terminal.
    /// term.clear(terminal::ClearType::UntilNewLine);
    /// ```
    pub fn clear(&self, clear_type: ClearType) -> Result<()> {
        self.terminal.clear(clear_type)
    }

    /// Get the terminal size (x,y).
    ///
    /// # Remark
    /// This will return a tuple of (x: u16, y: u16)
    pub fn terminal_size(&self) -> (u16, u16) {
        self.terminal.terminal_size()
    }

    /// Scroll `n` lines up in the current terminal.
    ///
    /// # Parameter
    /// - `count`: the number of rows should be shifted up.
    pub fn scroll_up(&self, count: i16) -> Result<()> {
        self.terminal.scroll_up(count)
    }

    /// Scroll `n` lines down in the current terminal.
    ///
    /// # Parameter
    /// - `count`: the number of rows should be shifted down.
    pub fn scroll_down(&self, count: i16) -> Result<()> {
        self.terminal.scroll_down(count)
    }

    /// Set the terminal size. Note that not all terminals can be set to a very small scale.
    ///
    /// ```rust
    /// let mut term = terminal();
    ///
    /// // Set of the size to X: 10 and Y: 10
    /// let size = term.set_size(10,10);
    /// ```
    pub fn set_size(&self, width: i16, height: i16) -> Result<()> {
        self.terminal.set_size(width, height)
    }

    /// Exit the current process.
    ///
    /// ```rust
    /// let mut term = terminal();
    ///
    /// let size = term.exit();
    /// ```
    pub fn exit(&self) {
        crate::sys::exit();
    }

    /// Write any displayable content to the current terminal screen.
    ///
    /// ```rust
    /// let mut term = terminal();
    ///
    /// let size = term.write("Some text \n Some text on new line");
    /// ```
    ///
    /// This will also flush the standard output.
    pub fn write<D: fmt::Display>(&self, value: D) -> Result<usize> {
        write_cout!(format!("{}", value))?;
        Ok(0)
    }
}

/// Get a `Terminal` instance whereon terminal related actions can be performed.
pub fn terminal() -> Terminal {
    Terminal::new()
}
