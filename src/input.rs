#![deny(unused_imports, unused_must_use)]

//! # Input
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
//! on the terminal screen.

use std::time::Duration;

use crate::{input::event_pool::EventPool, Command, Result};

pub use self::{
    event_source::{fake::FakeEventSource, EventSource},
    events::{Event, KeyEvent, MouseButton, MouseEvent},
};

mod ansi;
mod event_poll;
mod event_reader;
mod event_source;
mod sys;

pub(crate) mod event_pool;
pub(crate) mod events;
pub(crate) mod poll_timeout;

/// Polls during an given duration for ready events.
///
/// This function takes in an optional duration.
/// * `None`: will block indefinitely until an event is read.
/// * `Some(duration)`: will block for the given duration.
///
/// The following value can be returned:
/// * `Ok(true)`: in case an event is ready.
/// * `Ok(false)`: in case the given duration is elapsed.
/// * `Err(err)`: in case of an error.
///
/// An ready event can be read with [read](LINK)
/// ```no_run
/// use std::time::Duration;
/// use crossterm::{Result, input::poll};
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
    let mut lock = EventPool::get_mut();
    lock.pool().poll(timeout)
}

/// Reads a single event.
///
/// This function will block until an event is received.
/// Use [poll](LINK) to check for ready events.
///
/// ```no_run
/// use crossterm::{Result, input::{read, poll, Event}};
/// use std::time::Duration;
///
/// fn main() -> Result<()> {
///     // poll maximal 1 second for an ready event.
///     if poll(Some(Duration::from_millis(1000)))? {
///         // read the ready event.
///         match read() {
///             Ok(Event(event)) => { println!("{:?}", event) }
///             _ => { }
///         }
///      }
///     Ok(())
/// }
/// ```
pub fn read() -> Result<Event> {
    let mut lock = EventPool::get_mut();
    lock.pool().read()
}

/// Changes the default `EventSource` to the given `EventSource`.
///
/// This might be usefull for testing.
/// See [FakeEventSource](LINK) for more information.
pub fn swap_event_source(new: Box<dyn EventSource>) {
    let mut lock = EventPool::get_mut();
    lock.pool().swap_event_source(new);
}

/// A command that enables mouse mode
///
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

/// A command that disables mouse event monitoring.
///
/// Mouse events will be produced by the
/// [`AsyncReader`](struct.AsyncReader.html)/[`SyncReader`](struct.SyncReader.html).
///
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
