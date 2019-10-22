//! # Input
//!
//! **The `crossterm_input` crate is deprecated and no longer maintained. The GitHub repository will
//! be archived soon. All the code is being moved to the `crossterm`
//! [crate](https://github.com/crossterm-rs/crossterm). You can learn more in
//! the [Merge sub-crates to the crossterm crate](https://github.com/crossterm-rs/crossterm/issues/265)
//! issue.**
//!
//! The `crossterm_input` crate provides a functionality to read the input events.
//!
//! This documentation does not contain a lot of examples. The reason is that it's fairly
//! obvious how to use this crate. Although, we do provide
//! [examples](https://github.com/crossterm-rs/examples) repository
//! to demonstrate the capabilities.
//!
//! ## Synchronous vs Asynchronous
//!
//! ### Synchronous Reading
//!
//! Read the input synchronously from the user, the reads performed will be blocking calls.
//! Using synchronous over asynchronous reading has the benefit that it is using fewer resources than
//! the asynchronous because background thread and queues are left away.
//!
//! See the [`SyncReader`](struct.SyncReader.html) documentation for more details.
//!
//! ### Asynchronous Reading
//!
//! Read the input asynchronously, input events are gathered in the background and queued for you to read.
//! Using asynchronous reading has the benefit that input events are queued until you read them. You can poll
//! for occurred events, and the reads won't block your program.
//!
//! See the [`AsyncReader`](struct.AsyncReader.html) documentation for more details.
//!
//! ### Technical details
//!
//! On UNIX systems crossterm reads from the TTY, on Windows, it uses `ReadConsoleInputW`.
//! For asynchronous reading, a background thread will be fired up to read input events,
//! occurred events will be queued on an MPSC-channel, and the user can iterate over those events.
//!
//! The terminal has to be in the raw mode, raw mode prevents the input of the user to be displayed
//! on the terminal screen. See the
//! [`crossterm_screen`](https://docs.rs/crossterm_screen/) crate documentation to learn more.

#[doc(no_inline)]
pub use crate::screen::{IntoRawMode, RawScreen};
#[doc(no_inline)]
pub use crate::utils::Result;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(unix)]
use self::input::unix::UnixInput;
#[cfg(windows)]
use self::input::windows::WindowsInput;
use self::input::Input;
pub use self::input::{AsyncReader, SyncReader};

mod input;
mod sys;

/// Represents an input event.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialOrd, PartialEq, Hash, Clone)]
pub enum InputEvent {
    /// A single key or a combination of keys.
    Keyboard(KeyEvent),
    /// A mouse event.
    Mouse(MouseEvent),
    /// An unsupported event.
    ///
    /// You can ignore this type of event, because it isn't used.
    Unsupported(Vec<u8>), // TODO Not used, should be removed.
    /// An unknown event.
    Unknown,
    /// Internal cursor position event. Don't use it, it will be removed in the
    /// `crossterm` 1.0.
    #[doc(hidden)]
    #[cfg(unix)]
    CursorPosition(u16, u16), // TODO 1.0: Remove
}

/// Represents a mouse event.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialOrd, PartialEq, Hash, Clone, Copy)]
pub enum MouseEvent {
    /// Pressed mouse button at the location (column, row).
    Press(MouseButton, u16, u16),
    /// Released mouse button at the location (column, row).
    Release(u16, u16),
    /// Mouse moved with a pressed left button to the new location (column, row).
    Hold(u16, u16),
    /// An unknown mouse event.
    Unknown,
}

/// Represents a mouse button/wheel.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialOrd, PartialEq, Hash, Clone, Copy)]
pub enum MouseButton {
    /// Left mouse button.
    Left,
    /// Right mouse button.
    Right,
    /// Middle mouse button.
    Middle,
    /// Wheel scrolled up.
    WheelUp,
    /// Wheel scrolled down.
    WheelDown,
}

/// Represents a key or a combination of keys.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum KeyEvent {
    /// Backspace key.
    Backspace,
    /// Enter key.
    Enter,
    /// Left arrow key.
    Left,
    /// Right arrow key.
    Right,
    /// Up arrow key.
    Up,
    /// Down arrow key.
    Down,
    /// Home key.
    Home,
    /// End key.
    End,
    /// Page up key.
    PageUp,
    /// Page dow key.
    PageDown,
    /// Tab key.
    Tab,
    /// Shift + Tab key.
    BackTab,
    /// Delete key.
    Delete,
    /// Insert key.
    Insert,
    /// F key.
    ///
    /// `KeyEvent::F(1)` represents F1 key, etc.
    F(u8),
    /// A character.
    ///
    /// `KeyEvent::Char('c')` represents `c` character, etc.
    Char(char),
    /// Alt key + character.
    ///
    /// `KeyEvent::Alt('c')` represents `Alt + c`, etc.
    Alt(char),
    /// Ctrl key + character.
    ///
    /// `KeyEvent::Ctrl('c') ` represents `Ctrl + c`, etc.
    Ctrl(char),
    /// Null.
    Null,
    /// Escape key.
    Esc,
    /// Ctrl + up arrow key.
    CtrlUp,
    /// Ctrl + down arrow key.
    CtrlDown,
    /// Ctrl + right arrow key.
    CtrlRight,
    /// Ctrl + left arrow key.
    CtrlLeft,
    /// Shift + up arrow key.
    ShiftUp,
    /// Shift + down arrow key.
    ShiftDown,
    /// Shift + right arrow key.
    ShiftRight,
    /// Shift + left arrow key.
    ShiftLeft,
}

/// An internal event.
///
/// Encapsulates publicly available `InputEvent` with additional internal
/// events that shouldn't be publicly available to the crate users.
#[cfg(unix)]
#[derive(Debug, PartialOrd, PartialEq, Hash, Clone)]
pub(crate) enum InternalEvent {
    /// An input event.
    Input(InputEvent),
    /// A cursor position (`x`, `y`).
    CursorPosition(u16, u16),
}

/// Converts an `InternalEvent` into a possible `InputEvent`.
#[cfg(unix)]
impl From<InternalEvent> for Option<InputEvent> {
    fn from(ie: InternalEvent) -> Self {
        match ie {
            InternalEvent::Input(input_event) => Some(input_event),
            // TODO 1.0: Swallow `CursorPosition` and return `None`.
            // `cursor::pos_raw()` will be able to use this module `internal_event_receiver()`
            InternalEvent::CursorPosition(x, y) => Some(InputEvent::CursorPosition(x, y)),
        }
    }
}

/// A terminal input.
///
/// # Examples
///
/// ```no_run
/// // You can replace the following line with `use crossterm::...;`
/// // if you're using the `crossterm` crate with the `input` feature enabled.
/// use crossterm::{Result, TerminalInput, RawScreen};
///
/// fn main() -> Result<()> {
///     let input = TerminalInput::new();
///     // Read a single character
///     let char = input.read_char()?;
///     // Read a single line
///     let line = input.read_line()?;
///
///     // Make sure to enable raw screen when reading input events
///     let screen = RawScreen::into_raw_mode();
///
///     // Create async reader
///     let mut async_stdin = input.read_async();
///
///     // Create sync reader
///     let mut sync_stdin = input.read_sync();
///
///     // Enable mouse input events
///     input.enable_mouse_mode()?;
///     // Disable mouse input events
///     input.disable_mouse_mode()
/// }
/// ```
pub struct TerminalInput {
    #[cfg(windows)]
    input: WindowsInput,
    #[cfg(unix)]
    input: UnixInput,
}

impl TerminalInput {
    /// Creates a new `TerminalInput`.
    pub fn new() -> TerminalInput {
        #[cfg(windows)]
        let input = WindowsInput::new();

        #[cfg(unix)]
        let input = UnixInput::new();

        TerminalInput { input }
    }

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
    /// let input = crossterm::input();
    /// match input.read_line() {
    ///     Ok(s) => println!("string typed: {}", s),
    ///     Err(e) => println!("error: {}", e),
    /// }
    /// ```
    pub fn read_line(&self) -> Result<String> {
        self.input.read_line()
    }

    /// Reads one character from the user input.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let input = crossterm::input();
    /// match input.read_char() {
    ///     Ok(c) => println!("character pressed: {}", c),
    ///     Err(e) => println!("error: {}", e),
    /// }
    /// ```
    pub fn read_char(&self) -> Result<char> {
        self.input.read_char()
    }

    /// Creates a new `AsyncReader` allowing to read the input asynchronously (not blocking).
    ///
    /// If you want a blocking, or less resource consuming read, see the
    /// [`read_sync`](struct.TerminalInput.html#method.read_sync) method.
    ///
    /// # Notes
    ///
    /// * It requires enabled raw mode (see the
    ///   [`crossterm_screen`](https://docs.rs/crossterm_screen/) crate documentation to learn more).
    /// * A thread is spawned to read the input.
    /// * The reading thread is cleaned up when you drop the [`AsyncReader`](struct.AsyncReader.html).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::{thread, time::Duration};
    /// use crossterm::input;
    ///
    /// let mut async_stdin = input().read_async();
    ///
    /// loop {
    ///     if let Some(key_event) = async_stdin.next() {
    ///         /* Check which event occurred here */
    ///     }
    ///
    ///     thread::sleep(Duration::from_millis(50));
    /// }
    ///  ```
    pub fn read_async(&self) -> AsyncReader {
        self.input.read_async()
    }

    /// Creates a new `AsyncReader` allowing to read the input asynchronously (not blocking) until the
    /// given `delimiter`.
    ///
    /// It behaves in the same way as the [`read_async`](struct.TerminalInput.html#method.read_async)
    /// method, but it stops reading when the `delimiter` is hit.
    ///
    /// # Notes
    ///
    /// * It requires enabled raw mode (see the
    ///   [`crossterm_screen`](https://docs.rs/crossterm_screen/) crate documentation to learn more).
    /// * A thread is spawned to read the input.
    /// * The reading thread is cleaned up when you drop the [`AsyncReader`](struct.AsyncReader.html).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::{thread, time::Duration};
    ///
    /// let mut async_stdin = crossterm::input().read_until_async(b'x');
    ///
    /// loop {
    ///     if let Some(key_event) = async_stdin.next() {
    ///         /* Check which event occurred here */
    ///     }
    ///
    ///     thread::sleep(Duration::from_millis(50));
    /// }
    ///  ```
    pub fn read_until_async(&self, delimiter: u8) -> AsyncReader {
        self.input.read_until_async(delimiter)
    }

    /// Creates a new `SyncReader` allowing to read the input synchronously (blocking).
    ///
    /// It's less resource hungry when compared to the
    /// [`read_async`](struct.TerminalInput.html#method.read_async) method, because it doesn't
    /// spawn any reading threads.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::{thread, time::Duration};
    ///
    /// let mut sync_stdin = crossterm::input().read_sync();
    ///
    /// loop {
    ///     if let Some(key_event) = sync_stdin.next() {
    ///         /* Check which event occurred here */
    ///     }
    /// }
    ///  ```
    pub fn read_sync(&self) -> SyncReader {
        self.input.read_sync()
    }

    /// Enables mouse events.
    ///
    /// Mouse events will be produced by the
    /// [`AsyncReader`](struct.AsyncReader.html)/[`SyncReader`](struct.SyncReader.html).
    pub fn enable_mouse_mode(&self) -> Result<()> {
        self.input.enable_mouse_mode()
    }

    /// Disables mouse events.
    ///
    /// Mouse events wont be produced by the
    /// [`AsyncReader`](struct.AsyncReader.html)/[`SyncReader`](struct.SyncReader.html).
    pub fn disable_mouse_mode(&self) -> Result<()> {
        self.input.disable_mouse_mode()
    }
}

/// Creates a new `TerminalInput`.
///
/// # Examples
///
/// ```no_run
/// // You can replace the following line with `use crossterm::...;`
/// // if you're using the `crossterm` crate with the `input` feature enabled.
/// use crossterm::{input, RawScreen, Result};
///
/// fn main() -> Result<()> {
///     let input = input();
///     // Read a single character
///     let char = input.read_char()?;
///     // Read a single line
///     let line = input.read_line()?;
///
///     // Make sure to enable raw screen when reading input events
///     let screen = RawScreen::into_raw_mode();
///
///     // Create async reader
///     let mut async_stdin = input.read_async();
///
///     // Create sync reader
///     let mut sync_stdin = input.read_sync();
///
///     // Enable mouse input events
///     input.enable_mouse_mode()?;
///     // Disable mouse input events
///     input.disable_mouse_mode()
/// }
/// ```
pub fn input() -> TerminalInput {
    TerminalInput::new()
}
