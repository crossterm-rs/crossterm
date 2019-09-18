//! A module that contains all the actions related to reading input from the terminal.
//! Like reading a line, reading a character and reading asynchronously.

use std::io;

use crossterm_utils::Result;

#[cfg(unix)]
use super::unix_input::{AsyncReader, SyncReader, UnixInput};
#[cfg(windows)]
use super::windows_input::{AsyncReader, SyncReader, WindowsInput};
use super::ITerminalInput;

/// Allows you to read user input.
///
/// # Features:
///
/// - Read character
/// - Read line
/// - Read async
/// - Read async until
/// - Read sync
/// - Wait for key event (terminal pause)
///
/// Check `/examples/` in the library for more specific examples.
pub struct TerminalInput {
    #[cfg(windows)]
    input: WindowsInput,
    #[cfg(unix)]
    input: UnixInput,
}

impl TerminalInput {
    /// Create a new instance of `TerminalInput` whereon input related actions could be performed.
    pub fn new() -> TerminalInput {
        #[cfg(windows)]
        let input = WindowsInput::new();

        #[cfg(unix)]
        let input = UnixInput::new();

        TerminalInput { input }
    }

    /// Read one line from the user input.
    ///
    /// # Remark
    /// This function is not work when raw screen is turned on.
    /// When you do want to read a line in raw mode please, checkout `read_async`, `read_async_until` or `read_sync`.
    /// Not sure what 'raw mode' is, checkout the 'crossterm_screen' crate.
    ///
    /// # Example
    /// ```ignore
    /// let in = input();
    /// match in.read_line() {
    ///     Ok(s) => println!("string typed: {}", s),
    ///     Err(e) => println!("error: {}", e),
    /// }
    /// ```
    pub fn read_line(&self) -> Result<String> {
        let mut rv = String::new();
        io::stdin().read_line(&mut rv)?;
        let len = rv.trim_end_matches(&['\r', '\n'][..]).len();
        rv.truncate(len);
        Ok(rv)
    }

    /// Read one character from the user input
    ///
    /// ```ignore
    /// let in = input();
    /// match in.read_char() {
    ///     Ok(c) => println!("character pressed: {}", c),
    ///     Err(e) => println!("error: {}", e),
    /// }
    /// ```
    pub fn read_char(&self) -> Result<char> {
        self.input.read_char()
    }

    /// Read the input asynchronously, which means that input events are gathered on the background and will be queued for you to read.
    ///
    /// If you want a blocking, or less resource consuming read to happen use `read_sync()`, this will leave a way all the thread and queueing and will be a blocking read.
    ///
    /// This is the same as `read_async()` but stops reading when a certain character is hit.
    ///
    /// # Remarks
    /// - Readings won't be blocking calls.
    ///   A thread will be fired to read input, on unix systems from TTY and on windows WinApi
    ///   `ReadConsoleW` will be used.
    /// - Input events read from the user will be queued on a MPSC-channel.
    /// - The reading thread will be cleaned up when it drops.
    /// - Requires 'raw screen to be enabled'.
    ///   Not sure what this is? Please checkout the 'crossterm_screen' crate.
    ///
    /// # Examples
    /// Please checkout the example folder in the repository.
    pub fn read_async(&self) -> AsyncReader {
        self.input.read_async()
    }

    /// Read the input asynchronously until a certain delimiter (character as byte) is hit, which means that input events are gathered on the background and will be queued for you to read.
    ///
    /// If you want a blocking or less resource consuming read to happen, use `read_sync()`. This will leave alone the background thread and queues and will be a blocking read.
    ///
    /// This is the same as `read_async()` but stops reading when a certain character is hit.
    ///
    /// # Remarks
    /// - Readings won't be blocking calls.
    ///   A thread will be fired to read input, on unix systems from TTY and on windows WinApi
    ///   `ReadConsoleW` will be used.
    /// - Input events read from the user will be queued on a MPSC-channel.
    /// - The reading thread will be cleaned up when it drops.
    /// - Requires 'raw screen to be enabled'.
    ///   Not sure what this is? Please checkout the 'crossterm_screen' crate.
    ///
    /// # Examples
    /// Please checkout the example folder in the repository.
    pub fn read_until_async(&self, delimiter: u8) -> AsyncReader {
        self.input.read_until_async(delimiter)
    }

    /// Read the input synchronously from the user, which means that reading calls will block.
    /// It also uses less resources than the `AsyncReader` because the background thread and queues are left alone.
    ///
    /// Consider using `read_async` if you don't want the reading call to block your program.
    ///
    /// # Remark
    /// - Readings will be blocking calls.
    ///
    /// # Examples
    /// Please checkout the example folder in the repository.
    pub fn read_sync(&self) -> SyncReader {
        self.input.read_sync()
    }

    /// Enable mouse events to be captured.
    ///
    /// When enabling mouse input, you will be able to capture mouse movements, pressed buttons, and locations.
    ///
    /// # Remark
    /// - Mouse events will be send over the reader created with `read_async`, `read_async_until`, `read_sync`.
    pub fn enable_mouse_mode(&self) -> Result<()> {
        self.input.enable_mouse_mode()
    }

    /// Disable mouse events to be captured.
    ///
    /// When disabling mouse input, you won't be able to capture mouse movements, pressed buttons, and locations anymore.
    pub fn disable_mouse_mode(&self) -> Result<()> {
        self.input.disable_mouse_mode()
    }
}

/// Get a `TerminalInput` instance whereon input related actions can be performed.
pub fn input() -> TerminalInput {
    TerminalInput::new()
}
