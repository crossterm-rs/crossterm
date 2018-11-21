//! With this module you can perform actions that are terminal related.
//! Like clearing and scrolling in the terminal or getting the size of the terminal.

use super::*;

use std::fmt;

/// Struct that stores an specific platform implementation for terminal related actions.
///
/// Check `/examples/terminal` in the library for more specific examples.
///
/// ```rust
/// use crossterm::terminal;
///
/// let term = terminal();
///
/// term.scroll_down(5);
/// term.scroll_up(4);
/// let (with, height) = term.terminal_size();
///
/// When you want to use 'terminal' actions on 'alternate screen' use the `Screen` type instead, and pass it to the `terminal::from_screen()` function.
/// By doing that terminal actions will be performed on the alternate screen.
/// ```
pub struct Terminal<'stdout> {
    terminal: Box<ITerminal + Sync + Send>,
    screen: Option<&'stdout Arc<TerminalOutput>>,
}

impl<'stdout> Terminal<'stdout> {
    /// Create new terminal instance whereon terminal related actions can be performed.
    pub fn new() -> Terminal<'stdout> {
        #[cfg(target_os = "windows")]
        let terminal = functions::get_module::<Box<ITerminal + Sync + Send>>(
            Box::new(WinApiTerminal::new()),
            Box::new(AnsiTerminal::new()),
        ).unwrap();

        #[cfg(not(target_os = "windows"))]
        let terminal = Box::from(AnsiTerminal::new()) as Box<ITerminal + Sync + Send>;

        Terminal {
            terminal,
            screen: None,
        }
    }

    /// Create new instance of TerminalInput whereon input related actions could be preformed.
    pub fn on_screen(stdout: &'stdout Arc<TerminalOutput>) -> Terminal<'stdout> {
        #[cfg(target_os = "windows")]
        let terminal = functions::get_module::<Box<ITerminal + Sync + Send>>(
            Box::new(WinApiTerminal::new()),
            Box::new(AnsiTerminal::new()),
        ).unwrap();

        #[cfg(not(target_os = "windows"))]
        let terminal = Box::from(AnsiTerminal::new()) as Box<ITerminal + Sync + Send>;

        Terminal {
            terminal,
            screen: Some(stdout),
        }
    }

    /// Clear the current cursor by specifying the clear type.
    ///
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
    pub fn clear(&self, clear_type: ClearType) {
        self.terminal.clear(clear_type, &self.screen);
    }

    /// Get the terminal size (x,y).
    ///
    /// ```rust
    /// let mut term = terminal();
    ///
    /// let size = term.terminal_size();
    /// println!("{:?}", size);
    /// ```
    pub fn terminal_size(&self) -> (u16, u16) {
        return self.terminal.terminal_size(&self.screen);
    }

    /// Scroll `n` lines up in the current terminal.
    ///
    /// ```rust
    /// let mut term = terminal();
    ///
    /// // scroll up by 5 lines
    /// let size = term.scroll_up(5);
    /// ```
    pub fn scroll_up(&self, count: i16) {
        self.terminal.scroll_up(count, &self.screen);
    }

    /// Scroll `n` lines up in the current terminal.
    ///
    /// ```rust
    /// let mut term = terminal();
    ///
    /// // scroll down by 5 lines
    /// let size = term.scroll_down(5);
    /// ```
    pub fn scroll_down(&self, count: i16) {
        self.terminal.scroll_down(count, &self.screen);
    }

    /// Set the terminal size. Note that not all terminals can be set to a very small scale.
    ///
    /// ```rust
    /// let mut term = terminal();
    ///
    /// // Set of the size to X: 10 and Y: 10
    /// let size = term.set_size(10,10);
    /// ```
    pub fn set_size(&self, width: i16, height: i16) {
        self.terminal.set_size(width, height, &self.screen);
    }

    /// Exit the current process.
    ///
    /// ```rust
    /// let mut term = terminal();
    ///
    /// let size = term.exit();
    /// ```
    pub fn exit(&self) {
        self.terminal.exit(&self.screen);
    }

    /// Write any displayable content to the current terminal screen.
    ///
    /// ```rust
    /// let mut term = terminal();
    ///
    /// let size = term.write("Some text \n Some text on new line");
    /// ```
    pub fn write<D: fmt::Display>(&self, value: D) {
        use std::fmt::Write;
        let mut string = String::new();
        write!(string, "{}", value).unwrap();
        functions::write(&self.screen, string);
    }
}

/// Get an terminal implementation whereon terminal related actions could performed.
/// Pass the reference to any screen you want this type to perform actions on.
pub fn terminal<'stdout>() -> Terminal<'stdout> {
    Terminal::new()
}

/// Get an Terminal Color implementation whereon color related actions can be performed.
/// Pass the reference to any screen you want this type to perform actions on.
pub fn from_screen(screen: &Screen) -> Terminal {
    Terminal::on_screen(&screen.stdout)
}
