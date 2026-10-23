//! # Event
//!
//! The `event` module provides the functionality to read keyboard, mouse and terminal resize
//! events.
//!
//! * The [`read`](fn.read.html) function returns an [`Event`](enum.Event.html) immediately
//! (if available) or blocks until an [`Event`](enum.Event.html) is available.
//!
//! * The [`poll`](fn.poll.html) function allows you to check if there is or isn't an
//!   [`Event`](enum.Event.html) available
//! within the given period of time. In other words - if subsequent call to the
//! [`read`](fn.read.html) function will block or not.
//!
//! It's **not allowed** to call these functions from different threads or combine them with the
//! [`EventStream`](struct.EventStream.html). You're allowed to either:
//!
//! * use the [`read`](fn.read.html) & [`poll`](fn.poll.html) functions on any, but same, thread
//! * or the [`EventStream`](struct.EventStream.html).
//!
//! **Make sure to enable [raw mode](../terminal/index.html#raw-mode) in order for keyboard events
//! to work properly**
//!
//! ## Mouse and Focus Events
//!
//! Mouse and focus events are not enabled by default. You have to enable them with the
//! [`EnableMouseCapture`](struct.EnableMouseCapture.html) /
//! [`EnableFocusChange`](struct.EnableFocusChange.html) command. See [Command
//! API](../index.html#command-api) for more information.
//!
//! ## Examples
//!
//! Blocking read:
//!
//! ```no_run
//! #[cfg(feature = "bracketed-paste")]
//! use crossterm::event::{DisableBracketedPaste, EnableBracketedPaste};
//! use crossterm::{
//!     event::{
//!         DisableFocusChange, DisableMouseCapture, EnableFocusChange, EnableMouseCapture, Event,
//!         read,
//!     },
//!     execute,
//! };
//!
//! fn print_events() -> std::io::Result<()> {
//!     #[cfg(feature = "bracketed-paste")]
//!     execute!(std::io::stdout(), EnableBracketedPaste)?;
//!     execute!(std::io::stdout(), EnableFocusChange, EnableMouseCapture)?;
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
//!             _ => {}
//!         }
//!     }
//!     #[cfg(feature = "bracketed-paste")]
//!     execute!(std::io::stdout(), DisableBracketedPaste)?;
//!     execute!(std::io::stdout(), DisableFocusChange, DisableMouseCapture)?;
//!     Ok(())
//! }
//! ```
//!
//! Non-blocking read:
//!
//! ```no_run
//! use std::{io, time::Duration};
//!
//! #[cfg(feature = "bracketed-paste")]
//! use crossterm::event::{DisableBracketedPaste, EnableBracketedPaste};
//! use crossterm::{
//!     event::{
//!         DisableFocusChange, DisableMouseCapture, EnableFocusChange, EnableMouseCapture, Event,
//!         poll, read,
//!     },
//!     execute,
//! };
//!
//! fn print_events() -> io::Result<()> {
//!     #[cfg(feature = "bracketed-paste")]
//!     execute!(std::io::stdout(), EnableBracketedPaste)?;
//!     execute!(std::io::stdout(), EnableFocusChange, EnableMouseCapture)?;
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
//!                 _ => {}
//!             }
//!         } else {
//!             // Timeout expired and no `Event` is available
//!         }
//!     }
//!     #[cfg(feature = "bracketed-paste")]
//!     execute!(std::io::stdout(), DisableBracketedPaste)?;
//!     execute!(std::io::stdout(), DisableFocusChange, DisableMouseCapture)?;
//!     Ok(())
//! }
//! ```
//!
//! Check the [examples](https://github.com/crossterm-rs/crossterm/tree/master/examples) folder for more of
//! them (`event-*`).

/// Implements `is_*` predicates for unit and tuple enum variants.
///
/// Entries use the same left-to-right shape as `match` arms:
/// `Variant` or `Variant(..)`, followed by `=>` and the method name. The
/// method name is explicit because `macro_rules!` cannot construct an
/// identifier such as `is_focus_gained` from `FocusGained`.
///
/// Attributes on an entry are applied to the generated method. This keeps
/// feature-gated helpers, such as `Event::is_paste`, aligned with their
/// variants.
///
/// Variants whose payload needs to be compared belong in a handwritten
/// method instead; see the `KeyCode` helpers in `key_code.rs`. Generated methods are
/// `const`, inline, and `must_use`.
macro_rules! impl_is_variant {
    (
        $type:ident {
            $(
                $(#[$meta:meta])*
                $variant:ident $(($field:tt))? => $method:ident
            ),* $(,)?
        }
    ) => {
        impl $type {
            $(
                $(#[$meta])*
                #[doc = concat!(
                    "Returns `true` if this value is the `",
                    stringify!($type),
                    "::",
                    stringify!($variant),
                    "` variant. Returns `false` otherwise."
                )]
                #[inline]
                #[must_use]
                pub const fn $method(&self) -> bool {
                    matches!(self, Self::$variant $(($field))?)
                }
            )*
        }
    };
}

pub(crate) mod filter;
pub(crate) mod internal;
pub(crate) mod read;
pub(crate) mod source;
#[cfg(feature = "event-stream")]
pub(crate) mod stream;
pub(crate) mod sys;
pub(crate) mod timeout;

mod commands;
mod key_code;
mod key_event;
mod modifiers;
mod mouse;

#[cfg(feature = "event-stream")]
pub use stream::EventStream;

#[cfg(feature = "bracketed-paste")]
pub use commands::{DisableBracketedPaste, EnableBracketedPaste};
pub use commands::{
    DisableFocusChange, DisableMouseCapture, EnableFocusChange, EnableMouseCapture,
    KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};
pub use key_code::{KeyCode, MediaKeyCode, ModifierKeyCode};
pub use key_event::{KeyEvent, KeyEventKind, KeyEventState};
pub use modifiers::KeyModifiers;
pub use mouse::{MouseButton, MouseEvent, MouseEventKind};

use crate::event::{filter::EventFilter, internal::InternalEvent};
use std::time::Duration;

use std::hash::Hash;

/// Checks if there is an [`Event`](enum.Event.html) available.
///
/// Returns `Ok(true)` if an [`Event`](enum.Event.html) is available otherwise it returns
/// `Ok(false)`.
///
/// `Ok(true)` guarantees that subsequent call to the [`read`](fn.read.html) function
/// won't block.
///
/// # Arguments
///
/// * `timeout` - maximum waiting time for event availability
///
/// # Examples
///
/// Return immediately:
///
/// ```no_run
/// use crossterm::event::poll;
/// use std::{io, time::Duration};
///
/// fn is_event_available() -> io::Result<bool> {
///     // Zero duration says that the `poll` function must return immediately
///     // with an `Event` availability information
///     poll(Duration::from_secs(0))
/// }
/// ```
///
/// Wait up to 100ms:
///
/// ```no_run
/// use std::{io, time::Duration};
///
/// use crossterm::event::poll;
///
/// fn is_event_available() -> io::Result<bool> {
///     // Wait for an `Event` availability for 100ms. It returns immediately
///     // if an `Event` is/becomes available.
///     poll(Duration::from_millis(100))
/// }
/// ```
pub fn poll(timeout: Duration) -> std::io::Result<bool> {
    internal::poll(Some(timeout), &EventFilter)
}

/// Reads a single [`Event`](enum.Event.html).
///
/// This function blocks until an [`Event`](enum.Event.html) is available. Combine it with the
/// [`poll`](fn.poll.html) function to get non-blocking reads.
///
/// # Examples
///
/// Blocking read:
///
/// ```no_run
/// use crossterm::event::read;
/// use std::io;
///
/// fn print_events() -> io::Result<bool> {
///     loop {
///         // Blocks until an `Event` is available
///         println!("{:?}", read()?);
///     }
/// }
/// ```
///
/// Non-blocking read:
///
/// ```no_run
/// use std::io;
/// use std::time::Duration;
///
/// use crossterm::event::{poll, read};
///
/// fn print_events() -> io::Result<bool> {
///     loop {
///         if poll(Duration::from_millis(100))? {
///             // It's guaranteed that `read` won't block, because `poll` returned
///             // `Ok(true)`.
///             println!("{:?}", read()?);
///         } else {
///             // Timeout expired, no `Event` is available
///         }
///     }
/// }
/// ```
pub fn read() -> std::io::Result<Event> {
    match internal::read(&EventFilter)? {
        InternalEvent::Event(event) => Ok(event),
        #[cfg(unix)]
        _ => unreachable!(),
    }
}

/// Attempts to read a single [`Event`](enum.Event.html) without blocking the thread.
///
/// If no event is found, `None` is returned.
///
/// # Examples
///
/// ```no_run
/// use crossterm::event::{poll, try_read};
/// use std::{io, time::Duration};
///
/// fn print_all_events() -> io::Result<bool> {
///     loop {
///         if poll(Duration::from_millis(100))? {
///             // Fetch *all* available events at once
///             while let Some(event) = try_read() {
///                 // ...
///             }
///         }
///     }
/// }
/// ```
pub fn try_read() -> Option<Event> {
    match internal::try_read(&EventFilter) {
        Some(InternalEvent::Event(event)) => Some(event),
        None => None,
        #[cfg(unix)]
        _ => unreachable!(),
    }
}

/// Represents an event.
///
/// New variants may be added in minor releases as crossterm learns to report more of what
/// terminals send, so a `match` on `Event` needs a wildcard arm.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Hash)]
#[non_exhaustive]
pub enum Event {
    /// The terminal gained focus
    FocusGained,
    /// The terminal lost focus
    FocusLost,
    /// A single key event with additional pressed modifiers.
    Key(KeyEvent),
    /// A single mouse event with additional pressed modifiers.
    Mouse(MouseEvent),
    /// A string that was pasted into the terminal. Only emitted if bracketed paste has been
    /// enabled.
    #[cfg(feature = "bracketed-paste")]
    Paste(String),
    /// A resize event with new dimensions after resize (columns, rows).
    /// **Note** that resize events can occur in batches.
    Resize(u16, u16),
}

impl_is_variant! {
    Event {
        FocusGained => is_focus_gained,
        FocusLost => is_focus_lost,
        Key(..) => is_key,
        Mouse(..) => is_mouse,
        #[cfg(feature = "bracketed-paste")]
        Paste(..) => is_paste,
        Resize(..) => is_resize,
    }
}

impl Event {
    /// Returns `true` if the event is a key press event.
    ///
    /// This is useful for waiting for any key press event, regardless of the key that was pressed.
    ///
    /// Returns `false` for key release and repeat events (as well as for non-key events).
    ///
    /// # Examples
    ///
    /// The following code runs a loop that processes events until a key press event is encountered:
    ///
    /// ```no_run
    /// use crossterm::event;
    ///
    /// while !event::read()?.is_key_press() {
    ///     // ...
    /// }
    /// # Ok::<(), std::io::Error>(())
    /// ```
    #[inline]
    pub fn is_key_press(&self) -> bool {
        matches!(
            self,
            Event::Key(KeyEvent {
                kind: KeyEventKind::Press,
                ..
            })
        )
    }

    /// Returns `true` if the event is a key release event.
    #[inline]
    pub fn is_key_release(&self) -> bool {
        matches!(
            self,
            Event::Key(KeyEvent {
                kind: KeyEventKind::Release,
                ..
            })
        )
    }

    /// Returns `true` if the event is a key repeat event.
    #[inline]
    pub fn is_key_repeat(&self) -> bool {
        matches!(
            self,
            Event::Key(KeyEvent {
                kind: KeyEventKind::Repeat,
                ..
            })
        )
    }

    /// Returns the key event if the event is a key event, otherwise `None`.
    ///
    /// This is a convenience method that makes apps that only care about key events easier to
    /// write.
    ///
    /// # Examples
    ///
    /// The following code runs a loop that only processes key events:
    ///
    /// ```no_run
    /// use crossterm::event;
    ///
    /// while let Some(key_event) = event::read()?.as_key_event() {
    ///     // ...
    /// }
    /// # std::io::Result::Ok(())
    /// ```
    #[inline]
    pub fn as_key_event(&self) -> Option<KeyEvent> {
        match self {
            Event::Key(event) => Some(*event),
            _ => None,
        }
    }

    /// Returns an Option containing the KeyEvent if the event is a key press event.
    ///
    /// This is a convenience method that makes apps that only care about key press events, and not
    /// key release or repeat events (or non-key events), easier to write.
    ///
    /// Returns `None` for key release and repeat events (as well as for non-key events).
    ///
    /// # Examples
    ///
    /// The following code runs a loop that only processes key press events:
    ///
    /// ```no_run
    /// use crossterm::event;
    ///
    /// while let Ok(event) = event::read() {
    ///     if let Some(key) = event.as_key_press_event() {
    ///         // ...
    ///     }
    /// }
    #[inline]
    pub fn as_key_press_event(&self) -> Option<KeyEvent> {
        match self {
            Event::Key(event) if self.is_key_press() => Some(*event),
            _ => None,
        }
    }

    /// Returns an Option containing the `KeyEvent` if the event is a key release event.
    #[inline]
    pub fn as_key_release_event(&self) -> Option<KeyEvent> {
        match self {
            Event::Key(event) if self.is_key_release() => Some(*event),
            _ => None,
        }
    }

    /// Returns an Option containing the `KeyEvent` if the event is a key repeat event.
    #[inline]
    pub fn as_key_repeat_event(&self) -> Option<KeyEvent> {
        match self {
            Event::Key(event) if self.is_key_repeat() => Some(*event),
            _ => None,
        }
    }

    /// Returns the mouse event if the event is a mouse event, otherwise `None`.
    ///
    /// This is a convenience method that makes code which only cares about mouse events easier to
    /// write.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crossterm::event;
    ///
    /// while let Some(mouse_event) = event::read()?.as_mouse_event() {
    ///     // ...
    /// }
    /// # std::io::Result::Ok(())
    /// ```
    #[inline]
    pub fn as_mouse_event(&self) -> Option<MouseEvent> {
        match self {
            Event::Mouse(event) => Some(*event),
            _ => None,
        }
    }

    /// Returns the pasted string if the event is a paste event, otherwise `None`.
    ///
    /// This is a convenience method that makes code which only cares about paste events easier to
    /// write.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crossterm::event;
    ///
    /// while let Some(paste) = event::read()?.as_paste_event() {
    ///     // ...
    /// }
    /// # std::io::Result::Ok(())
    /// ```
    #[cfg(feature = "bracketed-paste")]
    #[inline]
    pub fn as_paste_event(&self) -> Option<&str> {
        match self {
            Event::Paste(paste) => Some(paste),
            _ => None,
        }
    }

    /// Returns the size as a tuple if the event is a resize event, otherwise `None`.
    ///
    /// This is a convenience method that makes code which only cares about resize events easier to
    /// write.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crossterm::event;
    ///
    /// while let Some((columns, rows)) = event::read()?.as_resize_event() {
    ///     // ...
    /// }
    /// # std::io::Result::Ok(())
    /// ```
    #[inline]
    pub fn as_resize_event(&self) -> Option<(u16, u16)> {
        match self {
            Event::Resize(columns, rows) => Some((*columns, *rows)),
            _ => None,
        }
    }
}

impl From<KeyEvent> for Event {
    /// Wraps a key event in [`Event::Key`].
    ///
    /// # Examples
    ///
    /// ```
    /// use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
    ///
    /// let key = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
    /// let event: Event = key.into();
    /// assert_eq!(event, Event::Key(key));
    /// ```
    fn from(event: KeyEvent) -> Self {
        Event::Key(event)
    }
}

impl From<MouseEvent> for Event {
    /// Wraps a mouse event in [`Event::Mouse`].
    ///
    /// # Examples
    ///
    /// ```
    /// use crossterm::event::{Event, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};
    ///
    /// let mouse = MouseEvent::new(MouseEventKind::Down(MouseButton::Left), 3, 7, KeyModifiers::NONE);
    /// let event: Event = mouse.into();
    /// assert_eq!(event, Event::Mouse(mouse));
    /// ```
    fn from(event: MouseEvent) -> Self {
        Event::Mouse(event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ESC_PRESSED: KeyEvent =
        KeyEvent::new_with_kind(KeyCode::Esc, KeyModifiers::empty(), KeyEventKind::Press);
    const ESC_RELEASED: KeyEvent =
        KeyEvent::new_with_kind(KeyCode::Esc, KeyModifiers::empty(), KeyEventKind::Release);
    const ESC_REPEAT: KeyEvent =
        KeyEvent::new_with_kind(KeyCode::Esc, KeyModifiers::empty(), KeyEventKind::Repeat);
    const MOUSE_CLICK: MouseEvent = MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: 1,
        row: 1,
        modifiers: KeyModifiers::empty(),
    };

    #[test]
    fn event_is() {
        let event = Event::FocusGained;
        assert!(event.is_focus_gained());
        assert!(event.is_focus_gained());
        assert!(!event.is_key());

        let event = Event::FocusLost;
        assert!(event.is_focus_lost());
        assert!(!event.is_focus_gained());
        assert!(!event.is_key());

        let event = Event::Resize(1, 1);
        assert!(event.is_resize());
        assert!(!event.is_key());

        let event = Event::Key(ESC_PRESSED);
        assert!(event.is_key());
        assert!(event.is_key_press());
        assert!(!event.is_key_release());
        assert!(!event.is_key_repeat());
        assert!(!event.is_focus_gained());

        let event = Event::Key(ESC_RELEASED);
        assert!(event.is_key());
        assert!(!event.is_key_press());
        assert!(event.is_key_release());
        assert!(!event.is_key_repeat());
        assert!(!event.is_focus_gained());

        let event = Event::Key(ESC_REPEAT);
        assert!(event.is_key());
        assert!(!event.is_key_press());
        assert!(!event.is_key_release());
        assert!(event.is_key_repeat());
        assert!(!event.is_focus_gained());

        let event = Event::Mouse(MOUSE_CLICK);
        assert!(event.is_mouse());
        assert!(!event.is_key());

        #[cfg(feature = "bracketed-paste")]
        {
            let event = Event::Paste("".to_string());
            assert!(event.is_paste());
            assert!(!event.is_key());
        }
    }

    #[test]
    fn event_as() {
        let event = Event::FocusGained;
        assert_eq!(event.as_key_event(), None);

        let event = Event::Key(ESC_PRESSED);
        assert_eq!(event.as_key_event(), Some(ESC_PRESSED));
        assert_eq!(event.as_key_press_event(), Some(ESC_PRESSED));
        assert_eq!(event.as_key_release_event(), None);
        assert_eq!(event.as_key_repeat_event(), None);
        assert_eq!(event.as_resize_event(), None);

        let event = Event::Key(ESC_RELEASED);
        assert_eq!(event.as_key_event(), Some(ESC_RELEASED));
        assert_eq!(event.as_key_release_event(), Some(ESC_RELEASED));
        assert_eq!(event.as_key_press_event(), None);
        assert_eq!(event.as_key_repeat_event(), None);
        assert_eq!(event.as_resize_event(), None);

        let event = Event::Key(ESC_REPEAT);
        assert_eq!(event.as_key_event(), Some(ESC_REPEAT));
        assert_eq!(event.as_key_repeat_event(), Some(ESC_REPEAT));
        assert_eq!(event.as_key_press_event(), None);
        assert_eq!(event.as_key_release_event(), None);
        assert_eq!(event.as_resize_event(), None);

        let event = Event::Resize(1, 1);
        assert_eq!(event.as_resize_event(), Some((1, 1)));
        assert_eq!(event.as_key_event(), None);

        let event = Event::Mouse(MOUSE_CLICK);
        assert_eq!(event.as_mouse_event(), Some(MOUSE_CLICK));
        assert_eq!(event.as_key_event(), None);

        #[cfg(feature = "bracketed-paste")]
        {
            let event = Event::Paste("".to_string());
            assert_eq!(event.as_paste_event(), Some(""));
            assert_eq!(event.as_key_event(), None);
        }
    }
}
