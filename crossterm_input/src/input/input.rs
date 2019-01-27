//! A module that contains all the actions related to reading input from the terminal.
//! Like reading a line, reading a character and reading asynchronously.

use super::*;
use std::{thread, time::Duration};

use crossterm_utils::TerminalOutput;

/// Allows you to preform actions with the < option >.
///
/// # Features:
///
/// - features
///
/// Check `/examples/` in the library for more specific examples.
///
/// # Remarks
///
/// When you want to use '< name >' on 'alternate screen' use the 'crossterm_screen' crate.

/// Allows you to read user input.
///
/// # Features:
///
/// - Read character
/// - Read line
/// - Read async
/// - Read async until
/// - Wait for key event (terminal pause)
///
/// Check `/examples/` in the library for more specific examples.
///
/// # Remarks
///
/// When you want to use 'input' on 'alternate screen' use the 'crossterm_screen' crate.
pub struct TerminalInput<'stdout> {
    terminal_input: Box<ITerminalInput + Sync + Send>,
    stdout: Option<&'stdout Arc<TerminalOutput>>,
}

impl<'stdout> TerminalInput<'stdout> {
    /// Create a new instance of `TerminalInput` whereon input related actions could be preformed.
    pub fn new() -> TerminalInput<'stdout> {
        #[cfg(target_os = "windows")]
        let input = Box::from(WindowsInput::new());

        #[cfg(not(target_os = "windows"))]
        let input = Box::from(UnixInput::new());

        TerminalInput {
            terminal_input: input,
            stdout: None,
        }
    }

    /// Create a new instance of `TerminalInput` whereon input related actions could be preformed.
    ///
    /// # Remarks
    ///
    /// Use this function when you want your terminal to operate with a specific output.
    /// This could be useful when you have a screen which is in 'alternate mode',
    /// and you want your actions from the `TerminalInput`, created by this function, to operate on the 'alternate screen'.
    ///
    /// You should checkout the 'crossterm_screen' crate for more information about this.
    /// # Example
    /// ```rust
    /// let screen = Screen::default();
    //
    /// if let Ok(alternate) = screen.enable_alternate_modes(false) {
    ///    let terminal = TerminalInput::from_output(&alternate.screen.stdout);
    /// }
    /// ```
    pub fn from_output(stdout: &'stdout Arc<TerminalOutput>) -> TerminalInput<'stdout> {
        #[cfg(target_os = "windows")]
        let input = Box::from(WindowsInput::new());

        #[cfg(not(target_os = "windows"))]
        let input = Box::from(UnixInput::new());

        TerminalInput {
            terminal_input: input,
            stdout: Some(stdout),
        }
    }

    /// Read one line from the user input.
    ///
    /// # Remark
    /// This function is not work when raw screen is turned on.
    /// When you do want to read a line in raw mode please, checkout `read_async` or `read_async_until`.
    /// Not sure what 'raw mode' is, checkout the 'crossterm_screen' crate.
    ///
    /// # Example
    /// ```rust
    /// let input = input();
    ///  match input.read_line() {
    ///     Ok(s) => println!("string typed: {}", s),
    ///     Err(e) => println!("error: {}", e),
    ///  }
    /// ```
    pub fn read_line(&self) -> io::Result<String> {
        if let Some(stdout) = self.stdout {
            if stdout.is_in_raw_mode {
                return Err(Error::new(ErrorKind::Other, "Crossterm does not support readline in raw mode this should be done instead whit `read_async` or `read_async_until`"));
            }
        }

        let mut rv = String::new();
        io::stdin().read_line(&mut rv)?;
        let len = rv.trim_right_matches(&['\r', '\n'][..]).len();
        rv.truncate(len);
        Ok(rv)
    }

    /// Read one character from the user input
    ///
    /// ```rust
    /// let input = input();
    ///
    ///  match input.read_char() {
    ///     Ok(c) => println!("character pressed: {}", c),
    ///     Err(e) => println!("error: {}", e),
    ///   }
    /// ```
    pub fn read_char(&self) -> io::Result<char> {
        self.terminal_input.read_char(&self.stdout)
    }

    /// Read the input asynchronously from the user.
    ///
    /// # Remarks
    /// - This call will not block the current thread.
    ///   A thread will be fired to read input, on unix systems from TTY and on windows systems with '_getwch' and '_getwche'.
    /// - Requires 'raw screen to be enabled'.
    ///   Not sure what this is, please checkout the 'crossterm_screen' crate.
    ///
    /// ```rust
    /// // we need to enable raw mode otherwise the characters will be outputted by default before we are able to read them.
    /// let screen = Screen::new(true);
    /// let input = crossterm::input::from_screen(&screen);
    ///
    /// let mut stdin = input.read_async().bytes();
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

    /// Read the input asynchronously until a certain character is hit.
    ///
    /// This is the same as `read_async()` but stops reading when a certain character is hit.
    ///
    /// # Remarks
    /// - This call will not block the current thread.
    ///   A thread will be fired to read input, on unix systems from TTY and on windows systems with '_getwch' and '_getwche'.
    /// - Requires 'raw screen to be enabled'.
    ///   Not sure what this is, please checkout the 'crossterm_screen' crate.
    /// - Thread is automatically destroyed when the 'delimiter' is hit.
    ///
    /// ```rust
    /// // we need to enable raw mode otherwise the characters will be outputted by default before we are able to read them.
    /// let screen = Screen::new(true);
    ///
    /// // create an instance of `Crossterm` which will preform the actions on the raw screen.
    /// let crossterm = Crossterm::from_screen(&screen);
    /// let input = crossterm.input();
    /// let terminal = crossterm.terminal();
    /// let mut cursor = crossterm.cursor();
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

    /// This will prevent the current thread from continuing until the passed `KeyEvent` has happened.
    ///
    /// # Remark
    /// - Requires 'raw screen to be enabled'.
    ///   Not sure what this is, please checkout the 'crossterm_screen' crate.
    ///
    /// ```
    /// use crossterm::input::{TerminalInput, KeyEvent};
    ///
    /// fn main() {
    ///     println!("Press 'x' to quit...");
    ///     TerminalInput::wait_until(KeyEvent::OnKeyPress(b'x'));
    /// }
    /// ```
    pub fn wait_until(&self, key_event: KeyEvent) {
        let mut stdin = self.read_async().bytes();

        loop {
            let pressed_key: Option<Result<u8, Error>> = stdin.next();

            match pressed_key {
                Some(Ok(value)) => match key_event {
                    KeyEvent::OnKeyPress(ascii_code) => {
                        if value == ascii_code {
                            break;
                        }
                    }
                    KeyEvent::OnEnter => {
                        if value == b'\r' {
                            break;
                        }
                    }
                    KeyEvent::OnAnyKeyPress => {
                        break;
                    }
                },
                _ => {}
            }

            // some sleeping time so that we don't 'dos' our cpu.
            thread::sleep(Duration::from_millis(10));
        }
    }
}

/// Get a `TerminalInput` instance whereon input related actions can be performed.
pub fn input<'stdout>() -> TerminalInput<'stdout> {
    TerminalInput::new()
}
