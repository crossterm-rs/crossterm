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
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::thread;
use std::sync::atomic::Ordering;

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
    fn read_async(&self, stdout: &Option<&Arc<TerminalOutput>>) -> AsyncReader<impl Fn(&Sender<InputEvent>)>;
    ///  Read the input asynchronously until a certain character is hit.
    fn read_until_async(&self, delimiter: u8, stdout: &Option<&Arc<TerminalOutput>>)
        -> AsyncReader<impl Fn(&Sender<InputEvent>)>;
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
pub struct AsyncReader<F: Fn(&Sender<InputEvent>)> {
    function: F,
    cancel_tx: Sender<bool>,
    cancel_rx: Receiver<bool>,
    shutdown: Arc<AtomicBool>,
    event_rx: Receiver<u8>,
    event_tx: Sender<u8>
}
// (dyn for<'r> Fn(&'r Sender<InputEvent>) + 'static)
impl<F: Fn(&Sender<InputEvent>)> AsyncReader<F> {
    pub fn new(function: F) -> AsyncReader<F> {
        let (event_tx, event_rx) = mpsc::channel();
        let (cancel_tx, cancel_rx) = mpsc::channel();

        AsyncReader {
            function,
            cancel_tx,
            cancel_rx,
            shutdown: Arc::new(AtomicBool::new(false)),
            event_rx,
            event_tx
        }
    }

    pub fn start_receiving(&mut self) {
        let shutdown = self.shutdown.clone();
        let sender = self.event_tx.clone();

        thread::spawn(|| {
            loop {
                self.function(&sender);

                if self.cancellation_requested() || shutdown.load(Ordering::SeqCst) {
                    return;
                }
            }
        });
    }

    pub fn stop_receiving(&mut self) {
        self.shutdown.store(true, Ordering::SeqCst);
    }

    fn cancellation_requested(&self) -> bool {
        if let Ok(cancellation) = self.cancel_rx.try_recv() {
            return true;
        }

        false
    }
}

impl<F> Iterator for AsyncReader<F>  where F: Fn(&Sender<InputEvent>) {
    type Item = InputEvent;

    fn next(&mut self) -> Option<Self::Item> {
        match self.event_rx.try_recv() {
            Ok(char_value) => {
                parse_event(char_value, self)
                    .map(|x| Some(x))
                    .unwrap_or(None)
            },
            Err(e) => { None },
        }
    }
}

impl<F> Drop for AsyncReader<F> where F: Fn(&Sender<InputEvent>) {
    fn drop(&mut self) {
        self.stop_receiving();
    }
}

//impl Read for AsyncReader {
//    /// Read from the byte stream.
//    ///
//    /// This will never block, but try to drain the event queue until empty. If the total number of
//    /// bytes written is lower than the buffer's length, the event queue is empty or that the event
//    /// stream halted.
//    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
//        let mut total = 0;
//
//        loop {
//            if *self.atomic_bool.get_mut() == true || total >= buf.len() {
//                break;
//            }
//
//            match self.event_rx.try_recv() {
//                Ok(Ok(value)) => {
//                    buf[total] = value;
//                    total += 1;
//                }
//                Ok(Err(e)) => return Err(e),
//                Err(_) => break,
//            }
//        }
//
//        Ok(total)
//    }
//}
