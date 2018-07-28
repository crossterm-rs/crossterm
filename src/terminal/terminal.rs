//! With this module you can perform actions that are terminal related.
//! Like clearing and scrolling in the terminal or getting the size of the terminal.

use super::super::shared::functions;
use super::super::style;
use super::*;
use Context;

use std::fmt;
use std::io::Write;

use std::rc::Rc;

/// Struct that stores an specific platform implementation for terminal related actions.
pub struct Terminal<'terminal> {
    terminal: Box<ITerminal>,
    screen_manager: &'terminal ScreenManager,
}

impl<'terminal> Terminal<'terminal> {
    /// Create new terminal instance whereon terminal related actions can be performed.
    pub fn new(screen_manager: &'terminal ScreenManager) -> Terminal<'terminal> {
        #[cfg(target_os = "windows")]
        let terminal = functions::get_module::<Box<ITerminal>>(
            Box::new(WinApiTerminal::new()),
            Box::new(AnsiTerminal::new()),
        ).unwrap();

        #[cfg(not(target_os = "windows"))]
        let terminal = AnsiTerminal::new() as Box<ITerminal>;

        Terminal {
            terminal,
            screen_manager: screen_manager
        }
    }

    /// Clear the current cursor by specifying the clear type
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    /// use crossterm::terminal;
    /// use crossterm::Context;
    ///
    /// let context = Context::new();
    /// let mut term = terminal::terminal(&context);
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
    pub fn clear(&self, clear_type: ClearType) {
        self.terminal.clear(clear_type, &self.screen_manager);
    }

    /// Get the terminal size (x,y).
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    /// use crossterm::terminal;
    /// use crossterm::Context;
    ///
    /// let context = Context::new();
    /// let mut term = terminal::terminal(&context);
    ///
    /// let size = term.terminal_size();
    /// println!("{:?}", size);
    ///
    /// ```
    pub fn terminal_size(&self) -> (u16, u16) {
        return self.terminal.terminal_size(&self.screen_manager);
    }

    /// Scroll `n` lines up in the current terminal.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    /// use crossterm::terminal;
    /// use crossterm::Context;
    ///
    /// let context = Context::new();
    /// let mut term = terminal::terminal(&context);
    ///
    /// // scroll up by 5 lines
    /// let size = term.scroll_up(5);
    ///
    /// ```
    pub fn scroll_up(&self, count: i16) {
        self.terminal.scroll_up(count,&self.screen_manager);
    }

    /// Scroll `n` lines up in the current terminal.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    /// use crossterm::terminal;
    /// use crossterm::Context;
    ///
    /// let context = Context::new();
    /// let mut term = terminal::terminal(&context);
    ///
    /// // scroll down by 5 lines
    /// let size = term.scroll_down(5);
    ///
    /// ```
    pub fn scroll_down(&self, count: i16) {
        self.terminal.scroll_down(count,&self.screen_manager);
    }

    /// Set the terminal size. Note that not all terminals can be set to a very small scale.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    /// use crossterm::terminal;
    /// use crossterm::Context;
    ///
    /// let context = Context::new();
    /// let mut term = terminal::terminal(&context);
    ///
    /// // Set of the size to X: 10 and Y: 10
    /// let size = term.set_size(10,10);
    ///
    /// ```
    pub fn set_size(&self, width: i16, height: i16) {
        self.terminal.set_size(width, height,&self.screen_manager);
    }

    /// Wraps an displayable object so it can be formatted with colors and attributes.
    ///
    /// Check `/examples/color` in the libary for more spesific examples.
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate crossterm;
    ///
    /// use self::crossterm::style::{paint,Color};
    ///
    /// fn main()
    /// {
    ///     // Create an styledobject object from the text 'Unstyled font'
    ///     // Currently it has the default foregroundcolor and backgroundcolor.
    ///     println!("{}",paint("Unstyled font"));
    ///
    ///     // Create an displayable object from the text 'Colored font',
    ///     // Paint this with the `Red` foreground color and `Blue` backgroundcolor.
    ///     // Print the result.
    ///     let styledobject = paint("Colored font").with(Color::Red).on(Color::Blue);
    ///     println!("{}", styledobject);
    ///
    ///     // Or all in one line
    ///     println!("{}", paint("Colored font").with(Color::Red).on(Color::Blue));
    /// }
    ///
    /// ```
//    pub fn paint<D>(&self, val: D) -> style::StyledObject<D>
//    where
//        D: fmt::Display,
//    {
//        style::ObjectStyle::new().apply_to(val, self.context.clone())
//    }

    /// Exit the current process.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    /// use crossterm::terminal;
    /// use crossterm::Context;
    ///
    /// let context = Context::new();
    /// let mut term = terminal::terminal(&context);
    ///
    /// let size = term.exit();
    ///
    /// ```
    pub fn exit(&self) {
        self.terminal.exit();
    }

    /// Write any displayable content to the current terminal screen.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    /// use crossterm::terminal;
    /// use crossterm::Context;
    ///
    /// let context = Context::new();
    /// let mut term = terminal::terminal(&context);
    ///
    /// let size = term.write("Some text \n Some text on new line");
    ///
    /// ```
    pub fn write<D: fmt::Display>(&self, value: D) {
        let mut string = String::new();
        write!(string, "{}", value).unwrap();
        self.screen_manager.write_string(string);
    }
}

/// Get an Terminal implementation whereon terminal related actions can be performed.
///
/// Check `/examples/version/terminal` in the libary for more spesific examples.
///
/// #Example
///
/// ```rust
///
/// extern crate crossterm;
/// use crossterm::terminal;
/// use crossterm::Context;
///
/// let context = Context::new();
///
/// let mut term = terminal::terminal(&context);
///
/// // scroll down by 5 lines
/// let size = term.scroll_down(5);
///
/// ```
///
pub fn terminal(screen_manager: &ScreenManager) -> Terminal {
    Terminal::new(screen_manager)
}
