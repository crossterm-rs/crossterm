//! This is a UNIX specific implementation for input related action.

use std::char;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::{self, Receiver, Sender},
    Arc,
};
use std::{
    io::{self, Read},
    str, thread,
};

use crossterm_utils::{csi, write_cout, ErrorKind, Result};

use crate::sys::unix::{get_tty, read_char_raw};

use super::{ITerminalInput, InputEvent, KeyEvent, MouseButton, MouseEvent};

pub struct UnixInput;

impl UnixInput {
    pub fn new() -> UnixInput {
        UnixInput {}
    }
}

impl ITerminalInput for UnixInput {
    fn read_char(&self) -> Result<char> {
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
    pub fn new(function: Box<dyn Fn(&Sender<u8>, &Arc<AtomicBool>) + Send>) -> AsyncReader {
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
    pub fn stop(&mut self) {
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
        self.stop();
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

/// Parse an Event from `item` and possibly subsequent bytes through `iter`.
pub(crate) fn parse_event<I>(item: u8, iter: &mut I) -> Result<InputEvent>
where
    I: Iterator<Item = u8>,
{
    let error = ErrorKind::IoError(io::Error::new(
        io::ErrorKind::Other,
        "Could not parse an event",
    ));
    let input_event = match item {
        b'\x1B' => {
            let a = iter.next();
            // This is an escape character, leading a control sequence.
            match a {
                Some(b'O') => {
                    match iter.next() {
                        // F1-F4
                        Some(val @ b'P'..=b'S') => {
                            InputEvent::Keyboard(KeyEvent::F(1 + val - b'P'))
                        }
                        _ => return Err(error),
                    }
                }
                Some(b'[') => {
                    // This is a CSI sequence.
                    parse_csi(iter)
                }
                Some(b'\x1B') => InputEvent::Keyboard(KeyEvent::Esc),
                Some(c) => {
                    let ch = parse_utf8_char(c, iter);
                    InputEvent::Keyboard(KeyEvent::Alt(ch?))
                }
                None => InputEvent::Keyboard(KeyEvent::Esc),
            }
        }
        b'\r' | b'\n' => InputEvent::Keyboard(KeyEvent::Enter),
        b'\t' => InputEvent::Keyboard(KeyEvent::Tab),
        b'\x7F' => InputEvent::Keyboard(KeyEvent::Backspace),
        c @ b'\x01'..=b'\x1A' => {
            InputEvent::Keyboard(KeyEvent::Ctrl((c as u8 - 0x1 + b'a') as char))
        }
        c @ b'\x1C'..=b'\x1F' => {
            InputEvent::Keyboard(KeyEvent::Ctrl((c as u8 - 0x1C + b'4') as char))
        }
        b'\0' => InputEvent::Keyboard(KeyEvent::Null),
        c => {
            let ch = parse_utf8_char(c, iter);
            InputEvent::Keyboard(KeyEvent::Char(ch?))
        }
    };

    Ok(input_event)
}

/// Parses a CSI sequence, just after reading ^[
/// Returns Event::Unknown if an unrecognized sequence is found.
/// Most of this parsing code is been taken over from 'termion`.
fn parse_csi<I>(iter: &mut I) -> InputEvent
where
    I: Iterator<Item = u8>,
{
    match iter.next() {
        Some(b'[') => match iter.next() {
            // NOTE (@imdaveho): cannot find when this occurs;
            // having another '[' after ESC[ not a likely scenario
            Some(val @ b'A'..=b'E') => InputEvent::Keyboard(KeyEvent::F(1 + val - b'A')),
            _ => InputEvent::Unknown,
        },
        Some(b'D') => InputEvent::Keyboard(KeyEvent::Left),
        Some(b'C') => InputEvent::Keyboard(KeyEvent::Right),
        Some(b'A') => InputEvent::Keyboard(KeyEvent::Up),
        Some(b'B') => InputEvent::Keyboard(KeyEvent::Down),
        Some(b'H') => InputEvent::Keyboard(KeyEvent::Home),
        Some(b'F') => InputEvent::Keyboard(KeyEvent::End),
        Some(b'Z') => InputEvent::Keyboard(KeyEvent::BackTab),
        Some(b'M') => {
            // X10 emulation mouse encoding: ESC [ CB Cx Cy (6 characters only).
            // NOTE (@imdaveho): cannot find documentation on this
            let mut next = || iter.next().unwrap();

            let cb = next() as i8 - 32;
            // (1, 1) are the coords for upper left.
            let cx = next().saturating_sub(32) as u16;
            let cy = next().saturating_sub(32) as u16;

            InputEvent::Mouse(match cb & 0b11 {
                0 => {
                    if cb & 0x40 != 0 {
                        MouseEvent::Press(MouseButton::WheelUp, cx, cy)
                    } else {
                        MouseEvent::Press(MouseButton::Left, cx, cy)
                    }
                }
                1 => {
                    if cb & 0x40 != 0 {
                        MouseEvent::Press(MouseButton::WheelDown, cx, cy)
                    } else {
                        MouseEvent::Press(MouseButton::Middle, cx, cy)
                    }
                }
                2 => MouseEvent::Press(MouseButton::Right, cx, cy),
                3 => MouseEvent::Release(cx, cy),
                _ => MouseEvent::Unknown,
            })
        }
        Some(b'<') => {
            // xterm mouse handling:
            // ESC [ < Cb ; Cx ; Cy (;) (M or m)
            let mut buf = Vec::new();
            let mut c = iter.next().unwrap();
            while match c {
                b'm' | b'M' => false,
                _ => true,
            } {
                buf.push(c);
                c = iter.next().unwrap();
            }
            let str_buf = String::from_utf8(buf).unwrap();
            let nums = &mut str_buf.split(';');

            let cb = nums.next().unwrap().parse::<u16>().unwrap();
            let cx = nums.next().unwrap().parse::<u16>().unwrap();
            let cy = nums.next().unwrap().parse::<u16>().unwrap();

            match cb {
                0..=2 | 64..=65 => {
                    let button = match cb {
                        0 => MouseButton::Left,
                        1 => MouseButton::Middle,
                        2 => MouseButton::Right,
                        64 => MouseButton::WheelUp,
                        65 => MouseButton::WheelDown,
                        _ => unreachable!(),
                    };
                    match c {
                        b'M' => InputEvent::Mouse(MouseEvent::Press(button, cx, cy)),
                        b'm' => InputEvent::Mouse(MouseEvent::Release(cx, cy)),
                        _ => InputEvent::Unknown,
                    }
                }
                32 => InputEvent::Mouse(MouseEvent::Hold(cx, cy)),
                3 => InputEvent::Mouse(MouseEvent::Release(cx, cy)),
                _ => InputEvent::Unknown,
            }
        }
        Some(c @ b'0'..=b'9') => {
            // Numbered escape code.
            let mut buf = Vec::new();
            buf.push(c);
            let mut character = iter.next().unwrap();

            // The final byte of a CSI sequence can be in the range 64-126, so
            // let's keep reading anything else.
            while character < 64 || character > 126 {
                buf.push(character);
                character = iter.next().unwrap();
            }

            match character {
                // rxvt mouse encoding:
                // ESC [ Cb ; Cx ; Cy ; M
                b'M' => {
                    let str_buf = String::from_utf8(buf).unwrap();

                    let nums: Vec<u16> = str_buf.split(';').map(|n| n.parse().unwrap()).collect();

                    let cb = nums[0];
                    let cx = nums[1];
                    let cy = nums[2];

                    let event = match cb {
                        32 => MouseEvent::Press(MouseButton::Left, cx, cy),
                        33 => MouseEvent::Press(MouseButton::Middle, cx, cy),
                        34 => MouseEvent::Press(MouseButton::Right, cx, cy),
                        35 => MouseEvent::Release(cx, cy),
                        64 => MouseEvent::Hold(cx, cy),
                        96 | 97 => MouseEvent::Press(MouseButton::WheelUp, cx, cy),
                        _ => MouseEvent::Unknown,
                    };

                    InputEvent::Mouse(event)
                }
                // Special key code.
                b'~' => {
                    let str_buf = String::from_utf8(buf).unwrap();

                    // This CSI sequence can be a list of semicolon-separated numbers.
                    let nums: Vec<u8> = str_buf.split(';').map(|n| n.parse().unwrap()).collect();

                    if nums.is_empty() {
                        return InputEvent::Unknown;
                    }

                    // TODO: handle multiple values for key modifiers (ex: values [3, 2] means Shift+Delete)
                    if nums.len() > 1 {
                        return InputEvent::Unknown;
                    }

                    match nums[0] {
                        1 | 7 => InputEvent::Keyboard(KeyEvent::Home),
                        2 => InputEvent::Keyboard(KeyEvent::Insert),
                        3 => InputEvent::Keyboard(KeyEvent::Delete),
                        4 | 8 => InputEvent::Keyboard(KeyEvent::End),
                        5 => InputEvent::Keyboard(KeyEvent::PageUp),
                        6 => InputEvent::Keyboard(KeyEvent::PageDown),
                        v @ 11..=15 => InputEvent::Keyboard(KeyEvent::F(v - 10)),
                        v @ 17..=21 => InputEvent::Keyboard(KeyEvent::F(v - 11)),
                        v @ 23..=24 => InputEvent::Keyboard(KeyEvent::F(v - 12)),
                        _ => InputEvent::Unknown,
                    }
                }
                e => match (buf.last().unwrap(), e) {
                    (53, 65) => InputEvent::Keyboard(KeyEvent::CtrlUp),
                    (53, 66) => InputEvent::Keyboard(KeyEvent::CtrlDown),
                    (53, 67) => InputEvent::Keyboard(KeyEvent::CtrlRight),
                    (53, 68) => InputEvent::Keyboard(KeyEvent::CtrlLeft),
                    (50, 65) => InputEvent::Keyboard(KeyEvent::ShiftUp),
                    (50, 66) => InputEvent::Keyboard(KeyEvent::ShiftDown),
                    (50, 67) => InputEvent::Keyboard(KeyEvent::ShiftRight),
                    (50, 68) => InputEvent::Keyboard(KeyEvent::ShiftLeft),
                    _ => InputEvent::Unknown,
                },
            }
        }
        _ => InputEvent::Unknown,
    }
}

/// Parse `c` as either a single byte ASCII char or a variable size UTF-8 char.
fn parse_utf8_char<I>(c: u8, iter: &mut I) -> Result<char>
where
    I: Iterator<Item = u8>,
{
    let error = Err(ErrorKind::IoError(io::Error::new(
        io::ErrorKind::Other,
        "Input character is not valid UTF-8",
    )));

    if c.is_ascii() {
        Ok(c as char)
    } else {
        let mut bytes = Vec::new();
        bytes.push(c);

        while let Some(next) = iter.next() {
            bytes.push(next);
            if let Ok(st) = str::from_utf8(&bytes) {
                return Ok(st.chars().next().unwrap());
            }
            if bytes.len() >= 4 {
                return error;
            }
        }

        return error;
    }
}

#[cfg(test)]
mod tests {
    use super::parse_utf8_char;

    #[test]
    fn test_parse_utf8() {
        let st = "abcéŷ¤£€ù%323";
        let ref mut bytes = st.bytes();
        let chars = st.chars();
        for c in chars {
            let b = bytes.next().unwrap();
            assert_eq!(c, parse_utf8_char(b, bytes).unwrap());
        }
    }
}
