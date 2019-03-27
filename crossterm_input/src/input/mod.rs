//! A module that contains all the actions related to reading input from the terminal.
//! Like reading a line, reading a character and reading asynchronously.

mod input;

#[cfg(not(target_os = "windows"))]
mod unix_input;
#[cfg(target_os = "windows")]
mod windows_input;

#[cfg(not(target_os = "windows"))]
use self::unix_input::UnixInput;
#[cfg(target_os = "windows")]
use self::windows_input::WindowsInput;

pub use self::input::{input, parse_event, TerminalInput};

use std::io::{self, Read, Result};
use std::sync::{mpsc, Arc};

use crossterm_utils::TerminalOutput;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;

/// This trait defines the actions that can be preformed with the terminal input.
/// This trait can be implemented so that a concrete implementation of the ITerminalInput can fulfill
/// the wishes to work on a specific platform.
///
/// ## For example:
///
/// This trait is implemented for Windows and UNIX systems.
/// Unix is using the 'TTY' and windows is using 'libc' C functions to read the input.
trait ITerminalInput {
    /// Read one character from the user input
    fn read_char(&self, stdout: &Option<&Arc<TerminalOutput>>) -> io::Result<char>;
    /// Read the input asynchronously from the user.
    fn read_async(&self, stdout: &Option<&Arc<TerminalOutput>>) -> AsyncReader;
    ///  Read the input asynchronously until a certain character is hit.
    fn read_until_async(&self, delimiter: u8, stdout: &Option<&Arc<TerminalOutput>>)
        -> AsyncReader;
    fn enable_mouse_mode(&self, stdout: &Option<&Arc<TerminalOutput>>) -> io::Result<()>;
    fn disable_mouse_mode(&self, stdout: &Option<&Arc<TerminalOutput>>) -> io::Result<()>;
}

/// Enum to specify which input event has occurred.
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
pub enum MouseEvent {
    /// A mouse press has occurred, this contains the pressed button and the position of the press.
    Press(MouseButton, u16, u16),
    /// A mouse button was released.
    Release(u16, u16),
    /// A mouse button was hold.
    Hold(u16, u16),
}

/// Enum to define mouse buttons.
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
    Delete,
    Insert,
    F(u8),
    Char(char),
    Alt(char),
    Ctrl(char),
    Null,
    Esc,
}

/// This is a wrapper for reading from the input asynchronously.
/// This wrapper has a channel receiver that receives the input from the user whenever it typed something.
/// You only need to check whether there are new characters available.
pub struct AsyncReader {
    function: Box<Fn(&Sender<u8>, &Arc<AtomicBool>) + Send>,
}

impl AsyncReader {
    pub fn new(function: Box<Fn(&Sender<u8>, &Arc<AtomicBool>) + Send>) -> AsyncReader {
        AsyncReader { function }
    }

    pub fn start_receiving(mut self) -> AsyncReadHandle {
        let shutdown_handle = Arc::new(AtomicBool::new(false));

        let (event_tx, event_rx) = mpsc::channel();
        let thread_shutdown = shutdown_handle.clone();

        let function = self.function;

        thread::spawn(move || loop {
            function(&event_tx, &thread_shutdown);
        });

        AsyncReadHandle {
            event_rx,
            shutdown: shutdown_handle,
        }
    }
}

pub struct AsyncReadHandle {
    event_rx: Receiver<u8>,
    shutdown: Arc<AtomicBool>,
}

impl AsyncReadHandle {
    pub fn stop_receiving(&mut self) {
        self.shutdown.store(true, Ordering::SeqCst);
    }
}

impl Iterator for AsyncReadHandle {
    type Item = InputEvent;

    fn next(&mut self) -> Option<Self::Item> {
        let mut iterator = self.event_rx.iter();

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

impl Drop for AsyncReadHandle {
    fn drop(&mut self) {
        self.stop_receiving();
    }
}
