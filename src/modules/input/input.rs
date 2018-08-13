//! With this module you can perform actions that are input related.
//! Like reading a line, reading a character and reading asynchronously.

use std::io;
use std::sync::Arc;
use super::*;

/// Struct that stores an specific platform implementation for input related actions.
///
/// Check `/examples/input` the examples folder on github for more info.
///
/// #Example
///
/// ```rust
///
/// extern crate crossterm;
/// use self::crossterm::input::input;
///
/// let input = input(&Screen::default());
/// let result = input.read_line();
/// let pressed_char = input.read_char();
///
/// ```
pub struct TerminalInput {
    terminal_input: Box<ITerminalInput>,
    stdout: Arc<Stdout>,
}

impl TerminalInput{
    /// Create new instance of TerminalInput whereon input related actions could be preformed.
    pub fn new(stdout: &Arc<Stdout>) -> TerminalInput {
        #[cfg(target_os = "windows")]
        let input = Box::from(WindowsInput::new());

        #[cfg(not(target_os = "windows"))]
        let input = Box::from(UnixInput::new());

        TerminalInput {
            terminal_input: input,
            stdout: stdout.clone(),
        }
    }

    /// Read one line from the user input.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    ///  match input(&Screen::default()).read_line() {
    ///     Ok(s) => println!("string typed: {}", s),
    ///     Err(e) => println!("error: {}", e),
    ///  }
    ///
    /// ```
    pub fn read_line(&self) -> io::Result<String> {
        self.terminal_input.read_line(&self.stdout)
    }

    /// Read one character from the user input
    ///
    /// #Example
    ///
    /// ```rust
    ///
    ///  match crossterm.input(&Screen::default()).read_char() {
    ///     Ok(c) => println!("character pressed: {}", c),
    ///     Err(e) => println!("error: {}", e),
    ///   }
    ///
    /// ```
    pub fn read_char(&self) -> io::Result<char> {
        return self.terminal_input.read_char(&self.stdout);
    }

    /// Read the input asynchronously from the user.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// use crossterm::{Crossterm, Screen}
    ///
    /// // we need to enable raw mode otherwise the characters will be outputted by default before we are able to read them.
    /// let screen = Screen::new(true);
    /// let crossterm = Crossterm::new();
    ///
    /// let mut stdin = crossterm.input(&screen).read_async().bytes();
    ///
    /// for i in 0..100 {
    ///
    ///     // Get the next character typed. This is None if nothing is pressed. And Some(Ok(u8 value of character))
    ///     let a = stdin.next();
    ///
    ///     println!("pressed key: {:?}", a);
    ///
    ///     if let Some(Ok(b'x')) = a {
    ///         println!("The key: `x` was pressed and program is terminated.");
    ///         break;
    ///     }
    ///     // simulate some timeout so that we can see the character on the screen.
    ///     thread::sleep(time::Duration::from_millis(50));
    /// }
    ///
    /// ```
    pub fn read_async(&self) -> AsyncReader {
        self.terminal_input.read_async(&self.stdout)
    }

    ///  Read the input asynchronously until a certain character is hit.
    ///
    /// #Example
    ///
    /// ```rust
    /// // we need to enable raw mode otherwise the characters will be outputted by default before we are able to read them.
    /// let screen = Screen::new(true);
    ///
    /// let crossterm = Crossterm::new();
    /// let input = crossterm.input(&screen);
    /// let terminal = crossterm.terminal(&screen);
    /// let mut cursor = crossterm.cursor(&screen);
    ///
    ///
    /// let mut stdin = input.read_until_async(b'\r').bytes();
    ///
    /// for i in 0..100 {
    ///     terminal.clear(ClearType::All);
    ///     cursor.goto(1, 1);
    ///     let a = stdin.next();
    ///
    ///     println!("pressed key: {:?}", a);
    ///
    ///     if let Some(Ok(b'\r')) = a {
    ///         println!("The enter key is hit and program is not listening to input anymore.");
    ///         break;
    ///     }
    ///
    ///     if let Some(Ok(b'x')) = a {
    ///          println!("The key: x was pressed and program is terminated.");
    ///         break;
    ///     }
    ///
    ///     thread::sleep(time::Duration::from_millis(100));
    /// }
    /// ```
    pub fn read_until_async(&self, delimiter: u8) -> AsyncReader {
        self.terminal_input
            .read_until_async(delimiter, &self.stdout)
    }
}

/// Get an Terminal Input implementation whereon input related actions can be performed.
/// Pass the reference to any screen you want this type to perform actions on.
pub fn input(stdout: &Screen) -> TerminalInput {
    return TerminalInput::new(&stdout.stdout);
}
