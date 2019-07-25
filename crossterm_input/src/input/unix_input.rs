//! This is a UNIX specific implementation for input related action.

use super::*;
use crate::sys::unix::{get_tty, read_char_raw};

use crossterm_utils::{csi, write_cout, Result};
use std::char;
use std::io::{Read, Write};

pub struct UnixInput;

impl UnixInput {
    pub fn new() -> UnixInput {
        UnixInput {}
    }
}

impl ITerminalInput for UnixInput {
    fn read_char(&self) -> io::Result<char> {
        read_char_raw()
    }

    fn read_async(&self) -> AsyncReader {
        AsyncReader::new(Box::new(move |event_tx, cancellation_token| {
            for i in get_tty().unwrap().bytes() {
                if event_tx.send(i.unwrap()).is_err() {
                    return;
                }

                if cancellation_token.load(Ordering::SeqCst) {
                    return;
                }
            }
        }))
    }

    fn read_until_async(&self, delimiter: u8) -> AsyncReader {
        AsyncReader::new(Box::new(move |event_tx, cancellation_token| {
            for byte in get_tty().unwrap().bytes() {
                let byte = byte.unwrap();
                let end_of_stream = byte == delimiter;
                let send_error = event_tx.send(byte).is_err();

                if end_of_stream || send_error || cancellation_token.load(Ordering::SeqCst) {
                    return;
                }
            }
        }))
    }

    fn read_sync(&self) -> SyncReader {
        SyncReader {
            source: Box::from(get_tty().unwrap()),
            leftover: None,
        }
    }

    fn enable_mouse_mode(&self) -> Result<()> {
        write_cout!(&format!(
            "{}h{}h{}h{}h",
            csi!("?1000"),
            csi!("?1002"),
            csi!("?1015"),
            csi!("?1006")
        ))?;
        Ok(())
    }

    fn disable_mouse_mode(&self) -> Result<()> {
        write_cout!(&format!(
            "{}l{}l{}l{}l",
            csi!("?1006"),
            csi!("?1015"),
            csi!("?1002"),
            csi!("?1000")
        ))?;
        Ok(())
    }
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

/// This type allows you to read input synchronously, which means that reading calls will block.
///
/// This type is an iterator, and can be used to iterate over input events.
///
/// If you don't want to block your calls use [AsyncReader](./LINK), which will read input on the background and queue it for you to read.
pub struct SyncReader {
    source: Box<std::fs::File>,
    leftover: Option<u8>,
}

impl Iterator for SyncReader {
    type Item = InputEvent;
    /// Read input from the user.
    ///
    /// If there are no keys pressed, this will be a blocking call until there is one.
    /// This will return `None` in case of a failure and `Some(InputEvent)` in case of an occurred input event.
    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Currently errors are consumed and converted to a `None`. Maybe we shouldn't be doing this?
        let source = &mut self.source;

        if let Some(c) = self.leftover {
            // we have a leftover byte, use it
            self.leftover = None;
            if let Ok(e) = parse_event(c, &mut source.bytes().flatten()) {
                return Some(e);
            } else {
                return None;
            }
        }

        // Here we read two bytes at a time. We need to distinguish between single ESC key presses,
        // and escape sequences (which start with ESC or a x1B byte). The idea is that if this is
        // an escape sequence, we will read multiple bytes (the first byte being ESC) but if this
        // is a single ESC keypress, we will only read a single byte.
        let mut buf = [0u8; 2];

        let res = match source.read(&mut buf) {
            Ok(0) => return None,
            Ok(1) => match buf[0] {
                b'\x1B' => return Some(InputEvent::Keyboard(KeyEvent::Esc)),
                c => {
                    if let Ok(e) = parse_event(c, &mut source.bytes().flatten()) {
                        return Some(e);
                    } else {
                        return None;
                    }
                }
            },
            Ok(2) => {
                let option_iter = &mut Some(buf[1]).into_iter();
                let iter = option_iter.map(|c| Ok(c)).chain(source.bytes());
                if let Ok(e) = parse_event(buf[0], &mut iter.flatten()) {
                    self.leftover = option_iter.next();
                    Some(e)
                } else {
                    None
                }
            }
            Ok(_) => unreachable!(),
            Err(_) => return None, /* maybe we should not throw away the error?*/
        };

        res
    }
}
