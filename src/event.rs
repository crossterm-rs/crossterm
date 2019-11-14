//! # Event
//! The `event` module provides the functionality to read events.
//! Events include: input events, signal events, and terminal events.
//! Currently, only input events are supported however the ohter ones are upcomming.
//!
//! There are two functions important to know when you want to read events with crossterm.
//!
//! Those are:
//! 1. [poll(Duration)](./event/fn.poll.html)
//! Tells you if there are any events to be read withing the given optional duration.
//! 2. [read](./event/fn.read.html)
//! Reads events and returns immediately if there are events. Otherwise, a blocking read is performed.
//!
//! These two functions can be used together to read events asynchronous and synchronous.
//! The useful thing about `poll` is that it gives you complete control over how long you want to wait for an event while `read` blocks until an event occurs.
//!
//! Next to those two functions we have `wake()` that can be usefull in async envoirments.
//! This function will directly interupt the `poll` call and will make it return `Ok(false)`.
//!
//! Let's look at an example that shows these two functions in action.
//!
//! ```no_run
//! use crossterm::event::{poll, read, Event};
//! use std::time::Duration;
//!
//! fn try_get_event() -> crossterm::Result<()> {
//!     if poll(Some(Duration::from_millis(500)))? {
//!         match read()? {
//!             Event::Key(key_event) => { println!("{:?}", key_event) }
//!             Event::Mouse(mouse_event) => { println!("{:?}", mouse_event) }
//!             Event::Resize(width, height) => { println!("Terminal resized to {}x{}", width, height) }
//!         }
//!     } else {
//!         println!("timeout occurred");
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! As you can see, we poll first for input.
//! We indicate that we want to wait a maximum of 500ms for this.
//! If an event has occurred during this time, we will read it with `read`, and print it to the console.
//! Otherwise we print "timeout occured".
//!
//! Please have a look over at the [examples directory](https://github.com/crossterm-rs/examples) for more robust examples.
//!
//! ## Technical Implementation
//! Crossterm uses the poll/read meganism.
//! **Unix**
//!
//! [MIO](https://docs.rs/mio/) is used on UNIX systems. It will poll for event readiness from an file descriptor.
//! **Windows**
//!
//! On windows crossterm uses `WaitForMultipleObjects`, with this call we wait for a signal from eighter the input HANDLE or a semaphore HANDLE.
//! The semaphore HANDLE can be used to interupt the the waiting.
//!
//! `poll` and `read` are static functions that both aquire an underlying lock to crossterms input system.
//! You mustn't call `poll` from two threads at the same time because this can cause a deadlock.
//! However, `poll` and `read` can be called independently without influencing each other.

use std::time::Duration;

use parking_lot::RwLock;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use filter::{EventFilter, Filter};
use lazy_static::lazy_static;
use timeout::PollTimeout;

use crate::{Command, Result};

mod ansi;
pub(crate) mod filter;
mod read;
mod source;
mod sys;
mod timeout;

lazy_static! {
    /// Static instance of `InternalEventReader`.
    /// This needs to be static because there can be one event reader.
    static ref INTERNAL_EVENT_READER: RwLock<read::InternalEventReader> = { RwLock::new(read::InternalEventReader::default()) };
}

/// Polls during an given duration for ready events.
///
/// This function takes in an optional duration.
/// * `None`: blocks indefinitely until an event is able to be read.
/// * `Some(duration)`: blocks for the given duration.
///
/// The following value is returned returned when:
/// * `Ok(true)`: an event is ready.
/// * `Ok(false)`: the given duration is elapsed.
/// * `Err(err)`: there is an error.
///
/// Read an ready event with [read](fn.read.html)
/// ```no_run
/// use std::time::Duration;
/// use crossterm::{Result, event::poll};
///
/// fn main() -> Result<()> {
///     // poll maximal 1 second
///     if poll(Some(Duration::from_millis(1000)))? {  /* logic */  }
///
///     // poll indefinitely
///     if poll(None)? { /* logic */  }
///
///     Ok(())
/// }
/// ```
pub fn poll(timeout: Option<Duration>) -> Result<bool> {
    poll_internal(timeout, &EventFilter)
}

/// Reads a single event.
///
/// This function will block until an event is received.
/// Use [poll](fn.poll.html) to check for ready events.
///
/// ```no_run
/// use crossterm::{Result, event::{read, poll, Event}};
/// use std::time::Duration;
///
/// fn main() -> Result<()> {
///     // poll maximal 1 second for an ready event.
///     if poll(Some(Duration::from_millis(1000)))? {
///         // read the ready event.
///         match read() {
///             Ok(event) => { println!("{:?}", event) }
///             _ => { }
///         }
///      }
///     Ok(())
/// }
/// ```
pub fn read() -> Result<Event> {
    match read_internal(&EventFilter)? {
        InternalEvent::Event(event) => Ok(event),
        #[cfg(unix)]
        _ => unreachable!(),
    }
}

/// Wakes up the sleeping `poll` function.
///
/// `poll` returns immediately with `Ok(false)`.
pub fn wake() {
    INTERNAL_EVENT_READER.read().wake();
}

/// Polls to check if there are any `InternalEvent`s that can be read withing the given duration.
pub(crate) fn poll_internal<F>(timeout: Option<Duration>, filter: &F) -> Result<bool>
where
    F: Filter,
{
    let (mut reader, timeout) = if let Some(timeout) = timeout {
        let poll_timeout = PollTimeout::new(Some(timeout));
        if let Some(reader) = INTERNAL_EVENT_READER.try_write_for(timeout) {
            (reader, poll_timeout.leftover())
        } else {
            return Ok(false);
        }
    } else {
        (INTERNAL_EVENT_READER.write(), None)
    };
    reader.poll(timeout, filter)
}

/// Reads a single `InternalEvent`.
pub(crate) fn read_internal<F>(filter: &F) -> Result<InternalEvent>
where
    F: Filter,
{
    let mut reader = INTERNAL_EVENT_READER.write();
    reader.read(filter)
}

/// A command that enables mouse event capturing.
///
/// Mouse events can be captured with [read](./fn.read.html)/[poll](./fn.poll.html).
pub struct EnableMouseCapture;

impl Command for EnableMouseCapture {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::enable_mouse_mode_csi_sequence()
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::windows::enable_mouse_capture()
    }
}

/// A command that disables mouse event capturing.
///
/// Mouse events can be captured with [read](./fn.read.html)/[poll](./fn.poll.html).
pub struct DisableMouseCapture;

impl Command for DisableMouseCapture {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::disable_mouse_mode_csi_sequence()
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::windows::disable_mouse_capture()
    }
}

/// Represents an event.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialOrd, PartialEq, Hash, Clone)]
pub enum Event {
    /// A single key or a combination of keys.
    Key(KeyEvent),
    /// A mouse event.
    Mouse(MouseEvent),
    /// An resize event with new dimensions after resize (columns, rows).
    Resize(u16, u16),
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
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
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
/// Encapsulates publicly available `Event` with additional internal
/// events that shouldn't be publicly available to the crate users.
#[derive(Debug, PartialOrd, PartialEq, Hash, Clone)]
pub(crate) enum InternalEvent {
    /// An event.
    Event(Event),
    /// A cursor position (`col`, `row`).
    #[cfg(unix)]
    CursorPosition(u16, u16),
}