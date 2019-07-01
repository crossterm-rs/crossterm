//! A module that contains all the actions related to reading input from the terminal.
//! Like reading a line, reading a character and reading asynchronously.

use super::*;
use std::{io, str};

/// Allows you to read user input.
///
/// # Features:
///
/// - Read character
/// - Read line
/// - Read async
/// - Read async until
/// - Read sync
/// - Wait for key event (terminal pause)
///
/// Check `/examples/` in the library for more specific examples.
pub struct TerminalInput {
    #[cfg(windows)]
    input: WindowsInput,
    #[cfg(unix)]
    input: UnixInput,
}

impl TerminalInput {
    /// Create a new instance of `TerminalInput` whereon input related actions could be performed.
    pub fn new() -> TerminalInput {
        #[cfg(windows)]
        let input = WindowsInput::new();

        #[cfg(unix)]
        let input = UnixInput::new();

        TerminalInput { input }
    }

    /// Read one line from the user input.
    ///
    /// # Remark
    /// This function is not work when raw screen is turned on.
    /// When you do want to read a line in raw mode please, checkout `read_async`, `read_async_until` or `read_sync`.
    /// Not sure what 'raw mode' is, checkout the 'crossterm_screen' crate.
    ///
    /// # Example
    /// ```rust
    /// let input = input();
    ///  match input.read_line() {
    ///     Ok(s) => println!("string typed: {}", s),
    ///     Err(e) => println!("error: {}", e),
    ///  }
    /// ```
    pub fn read_line(&self) -> io::Result<String> {
        let mut rv = String::new();
        io::stdin().read_line(&mut rv)?;
        let len = rv.trim_right_matches(&['\r', '\n'][..]).len();
        rv.truncate(len);
        Ok(rv)
    }

    /// Read one character from the user input
    ///
    /// ```rust
    /// let input = input();
    ///
    ///  match input.read_char() {
    ///     Ok(c) => println!("character pressed: {}", c),
    ///     Err(e) => println!("error: {}", e),
    ///   }
    /// ```
    pub fn read_char(&self) -> io::Result<char> {
        self.input.read_char()
    }

    /// Read the input asynchronously, which means that input events are gathered on the background and will be queued for you to read.
    ///
    /// If you want a blocking, or less resource consuming read to happen use `read_sync()`, this will leave a way all the thread and queueing and will be a blocking read.
    ///
    /// This is the same as `read_async()` but stops reading when a certain character is hit.
    ///
    /// # Remarks
    /// - Readings won't be blocking calls.
    ///   A thread will be fired to read input, on unix systems from TTY and on windows WinApi
    ///   `ReadConsoleW` will be used.
    /// - Input events read from the user will be queued on a MPSC-channel.
    /// - The reading thread will be cleaned up when it drops.
    /// - Requires 'raw screen to be enabled'.
    ///   Not sure what this is? Please checkout the 'crossterm_screen' crate.
    ///
    /// # Examples
    /// Please checkout the example folder in the repository.
    pub fn read_async(&self) -> AsyncReader {
        self.input.read_async()
    }

    /// Read the input asynchronously until a certain character is hit, which means that input events are gathered on the background and will be queued for you to read.
    ///
    /// If you want a blocking or less resource consuming read to happen, use `read_sync()`. This will leave alone the background thread and queues and will be a blocking read.
    ///
    /// This is the same as `read_async()` but stops reading when a certain character is hit.
    ///
    /// # Remarks
    /// - Readings won't be blocking calls.
    ///   A thread will be fired to read input, on unix systems from TTY and on windows WinApi
    ///   `ReadConsoleW` will be used.
    /// - Input events read from the user will be queued on a MPSC-channel.
    /// - The reading thread will be cleaned up when it drops.
    /// - Requires 'raw screen to be enabled'.
    ///   Not sure what this is? Please checkout the 'crossterm_screen' crate.
    ///
    /// # Examples
    /// Please checkout the example folder in the repository.
    pub fn read_until_async(&self, delimiter: u8) -> AsyncReader {
        self.input.read_until_async(delimiter)
    }

    /// Read the input synchronously from the user, which means that reading calls will block.
    /// It also uses less resources than the `AsyncReader` because the background thread and queues are left alone.
    ///
    /// Consider using `read_async` if you don't want the reading call to block your program.
    ///
    /// # Remark
    /// - Readings will be blocking calls.
    ///
    /// # Examples
    /// Please checkout the example folder in the repository.
    pub fn read_sync(&self) -> SyncReader {
        self.input.read_sync()
    }

    /// Enable mouse events to be captured.
    ///
    /// When enabling mouse input, you will be able to capture mouse movements, pressed buttons, and locations.
    ///
    /// # Remark
    /// - Mouse events will be send over the reader created with `read_async`, `read_async_until`, `read_sync`.
    pub fn enable_mouse_mode(&self) -> Result<()> {
        self.input.enable_mouse_mode()
    }

    /// Disable mouse events to be captured.
    ///
    /// When disabling mouse input, you won't be able to capture mouse movements, pressed buttons, and locations anymore.
    pub fn disable_mouse_mode(&self) -> Result<()> {
        self.input.disable_mouse_mode()
    }
}

/// Get a `TerminalInput` instance whereon input related actions can be performed.
pub fn input() -> TerminalInput {
    TerminalInput::new()
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
                        Some(val @ b'P'...b'S') => {
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
        b'\n' | b'\r' => InputEvent::Keyboard(KeyEvent::Char('\n')),
        b'\t' => InputEvent::Keyboard(KeyEvent::Char('\t')),
        b'\x7F' => InputEvent::Keyboard(KeyEvent::Backspace),
        c @ b'\x01'...b'\x1A' => {
            InputEvent::Keyboard(KeyEvent::Ctrl((c as u8 - 0x1 + b'a') as char))
        }
        c @ b'\x1C'...b'\x1F' => {
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
            Some(val @ b'A'...b'E') => InputEvent::Keyboard(KeyEvent::F(1 + val - b'A')),
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
                0...2 | 64...65 => {
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
        Some(c @ b'0'...b'9') => {
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
                        v @ 11...15 => InputEvent::Keyboard(KeyEvent::F(v - 10)),
                        v @ 17...21 => InputEvent::Keyboard(KeyEvent::F(v - 11)),
                        v @ 23...24 => InputEvent::Keyboard(KeyEvent::F(v - 12)),
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
#[test]
fn test_parse_utf8() {
    let st = "abcéŷ¤£€ù%323";
    let ref mut bytes = st.bytes().map(|x| Ok(x));
    let chars = st.chars();
    for c in chars {
        let b = bytes.next().unwrap().unwrap();
        assert_eq!(c, parse_utf8_char(b, bytes).unwrap());
    }
}
