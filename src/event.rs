#![deny(unused_imports, unused_must_use)]

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
//! On UNIX we can use [MIO](https://docs.rs/mio/) to poll for event readiness.
//! However, for windows we use a delayed spinning loop that checks for `GetNumberOfConsoleInputEvents` and tells based on that if an event is ready to be read.
//! In the future we are probably going to improve this by using `Semaphore Objects`.
//!
//! `poll` and `read` are static functions that both aquire an underlying lock to crossterms input system.
//! You mustn't call `poll` from two threads because this can cause a deadlock.
//! However, `poll` and `read` can be called independently without influencing each other.

use std::time::Duration;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use lazy_static::lazy_static;
use parking_lot::RwLock;
use poll::EventPoll;

use crate::{Command, Result};

use timeout::PollTimeout;

mod ansi;
pub(crate) mod filter;
mod poll;
mod read;
mod source;
mod sys;
mod timeout;

lazy_static! {
    /// Static instance of `EventReader`.
    /// This needs to be static because there can be one internal event reader.
    static ref EVENT_READER: RwLock<read::EventReader> = { RwLock::new(read::EventReader::default()) };
}

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
    let (mut reader, timeout) = if let Some(timeout) = timeout {
        let poll_timeout = PollTimeout::new(Some(timeout));
        if let Some(reader) = EVENT_READER.try_write_for(timeout) {
            (reader, poll_timeout.leftover())
        } else {
            return Ok(false);
        }
    } else {
        (EVENT_READER.write(), None)
    };
    reader.poll(timeout)
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
    let mut reader = EVENT_READER.write();
    reader.read(filter::EventFilter)
}

/// Wakes up the sleeping `poll` function.
///
/// `poll` returns immediately with `Ok(false)`.
pub fn wake() {
    INTERNAL_EVENT_READER.read().wake();
}

/// Polls to check if there are any `InternalEvent`s that can be read withing the given duration.
pub(crate) fn poll_internal(timeout: Option<Duration>) -> Result<bool> {
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
    reader.poll(timeout)
}

/// Reads a single `InternalEvent`.
pub(crate) fn read_internal(mask: impl filter::Filter) -> Result<InternalEvent> {
    let mut reader = INTERNAL_EVENT_READER.write();
    reader.read(mask)
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
        sys::winapi::enable_mouse_capture()
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
        sys::winapi::disable_mouse_capture()
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

#[cfg(test)]
mod tests {
    use std::{
        sync::mpsc::{channel, Sender},
        thread,
        thread::JoinHandle,
        time::Duration,
    };

    use crate::event::filter::{Filter, InternalEventFilter};
    use crate::event::read::InternalEventReader;

    use super::{
        poll, poll_internal, read, read_internal,
        source::{fake::FakeEventSource, EventSource},
        Event, InternalEvent, KeyEvent,
    };

    #[test]
    fn test_internal_poll_with_timeout_should_return() {
        let poll = internal_event_polling_thread(
            Some(Duration::from_millis(200)),
            true,
            InternalEventFilter,
        );

        sleep_thread_millis(100);

        poll.event_sender.send(test_internal_key()).unwrap();

        let (poll_result, read) = poll.handle.join().unwrap();

        assert_eq!(poll_result, true);
        assert_eq!(read, Some(test_internal_key()));
    }

    #[test]
    fn test_internal_poll_with_timeout_should_not_return() {
        let poll = internal_event_polling_thread(
            Some(Duration::from_millis(100)),
            true,
            InternalEventFilter,
        );

        sleep_thread_millis(200);

        let (poll_result, read) = poll.handle.join().unwrap();

        assert_eq!(poll_result, false);
        assert_eq!(read, None);
    }

    #[test]
    fn test_internal_poll_without_timeout_should_return() {
        // spin up a thread waiting 2 seconds for input.
        let poll = internal_event_polling_thread(None, true, InternalEventFilter);

        poll.event_sender.send(test_internal_key()).unwrap();

        let (poll_result, read) = poll.handle.join().unwrap();

        assert_eq!(poll_result, true);
        assert_eq!(read, Some(test_internal_key()));
    }

    #[test]
    fn test_poll_with_timeout_should_return() {
        let poll = event_polling_thread(Some(Duration::from_millis(200)));

        sleep_thread_millis(50);

        poll.event_sender.send(test_internal_key()).unwrap();

        let (poll_result, read) = poll.handle.join().unwrap();

        assert_eq!(poll_result, true);
        assert_eq!(
            Some(InternalEvent::Event(read.unwrap())),
            Some(test_internal_key())
        );
    }

    #[test]
    fn test_poll_with_timeout_should_not_return() {
        let poll = event_polling_thread(Some(Duration::from_millis(100)));

        sleep_thread_millis(200);

        let (poll_result, read) = poll.handle.join().unwrap();

        assert_eq!(poll_result, false);
        assert_eq!(read, None);
    }

    #[test]
    fn test_poll_without_timeout_should_return() {
        let poll = event_polling_thread(None);

        poll.event_sender.send(test_internal_key()).unwrap();

        let (poll_result, read) = poll.handle.join().unwrap();

        assert_eq!(poll_result, true);
        assert_eq!(
            Some(InternalEvent::Event(read.unwrap())),
            Some(test_internal_key())
        );
    }

    #[test]
    #[cfg(unix)]
    fn test_event_should_not_thrown_away() {
        // first sent cursor position event, and try to poll with `EventReader`
        let poll = event_polling_thread(Some(Duration::from_millis(100)));

        poll.event_sender
            .send(InternalEvent::CursorPosition(5, 5))
            .unwrap();

        let (poll_result, read) = poll.handle.join().unwrap();

        assert_eq!(poll_result, false);
        assert_eq!(read, None);

        // then try to read with `InternalEventReader`, the cursor position event should still be in cache.
        let internal_poll = internal_event_polling_thread(
            Some(Duration::from_millis(100)),
            false,
            InternalEventFilter,
        );

        let (poll_result, read) = internal_poll.handle.join().unwrap();

        assert_eq!(poll_result, true);
        assert_eq!(read, Some(InternalEvent::CursorPosition(5, 5)));
    }

    /// Returns the handle to the thread that polls for input as long as the given duration and the sender to trigger the the thread to read the event.
    fn internal_event_polling_thread(
        timeout: Option<Duration>,
        set_fake_source: bool,
        filter: impl Filter,
    ) -> PollThreadHandleStub<InternalEvent> {
        let (event_sender, event_receiver) = channel();

        let handle = thread::spawn(move || {
            if set_fake_source {
                swap_event_source(Box::from(FakeEventSource::new(event_receiver)));
            }

            let poll_result = poll_internal(timeout).unwrap();

            let read = if poll_result {
                Some(read_internal(filter).unwrap())
            } else {
                None
            };

            (poll_result, read)
        });

        PollThreadHandleStub {
            handle,
            event_sender,
        }
    }

    fn event_polling_thread(timeout: Option<Duration>) -> PollThreadHandleStub<Event> {
        let (event_sender, event_receiver) = channel();

        let handle = thread::spawn(move || {
            swap_event_source(Box::from(FakeEventSource::new(event_receiver)));

            let poll_result = poll(timeout).unwrap();

            let read = if poll_result {
                Some(read().unwrap())
            } else {
                None
            };

            (poll_result, read)
        });

        PollThreadHandleStub {
            handle,
            event_sender,
        }
    }

    struct PollThreadHandleStub<T> {
        handle: JoinHandle<(bool, Option<T>)>,
        event_sender: Sender<InternalEvent>,
    }

    fn swap_event_source(new: Box<dyn EventSource>) {
        let mut reader = super::INTERNAL_EVENT_READER.write();
        *reader = InternalEventReader::new(new);
    }

    fn sleep_thread_millis(duration: u64) {
        thread::sleep(Duration::from_millis(duration));
    }

    fn test_internal_key() -> InternalEvent {
        InternalEvent::Event(Event::Key(KeyEvent::Char('q')))
    }
}
