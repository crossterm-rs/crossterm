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
    /// Instantiate an color implementation whereon color related actions can be performed.
    pub fn init(&mut self) {
        if let None = self.terminal {
            self.terminal = get_terminal();
        }
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
    /// let mut term = crossterm_terminal::get();
    /// 
    /// // clear all cells in terminal.
    /// term.clear(crossterm_terminal::ClearType::All);
    /// //clear all cells from the cursor position downwards in terminal.
    /// term.clear(crossterm_terminal::ClearType::FromCursorDown);
    /// //clear all cells from the cursor position upwards in terminal.
    /// term.clear(crossterm_terminal::ClearType::FromCursorUp);
    /// // clear current line cells in terminal.
    /// term.clear(crossterm_terminal::ClearType::CurrentLine);
    /// // clear all cells from cursor position until new line in terminal.
    /// term.clear(crossterm_terminal::ClearType::UntilNewLine);
    /// 
    /// ```
    pub fn clear(&mut self, clear_type: ClearType) {
        &self.init();
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
    /// let mut term = crossterm_terminal::get();
    /// 
    /// let size = term.terminal_size();
    /// println!("{:?}", size);
    /// 
    /// ```
    pub fn terminal_size(&mut self) -> Option<(u16, u16)> {
        &self.init();
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
    /// let mut term = crossterm_terminal::get();
    /// 
    /// // scroll up by 5 lines
    /// let size = term.scroll_up(5);
    /// 
    /// ```
    pub fn scroll_up(&mut self, count: i16) {
        &self.init();
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
    /// let mut term = crossterm_terminal::get();
    /// 
    /// // scroll down by 5 lines
    /// let size = term.scroll_down(5);
    /// 
    /// ```
    pub fn scroll_down(&mut self, count: i16) {
        &self.init();
        if let Some(ref terminal) = self.terminal {
            terminal.scroll_down(count);
        }
    }
}

/// Get the concrete ITerminal implementation based on the current operating system.
fn get_terminal() -> Option<Box<ITerminal>> {
    #[cfg(unix)]
    return Some(UnixTerminal::new());
    #[cfg(windows)]
    return Some(WinApiTerminal::new());
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
/// let mut term = crossterm_terminal::get();
/// 
/// // scroll down by 5 lines
/// let size = term.scroll_down(5);
/// 
/// ```
pub fn get() -> Box<Terminal> {
    Box::from(Terminal {
        terminal: get_terminal(),
    })
}
