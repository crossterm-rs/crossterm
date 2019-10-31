//! This is a UNIX specific implementation for input related action.

use std::sync::mpsc::Receiver;
use std::{char, sync::mpsc};

use crate::utils::Result;
use crate::write_cout;

use super::{
    super::{
        ansi::{disable_mouse_mode_csi_sequence, enable_mouse_mode_csi_sequence},
        sys::unix::internal_event_receiver,
        InputEvent, InternalEvent, KeyEvent,
    },
    Input,
};

pub(crate) struct UnixInput;

impl UnixInput {
    pub fn new() -> UnixInput {
        UnixInput {}
    }
}

impl Input for UnixInput {
    fn read_char(&self) -> Result<char> {
        let mut reader = self.read_sync();
        loop {
            if let Some(InputEvent::Keyboard(KeyEvent::Char(ch))) = reader.next() {
                return Ok(ch);
            }
        }
    }

    fn read_async(&self) -> AsyncReader {
        AsyncReader::new(None)
    }

    fn read_until_async(&self, delimiter: u8) -> AsyncReader {
        let sentinel = match delimiter {
            b'\n' | b'\r' => Some(KeyEvent::Enter),
            b'\x1B' => Some(KeyEvent::Esc),
            c if c.is_ascii() => Some(KeyEvent::Char(c as char)),
            _ => None,
        }
        .map(InputEvent::Keyboard);

        AsyncReader::new(sentinel)
    }

    fn read_sync(&self) -> SyncReader {
        SyncReader::new()
    }

    fn enable_mouse_mode(&self) -> Result<()> {
        write_cout!(enable_mouse_mode_csi_sequence())?;
        Ok(())
    }

    fn disable_mouse_mode(&self) -> Result<()> {
        write_cout!(disable_mouse_mode_csi_sequence())?;
        Ok(())
    }
}

/// An asynchronous input reader (not blocking).
///
/// `AsyncReader` implements the [`Iterator`](https://doc.rust-lang.org/std/iter/index.html#iterator)
/// trait. Documentation says:
///
/// > An iterator has a method, `next`, which when called, returns an `Option<Item>`. `next` will return
/// > `Some(Item)` as long as there are elements, and once they've all been exhausted, will return `None`
/// > to indicate that iteration is finished. Individual iterators may choose to resume iteration, and
/// > so calling `next` again may or may not eventually start returning `Some(Item)` again at some point.
///
/// `AsyncReader` is an individual iterator and it doesn't use `None` to indicate that the iteration is
/// finished. You can expect additional `Some(InputEvent)` after calling `next` even if you have already
/// received `None`.
///
/// # Notes
///
/// * It requires enabled raw mode (see the
///   [`crossterm_screen`](https://docs.rs/crossterm_screen/) crate documentation to learn more).
/// * A thread is spawned/reused to read the input.
/// * The reading thread is cleaned up when you drop the `AsyncReader`.
/// * See the [`SyncReader`](struct.SyncReader.html) if you want a blocking,
///   or a less resource hungry reader.
///
/// # Examples
///
/// ```no_run
/// use std::{thread, time::Duration};
///
/// use crossterm::{screen::RawScreen,  input::{input, InputEvent, KeyEvent}};
///
/// fn main() {
///     println!("Press 'ESC' to quit.");
///
///     // Enable raw mode and keep the `_raw` around otherwise the raw mode will be disabled
///     let _raw = RawScreen::into_raw_mode();
///
///     // Create an input from our screen
///     let input = input();
///
///     // Create an async reader
///     let mut reader = input.read_async();
///
///     loop {
///         if let Some(event) = reader.next() { // Not a blocking call
///             match event {
///                 InputEvent::Keyboard(KeyEvent::Esc) => {
///                     println!("Program closing ...");
///                     break;
///                  }
///                  InputEvent::Mouse(event) => { /* Mouse event */ }
///                  _ => { /* Other events */ }
///             }
///         }
///         thread::sleep(Duration::from_millis(50));
///     }
/// } // `reader` dropped <- thread cleaned up, `_raw` dropped <- raw mode disabled
/// ```
pub struct AsyncReader {
    rx: Option<Receiver<InternalEvent>>,
    stop_event: Option<InputEvent>,
}

impl AsyncReader {
    /// Creates a new `AsyncReader`.
    ///
    /// # Arguments
    ///
    /// * `stop_event` - if set, no more events will be produced if this exact event is reached.
    ///
    /// # Notes
    ///
    /// * A thread is spawned/reused to read the input.
    /// * The reading thread is cleaned up when you drop the `AsyncReader`.
    fn new(stop_event: Option<InputEvent>) -> AsyncReader {
        // TODO 1.0: Following expect is here to keep the API compatible (no Result)
        AsyncReader {
            rx: Some(internal_event_receiver().expect("Unable to get event receiver")),
            stop_event,
        }
    }

    // TODO If we we keep the Drop semantics, do we really need this in the public API? It's useless as
    //      there's no `start`, etc.
    /// Stops the input reader.
    ///
    /// # Notes
    ///
    /// * You don't need to call this method, because it will be automatically called when the
    ///   `AsyncReader` is dropped.
    pub fn stop(&mut self) {
        self.rx = None;
    }
}

impl Iterator for AsyncReader {
    type Item = InputEvent;

    /// Tries to read the next input event (not blocking).
    ///
    /// `None` doesn't mean that the iteration is finished. See the
    /// [`AsyncReader`](struct.AsyncReader.html) documentation for more information.
    fn next(&mut self) -> Option<Self::Item> {
        // TODO 1.0: This whole `InternalEvent` -> `InputEvent` mapping should be shared
        //           between UNIX & Windows implementations

        let ref mut rx = match self.rx.as_ref() {
            Some(rx) => rx,
            None => return None,
        };

        match rx.try_recv() {
            Ok(internal_event) => {
                let input_event = internal_event.into();

                if self.stop_event.is_some() && input_event == self.stop_event {
                    // Drop the receiver, stop event received
                    self.rx = None;
                }

                input_event
            }
            Err(mpsc::TryRecvError::Empty) => None,
            Err(mpsc::TryRecvError::Disconnected) => {
                // Sender dropped, drop the receiver
                self.rx = None;
                None
            }
        }
    }
}

/// A synchronous input reader (blocking).
///
/// `SyncReader` implements the [`Iterator`](https://doc.rust-lang.org/std/iter/index.html#iterator)
/// trait. Documentation says:
///
/// > An iterator has a method, `next`, which when called, returns an `Option<Item>`. `next` will return
/// > `Some(Item)` as long as there are elements, and once they've all been exhausted, will return `None`
/// > to indicate that iteration is finished. Individual iterators may choose to resume iteration, and
/// > so calling `next` again may or may not eventually start returning `Some(Item)` again at some point.
///
/// `SyncReader` is an individual iterator and it doesn't use `None` to indicate that the iteration is
/// finished. You can expect additional `Some(InputEvent)` after calling `next` even if you have already
/// received `None`. Unfortunately, `None` means that an error occurred, but you're free to call `next`
/// again. This behavior will be changed in the future to avoid errors consumption.
///
/// # Notes
///
/// * It requires enabled raw mode (see the
///   [`crossterm_screen`](https://docs.rs/crossterm_screen/) crate documentation to learn more).
/// * See the [`AsyncReader`](struct.AsyncReader.html) if you want a non blocking reader.
///
/// # Examples
///
/// ```no_run
/// use std::{thread, time::Duration};
///
/// use crossterm::{screen::RawScreen,  input::{input, InputEvent, KeyEvent}};
///
/// fn main() {
///     println!("Press 'ESC' to quit.");
///
///     // Enable raw mode and keep the `_raw` around otherwise the raw mode will be disabled
///     let _raw = RawScreen::into_raw_mode();
///
///     // Create an input from our screen
///     let input = input();
///
///     // Create a sync reader
///     let mut reader = input.read_sync();
///
///     loop {
///         if let Some(event) = reader.next() { // Blocking call
///             match event {
///                 InputEvent::Keyboard(KeyEvent::Esc) => {
///                     println!("Program closing ...");
///                     break;
///                  }
///                  InputEvent::Mouse(event) => { /* Mouse event */ }
///                  _ => { /* Other events */ }
///             }
///         }
///         thread::sleep(Duration::from_millis(50));
///     }
/// } // `_raw` dropped <- raw mode disabled
/// ```
pub struct SyncReader {
    rx: Option<Receiver<InternalEvent>>,
}

impl SyncReader {
    fn new() -> SyncReader {
        // TODO 1.0: Following expect is here to keep the API compatible (no Result)
        SyncReader {
            rx: Some(internal_event_receiver().expect("Unable to get event receiver")),
        }
    }
}

impl Iterator for SyncReader {
    type Item = InputEvent;

    /// Tries to read the next input event (blocking).
    ///
    /// `None` doesn't mean that the iteration is finished. See the
    /// [`SyncReader`](struct.SyncReader.html) documentation for more information.
    fn next(&mut self) -> Option<Self::Item> {
        // TODO 1.0: This whole `InternalEvent` -> `InputEvent` mapping should be shared
        //           between UNIX & Windows implementations

        let ref mut rx = match self.rx.as_ref() {
            Some(rx) => rx,
            None => return None,
        };

        match rx.recv() {
            Ok(internal_event) => internal_event.into(),
            Err(mpsc::RecvError) => {
                // Sender is dropped, drop the receiver
                self.rx = None;
                None
            }
        }
    }
}
