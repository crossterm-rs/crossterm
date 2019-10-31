//! A module that contains all the actions related to reading input from the terminal.
//! Like reading a line, reading a character and reading asynchronously.

use std::io;

use crate::utils::Result;

// TODO Create a new common AsyncReader structure (like TerminalCursor, TerminalInput, ...).
//      To avoid copy & pasting of the documentation, to sync the code organization, ...
#[cfg(unix)]
pub use self::unix::{AsyncReader, SyncReader};
#[cfg(windows)]
pub use self::windows::{AsyncReader, SyncReader};

#[cfg(unix)]
pub(crate) mod unix;
#[cfg(windows)]
pub(crate) mod windows;

/// This trait defines the actions that can be performed with the terminal input.
/// This trait can be implemented so that a concrete implementation of the ITerminalInput can fulfill
/// the wishes to work on a specific platform.
///
/// ## For example:
///
/// This trait is implemented for Windows and UNIX systems.
/// Unix is using the 'TTY' and windows is using 'libc' C functions to read the input.
pub(crate) trait Input {
    /// Reads one line from the user input and strips the new line character(s).
    ///
    /// This function **does not work** when the raw mode is enabled (see the
    /// [`crossterm_screen`](https://docs.rs/crossterm_screen/) crate documentation
    /// to learn more). You should use the
    /// [`read_async`](struct.TerminalInput.html#method.read_async),
    /// [`read_until_async`](struct.TerminalInput.html#method.read_until_async)
    /// or [`read_sync`](struct.TerminalInput.html#method.read_sync) method if the
    /// raw mode is enabled.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let input = crossterm::input::input();
    /// match input.read_line() {
    ///     Ok(s) => println!("string typed: {}", s),
    ///     Err(e) => println!("error: {}", e),
    /// }
    /// ```
    fn read_line(&self) -> Result<String> {
        let mut rv = String::new();
        io::stdin().read_line(&mut rv)?;
        let len = rv.trim_end_matches(&['\r', '\n'][..]).len();
        rv.truncate(len);
        Ok(rv)
    }

    /// Read one character from the user input
    fn read_char(&self) -> Result<char>;
    /// Read the input asynchronously from the user.
    fn read_async(&self) -> AsyncReader;
    ///  Read the input asynchronously until a certain character is hit.
    fn read_until_async(&self, delimiter: u8) -> AsyncReader;
    /// Read the input synchronously from the user.
    fn read_sync(&self) -> SyncReader;
    /// Start monitoring mouse events.
    fn enable_mouse_mode(&self) -> Result<()>;
    /// Stop monitoring mouse events.
    fn disable_mouse_mode(&self) -> Result<()>;
}
