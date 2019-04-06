//! This is a UNIX specific implementation for input related action.

use super::*;
use crate::sys::unix::{get_tty, read_char_raw};

use crossterm_utils::TerminalOutput;
use std::char;
use std::io::Read;

pub struct UnixInput;

impl UnixInput {
    pub fn new() -> UnixInput {
        UnixInput {}
    }
}

impl ITerminalInput for UnixInput {
    fn read_char(&self, stdout: &Option<&Arc<TerminalOutput>>) -> io::Result<char> {
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

    fn read_sync(&self) -> SyncReader {
        SyncReader {
            bytes: Box::new(get_tty().unwrap().bytes().flatten()),
        }
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

    fn enable_mouse_mode(&self, stdout: &Option<&Arc<TerminalOutput>>) -> io::Result<()> {
        write_cout!(
            stdout,
            &format!(
                "{}h{}h{}h{}h",
                csi!("?1000"),
                csi!("?1002"),
                csi!("?1015"),
                csi!("?1006")
            ),
        )?;
        Ok(())
    }

    fn disable_mouse_mode(&self, stdout: &Option<&Arc<TerminalOutput>>) -> io::Result<()> {
        write_cout!(
            stdout,
            &format!(
                "{}l{}l{}l{}l",
                csi!("?1006"),
                csi!("?1015"),
                csi!("?1002"),
                csi!("?1000")
            ),
        )?;
        Ok(())
    }
}

/// This type allows you to read input synchronously, which means that reading call will be blocking ones.
///
/// This type is an iterator, and could be used to iterate over input events.
///
/// If you don't want to block your calls use [AsyncReader](./LINK), which will read input on the background and queue it for you to read.
pub struct SyncReader {
    bytes: Box<Iterator<Item = u8>>,
}

impl Iterator for SyncReader {
    type Item = InputEvent;
    /// Read input from the user.
    ///
    /// If there are no keys pressed this will be a blocking call until there are.
    /// This will return `None` in case of a failure and `Some(InputEvent) in case of an occurred input event.`
    fn next(&mut self) -> Option<Self::Item> {
        let mut iterator = self.bytes.as_mut();
        match iterator.next() {
            Some(byte) => {
                if let Ok(event) = parse_event(byte, &mut iterator) {
                    Some(event)
                } else {
                    None
                }
            }
            None => None,
        }
    }
}
