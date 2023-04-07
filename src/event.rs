//! # Event
//!
//! The `event` module provides the functionality to read keyboard, mouse and terminal resize events.
//!
//! * The [`read`](fn.read.html) function returns an [`Event`](enum.Event.html) immediately
//! (if available) or blocks until an [`Event`](enum.Event.html) is available.
//!
//! * The [`poll`](fn.poll.html) function allows you to check if there is or isn't an [`Event`](enum.Event.html) available
//! within the given period of time. In other words - if subsequent call to the [`read`](fn.read.html)
//! function will block or not.
//!
//! It's **not allowed** to call these functions from different threads or combine them with the
//! [`EventStream`](struct.EventStream.html). You're allowed to either:
//!
//! * use the [`read`](fn.read.html) & [`poll`](fn.poll.html) functions on any, but same, thread
//! * or the [`EventStream`](struct.EventStream.html).
//!
//! **Make sure to enable [raw mode](../terminal/index.html#raw-mode) in order for keyboard events to work properly**
//!
//! ## Mouse Events
//!
//! Mouse events are not enabled by default. You have to enable them with the
//! [`EnableMouseCapture`](struct.EnableMouseCapture.html) command. See [Command API](../index.html#command-api)
//! for more information.
//!
//! ## Examples
//!
//! Blocking read:
//!
//! ```no_run
//! use crossterm::event::{read, Event};
//!
//! fn print_events() -> crossterm::Result<()> {
//!     loop {
//!         // `read()` blocks until an `Event` is available
//!         match read()? {
//!             Event::FocusGained => println!("FocusGained"),
//!             Event::FocusLost => println!("FocusLost"),
//!             Event::Key(event) => println!("{:?}", event),
//!             Event::Mouse(event) => println!("{:?}", event),
//!             #[cfg(feature = "bracketed-paste")]
//!             Event::Paste(data) => println!("{:?}", data),
//!             Event::Resize(width, height) => println!("New size {}x{}", width, height),
//!         }
//!     }
//!     Ok(())
//! }
//! ```
//!
//! Non-blocking read:
//!
//! ```no_run
//! use std::time::Duration;
//!
//! use crossterm::event::{poll, read, Event};
//!
//! fn print_events() -> crossterm::Result<()> {
//!     loop {
//!         // `poll()` waits for an `Event` for a given time period
//!         if poll(Duration::from_millis(500))? {
//!             // It's guaranteed that the `read()` won't block when the `poll()`
//!             // function returns `true`
//!             match read()? {
//!                 Event::FocusGained => println!("FocusGained"),
//!                 Event::FocusLost => println!("FocusLost"),
//!                 Event::Key(event) => println!("{:?}", event),
//!                 Event::Mouse(event) => println!("{:?}", event),
//!                 #[cfg(feature = "bracketed-paste")]
//!                 Event::Paste(data) => println!("Pasted {:?}", data),
//!                 Event::Resize(width, height) => println!("New size {}x{}", width, height),
//!             }
//!         } else {
//!             // Timeout expired and no `Event` is available
//!         }
//!     }
//!     Ok(())
//! }
//! ```
//!
//! Check the [examples](https://github.com/crossterm-rs/crossterm/tree/master/examples) folder for more of
//! them (`event-*`).

use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{csi, Command};

#[cfg(feature = "events")]
pub(crate) mod filter;
#[cfg(feature = "events")]
mod read;
#[cfg(feature = "events")]
mod source;
#[cfg(feature = "event-stream")]
#[cfg(feature = "events")]
mod stream;
#[cfg(feature = "events")]
mod timeout;

#[cfg(feature = "events")]
mod events_api;

#[cfg(feature = "events")]
pub use events_api::*;

pub(crate) mod sys;

/// A command that enables mouse event capturing.
///
/// Mouse events can be captured with [read](./fn.read.html)/[poll](./fn.poll.html).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnableMouseCapture;

impl Command for EnableMouseCapture {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        f.write_str(concat!(
            // Normal tracking: Send mouse X & Y on button press and release
            csi!("?1000h"),
            // Button-event tracking: Report button motion events (dragging)
            csi!("?1002h"),
            // Any-event tracking: Report all motion events
            csi!("?1003h"),
            // RXVT mouse mode: Allows mouse coordinates of >223
            csi!("?1015h"),
            // SGR mouse mode: Allows mouse coordinates of >223, preferred over RXVT mode
            csi!("?1006h"),
        ))
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> crate::Result<()> {
        sys::windows::enable_mouse_capture()
    }

    #[cfg(windows)]
    fn is_ansi_code_supported(&self) -> bool {
        false
    }
}

/// A command that disables mouse event capturing.
///
/// Mouse events can be captured with [read](./fn.read.html)/[poll](./fn.poll.html).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisableMouseCapture;

impl Command for DisableMouseCapture {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        f.write_str(concat!(
            // The inverse commands of EnableMouseCapture, in reverse order.
            csi!("?1006l"),
            csi!("?1015l"),
            csi!("?1003l"),
            csi!("?1002l"),
            csi!("?1000l"),
        ))
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> crate::Result<()> {
        sys::windows::disable_mouse_capture()
    }

    #[cfg(windows)]
    fn is_ansi_code_supported(&self) -> bool {
        false
    }
}

/// A command that enables focus event emission.
///
/// It should be paired with [`DisableFocusChange`] at the end of execution.
///
/// Focus events can be captured with [read](./fn.read.html)/[poll](./fn.poll.html).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnableFocusChange;

impl Command for EnableFocusChange {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        f.write_str(csi!("?1004h"))
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> crate::Result<()> {
        // Focus events are always enabled on Windows
        Ok(())
    }
}

/// A command that disables focus event emission.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisableFocusChange;

impl Command for DisableFocusChange {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        f.write_str(csi!("?1004l"))
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> crate::Result<()> {
        // Focus events can't be disabled on Windows
        Ok(())
    }
}

/// A command that enables [bracketed paste mode](https://en.wikipedia.org/wiki/Bracketed-paste).
///
/// It should be paired with [`DisableBracketedPaste`] at the end of execution.
///
/// This is not supported in older Windows terminals without
/// [virtual terminal sequences](https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences).
#[cfg(feature = "bracketed-paste")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnableBracketedPaste;

#[cfg(feature = "bracketed-paste")]
impl Command for EnableBracketedPaste {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        f.write_str(csi!("?2004h"))
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "Bracketed paste not implemented in the legacy Windows API.",
        ))
    }
}

/// A command that disables bracketed paste mode.
#[cfg(feature = "bracketed-paste")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisableBracketedPaste;

#[cfg(feature = "bracketed-paste")]
impl Command for DisableBracketedPaste {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        f.write_str(csi!("?2004l"))
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        Ok(())
    }
}
