//! A module that contains all the actions related to reading input from the terminal.
//! Like reading a line, reading a character and reading asynchronously.

mod input;

#[cfg(unix)]
mod unix_input;
#[cfg(windows)]
mod windows_input;

#[cfg(unix)]
pub use self::unix_input::SyncReader;
#[cfg(unix)]
use self::unix_input::UnixInput;

#[cfg(windows)]
pub use self::windows_input::SyncReader;
#[cfg(windows)]
use self::windows_input::WindowsInput;

use self::input::parse_event;
pub use self::input::{input, TerminalInput};
use crossterm_utils::{ErrorKind, Result};
use std::io;
use std::sync::{mpsc, Arc};

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

/// This trait defines the actions that can be performed with the terminal input.
/// This trait can be implemented so that a concrete implementation of the ITerminalInput can fulfill
/// the wishes to work on a specific platform.
///
/// ## For example:
///
/// This trait is implemented for Windows and UNIX systems.
/// Unix is using the 'TTY' and windows is using 'libc' C functions to read the input.
trait ITerminalInput {
    /// Read one character from the user input
    fn read_char(&self) -> io::Result<char>;
    /// Read the input asynchronously from the user.
    fn read_async(&self) -> AsyncReader;
    ///  Read the input asynchronously until a certain character is hit.
    fn read_until_async(&self, delimiter: u8) -> AsyncReader;
    /// Read the input synchronously from the user.
    fn read_sync(&self) -> SyncReader;
    fn enable_mouse_mode(&self) -> Result<()>;
    fn disable_mouse_mode(&self) -> Result<()>;
}

/// Enum to specify which input event has occurred.
#[derive(Debug, PartialOrd, PartialEq, Hash)]
pub enum InputEvent {
    /// A single key or a combination is pressed.
    Keyboard(KeyEvent),
    /// A mouse event occurred.
    Mouse(MouseEvent),
    /// A unsupported event has occurred.
    Unsupported(Vec<u8>),
    /// An unknown event has occurred.
    Unknown,
}

/// Enum to specify which mouse event has occurred.
#[derive(Debug, PartialOrd, PartialEq, Hash)]
pub enum MouseEvent {
    /// A mouse press has occurred, this contains the pressed button and the position of the press.
    Press(MouseButton, u16, u16),
    /// A mouse button was released.
    Release(u16, u16),
    /// A mouse button was hold.
    Hold(u16, u16),
    /// An unknown mouse event has occurred.
    Unknown,
}

/// Enum to define mouse buttons.
#[derive(Debug, PartialOrd, PartialEq, Hash)]
pub enum MouseButton {
    /// Left mouse button
    Left,
    /// Right mouse button
    Right,
    /// Middle mouse button
    Middle,
    /// Scroll up
    WheelUp,
    /// Scroll down
    WheelDown,
}

/// Enum with different key or key combinations.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub enum KeyEvent {
    Backspace,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    BackTab,
    Delete,
    Insert,
    F(u8),
    Char(char),
    Alt(char),
    Ctrl(char),
    Null,
    Esc,
    CtrlUp,
    CtrlDown,
    CtrlRight,
    CtrlLeft,
    ShiftUp,
    ShiftDown,
    ShiftRight,
    ShiftLeft,
}

/// This type allows you to read the input asynchronously which means that input events are gathered on the background and will be queued for you to read.
///
/// **[SyncReader](./LINK)**
/// If you want a blocking, or less resource consuming read to happen use `SyncReader`, this will leave a way all the thread and queueing and will be a blocking read.
///
/// This type is an iterator, and could be used to iterate over input events.
///
/// # Remarks
/// - Threads spawned will be disposed of as soon the `AsyncReader` goes out of scope.
/// - MPSC-channels are used to queue input events, this type implements an iterator of the rx side of the queue.
pub struct AsyncReader {
    event_rx: Receiver<u8>,
    shutdown: Arc<AtomicBool>,
}

impl AsyncReader {
    /// Construct a new instance of the `AsyncReader`.
    /// The reading will immediately start when calling this function.
    pub fn new(function: Box<Fn(&Sender<u8>, &Arc<AtomicBool>) + Send>) -> AsyncReader {
        let shutdown_handle = Arc::new(AtomicBool::new(false));

        let (event_tx, event_rx) = mpsc::channel();
        let thread_shutdown = shutdown_handle.clone();

        thread::spawn(move || loop {
            function(&event_tx, &thread_shutdown);
        });

        AsyncReader {
            event_rx,
            shutdown: shutdown_handle,
        }
    }

    /// Stop the input event reading.
    ///
    /// You don't necessarily have to call this function because it will automatically be called when this reader goes out of scope.
    ///
    /// # Remarks
    /// - Background thread will be closed.
    /// - This will consume the handle you won't be able to restart the reading with this handle, create a new `AsyncReader` instead.
    pub fn stop_reading(&mut self) {
        self.shutdown.store(true, Ordering::SeqCst);
    }
}

impl Iterator for AsyncReader {
    type Item = InputEvent;

    /// Check if there are input events to read.
    ///
    /// It will return `None` when nothing is there to read, `Some(InputEvent)` if there are events to read.
    ///
    /// # Remark
    /// - This is **not** a blocking call.
    fn next(&mut self) -> Option<Self::Item> {
        let mut iterator = self.event_rx.try_iter();

        match iterator.next() {
            Some(char_value) => {
                if let Ok(char_value) = parse_event(char_value, &mut iterator) {
                    Some(char_value)
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

impl Drop for AsyncReader {
    fn drop(&mut self) {
        self.stop_reading();
    }
}
