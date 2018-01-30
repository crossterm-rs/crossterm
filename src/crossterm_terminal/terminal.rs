//! With this module you can perform actions that are terminal related.
//! Like clearing and scrolling in the terminal or getting the size of the terminal.

use Construct;
use super::base_terminal::{ClearType, ITerminal};

#[cfg(unix)]
use super::UnixTerminal;
#[cfg(windows)]
use super::WinApiTerminal;

/// Struct that stores an specific platform implementation for terminal related actions.
pub struct Terminal {
    terminal: Option<Box<ITerminal>>,
}

impl Terminal {
    /// Create new terminal instance whereon terminal related actions can be performed.
    pub fn new() -> Terminal {
        let term: Option<Box<ITerminal>> =
        {
            #[cfg(unix)]
            Some(UnixTerminal::new());
            #[cfg(windows)]
            Some(WinApiTerminal::new())
        };

        Terminal { terminal: term }
    }

    /// Clear the current cursor by specifying the clear type
    /// 
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    /// use crossterm::crossterm_terminal;
    ///
    /// let mut term = crossterm_terminal::terminal();
    /// 
    /// // clear all cells in terminal.
    /// term.clear(crossterm_terminal::ClearType::All);
    /// // clear all cells from the cursor position downwards in terminal.
    /// term.clear(crossterm_terminal::ClearType::FromCursorDown);
    /// // clear all cells from the cursor position upwards in terminal.
    /// term.clear(crossterm_terminal::ClearType::FromCursorUp);
    /// // clear current line cells in terminal.
    /// term.clear(crossterm_terminal::ClearType::CurrentLine);
    /// // clear all cells from cursor position until new line in terminal.
    /// term.clear(crossterm_terminal::ClearType::UntilNewLine);
    /// 
    /// ```
    pub fn clear(&mut self, clear_type: ClearType) {
        if let Some(ref terminal) = self.terminal {
            terminal.clear(clear_type);
        }
    }

    /// Get the terminal size (x,y).
    /// 
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    /// use crossterm::crossterm_terminal;
    ///
    /// let mut term = crossterm_terminal::terminal();
    /// 
    /// let size = term.terminal_size();
    /// println!("{:?}", size);
    /// 
    /// ```
    pub fn terminal_size(&mut self) -> Option<(u16, u16)> {
        if let Some(ref terminal) = self.terminal {
            let a = terminal.terminal_size();
            a
        } else {
            None
        }
    }

    /// Scroll `n` lines up in the current terminal.
    /// 
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    /// use crossterm::crossterm_terminal;
    ///
    /// let mut term = crossterm_terminal::terminal();
    /// 
    /// // scroll up by 5 lines
    /// let size = term.scroll_up(5);
    /// 
    /// ```
    pub fn scroll_up(&mut self, count: i16) {
        if let Some(ref terminal) = self.terminal {
            terminal.scroll_up(count);
        }
    }

    /// Scroll `n` lines up in the current terminal.
    /// 
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    /// use crossterm::crossterm_terminal;
    ///
    /// let mut term = crossterm_terminal::terminal();
    /// 
    /// // scroll down by 5 lines
    /// let size = term.scroll_down(5);
    /// 
    /// ```
    pub fn scroll_down(&mut self, count: i16) {
        if let Some(ref terminal) = self.terminal {
            terminal.scroll_down(count);
        }
    }

    /// Set the terminal size. Note that not all terminals can be set to a very small scale.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    /// use crossterm::crossterm_terminal;
    ///
    /// let mut term = crossterm_terminal::terminal();
    /// 
    /// // Set of the size to X: 10 and Y: 10
    /// let size = term.set_size(10,10);
    /// 
    /// ```
    pub fn set_size(&mut self, width: i16, height: i16)
    {
        if let Some (ref terminal) = self.terminal {
            terminal.set_size(width,height);
        }
    }
}

/// Get an Terminal implementation whereon terminal related actions can be performed.
///
/// Check `/examples/terminal` in the libary for more spesific examples.
///
/// #Example
///
/// ```rust
///
/// extern crate crossterm;
/// use crossterm::crossterm_terminal;
///
/// let mut term = crossterm_terminal::terminal();
///
/// // scroll down by 5 lines
/// let size = term.scroll_down(5);
///
/// ```
///
pub fn terminal() -> Box<Terminal> {
    Box::from(Terminal::new())
}
