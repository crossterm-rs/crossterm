//! With this module you can perform actions that are terminal related.
//! Like clearing and scrolling in the terminal or getting the size of the terminal.

use super::*;
use Construct;
use super::super::shared::terminal;

/// Struct that stores an specific platform implementation for terminal related actions.
pub struct Terminal<'terminal> {
    terminal: Option<Box<ITerminal>>,
    term: &'terminal terminal::Terminal
}

impl<'terminal>  Terminal<'terminal> {
    /// Create new terminal instance whereon terminal related actions can be performed.
    pub fn new(term: &'terminal  terminal::Terminal) -> Terminal<'terminal> {
        #[cfg(target_os = "windows")]
        let terminal = functions::get_module::<Box<ITerminal>>(WinApiTerminal::new(), AnsiTerminal::new());

        #[cfg(not(target_os = "windows"))]
        let terminal = Some(AnsiTerminal::new() as Box<ITerminal>);

        Terminal { terminal, term }

    }

    /// Clear the current cursor by specifying the clear type
    /// 
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    /// use crossterm::terminal;
    ///
    /// let mut term = terminal::terminal();
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
    /// 
    /// ```
    pub fn clear(&mut self, clear_type: ClearType) {
        if let Some(ref terminal) = self.terminal {
            terminal.clear(clear_type, &self.term);
        }
    }

    /// Get the terminal size (x,y).
    /// 
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    /// use crossterm::terminal;
    ///
    /// let mut term = terminal::terminal();
    ///
    /// let size = term.terminal_size();
    /// println!("{:?}", size);
    /// 
    /// ```
    pub fn terminal_size(&mut self) -> (u16, u16) {
        if let Some(ref terminal) = self.terminal {
            return terminal.terminal_size(&self.term)
        }
        (0,0)
    }

    /// Scroll `n` lines up in the current terminal.
    /// 
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    /// use crossterm::terminal;
    ///
    /// let mut term = terminal::terminal();
    /// 
    /// // scroll up by 5 lines
    /// let size = term.scroll_up(5);
    /// 
    /// ```
    pub fn scroll_up(&mut self, count: i16) {
        if let Some(ref terminal) = self.terminal {
            terminal.scroll_up(count,&self.term);
        }
    }

    /// Scroll `n` lines up in the current terminal.
    /// 
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    /// use crossterm::terminal;
    ///
    /// let mut term = terminal::terminal();
    /// 
    /// // scroll down by 5 lines
    /// let size = term.scroll_down(5);
    /// 
    /// ```
    pub fn scroll_down(&mut self, count: i16) {
        if let Some(ref terminal) = self.terminal {
            terminal.scroll_down(count, &self.term);
        }
    }

    /// Set the terminal size. Note that not all terminals can be set to a very small scale.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    /// use crossterm::terminal;
    ///
    /// let mut term = terminal::terminal();
    /// 
    /// // Set of the size to X: 10 and Y: 10
    /// let size = term.set_size(10,10);
    /// 
    /// ```
    pub fn set_size(&mut self, width: i16, height: i16)
    {
        if let Some (ref terminal) = self.terminal {
            terminal.set_size(width,height,&self.term);
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
/// use crossterm::terminal;
///
/// let mut term = terminal::terminal();
///
/// // scroll down by 5 lines
/// let size = term.scroll_down(5);
///
/// ```
///
///

pub fn terminal<'terminal>(terminal: &'terminal terminal::Terminal) -> Box<Terminal<'terminal>> {
    Box::from(Terminal::new(&terminal))
}
