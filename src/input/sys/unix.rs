use std::{
    fs, io,
    os::unix::io::{IntoRawFd, RawFd},
};

use libc::c_int;

use crate::{
    input::{Event, KeyEvent, MouseButton, MouseEvent},
    ErrorKind, Result,
};

use super::super::events::InternalEvent;

/// A file descriptor wrapper.
///
/// It allows to retrieve raw file descriptor, write to the file descriptor and
/// mainly it closes the file descriptor once dropped.
pub struct FileDesc {
    fd: RawFd,
    close_on_drop: bool,
}

impl FileDesc {
    /// Constructs a new `FileDesc` with the given `RawFd`.
    ///
    /// # Arguments
    ///
    /// * `fd` - raw file descriptor
    /// * `close_on_drop` - specify if the raw file descriptor should be closed once the `FileDesc` is dropped
    pub fn new(fd: RawFd, close_on_drop: bool) -> FileDesc {
        FileDesc { fd, close_on_drop }
    }

    /// Reads a single byte from the file descriptor.
    pub fn read_byte(&self) -> Result<u8> {
        let mut buf: [u8; 1] = [0];
        crate::utils::sys::unix::wrap_with_result(unsafe {
            libc::read(self.fd, buf.as_mut_ptr() as *mut libc::c_void, 1) as c_int
        })?;

        Ok(buf[0])
    }

    /// Returns the underlying file descriptor.
    pub fn raw_fd(&self) -> RawFd {
        self.fd
    }
}

impl Drop for FileDesc {
    // libstd::sys::unix::fd.rs
    fn drop(&mut self) {
        if self.close_on_drop {
            // Note that errors are ignored when closing a file descriptor. The
            // reason for this is that if an error occurs we don't actually know if
            // the file descriptor was closed or not, and if we retried (for
            // something like EINTR), we might close another valid file descriptor
            // opened after we closed ours.
            let _ = unsafe { libc::close(self.fd) };
        }
    }
}

/// Creates a file descriptor pointing to the standard input or `/dev/tty`.
pub fn tty_fd() -> Result<FileDesc> {
    let (fd, close_on_drop) = if unsafe { libc::isatty(libc::STDIN_FILENO) == 1 } {
        (libc::STDIN_FILENO, false)
    } else {
        (
            fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open("/dev/tty")?
                .into_raw_fd(),
            true,
        )
    };

    Ok(FileDesc::new(fd, close_on_drop))
}

//
// Event parsing
//
// This code (& previous one) are kind of ugly. We have to think about this,
// because it's really not maintainable, no tests, etc.
//
// Every fn returns Result<Option<InputEvent>>
//
// Ok(None) -> wait for more bytes
// Err(_) -> failed to parse event, clear the buffer
// Ok(Some(event)) -> we have event, clear the buffer
//

fn could_not_parse_event_error() -> ErrorKind {
    ErrorKind::IoError(io::Error::new(
        io::ErrorKind::Other,
        "Could not parse an event",
    ))
}

pub fn parse_event(buffer: &[u8], input_available: bool) -> Result<Option<InternalEvent>> {
    if buffer.is_empty() {
        return Ok(None);
    }

    match buffer[0] {
        b'\x1B' => {
            if buffer.len() == 1 {
                if input_available {
                    // Possible Esc sequence
                    Ok(None)
                } else {
                    Ok(Some(InternalEvent::Input(Event::Key(KeyEvent::Esc))))
                }
            } else {
                match buffer[1] {
                    b'O' => {
                        if buffer.len() == 2 {
                            Ok(None)
                        } else {
                            match buffer[2] {
                                // F1-F4
                                val @ b'P'..=b'S' => Ok(Some(InternalEvent::Input(Event::Key(
                                    KeyEvent::F(1 + val - b'P'),
                                )))),
                                _ => Err(could_not_parse_event_error()),
                            }
                        }
                    }
                    b'[' => parse_csi(buffer),
                    b'\x1B' => Ok(Some(InternalEvent::Input(Event::Key(KeyEvent::Esc)))),
                    _ => parse_utf8_char(&buffer[1..]).map(|maybe_char| {
                        maybe_char
                            .map(KeyEvent::Alt)
                            .map(Event::Key)
                            .map(InternalEvent::Input)
                    }),
                }
            }
        }
        b'\r' | b'\n' => Ok(Some(InternalEvent::Input(Event::Key(KeyEvent::Enter)))),
        b'\t' => Ok(Some(InternalEvent::Input(Event::Key(KeyEvent::Tab)))),
        b'\x7F' => Ok(Some(InternalEvent::Input(Event::Key(KeyEvent::Backspace)))),
        c @ b'\x01'..=b'\x1A' => Ok(Some(InternalEvent::Input(Event::Key(KeyEvent::Ctrl(
            (c as u8 - 0x1 + b'a') as char,
        ))))),
        c @ b'\x1C'..=b'\x1F' => Ok(Some(InternalEvent::Input(Event::Key(KeyEvent::Ctrl(
            (c as u8 - 0x1C + b'4') as char,
        ))))),
        b'\0' => Ok(Some(InternalEvent::Input(Event::Key(KeyEvent::Null)))),
        _ => parse_utf8_char(buffer).map(|maybe_char| {
            maybe_char
                .map(KeyEvent::Char)
                .map(Event::Key)
                .map(InternalEvent::Input)
        }),
    }
}

pub fn parse_csi(buffer: &[u8]) -> Result<Option<InternalEvent>> {
    assert!(buffer.starts_with(&[b'\x1B', b'['])); // ESC [

    if buffer.len() == 2 {
        return Ok(None);
    }

    let input_event = match buffer[2] {
        b'[' => {
            if buffer.len() == 3 {
                None
            } else {
                match buffer[3] {
                    // NOTE (@imdaveho): cannot find when this occurs;
                    // having another '[' after ESC[ not a likely scenario
                    val @ b'A'..=b'E' => Some(Event::Key(KeyEvent::F(1 + val - b'A'))),
                    _ => return Err(could_not_parse_event_error()),
                }
            }
        }
        b'D' => Some(Event::Key(KeyEvent::Left)),
        b'C' => Some(Event::Key(KeyEvent::Right)),
        b'A' => Some(Event::Key(KeyEvent::Up)),
        b'B' => Some(Event::Key(KeyEvent::Down)),
        b'H' => Some(Event::Key(KeyEvent::Home)),
        b'F' => Some(Event::Key(KeyEvent::End)),
        b'Z' => Some(Event::Key(KeyEvent::BackTab)),
        b'M' => return parse_csi_x10_mouse(buffer),
        b'<' => return parse_csi_xterm_mouse(buffer),
        b'0'..=b'9' => {
            // Numbered escape code.
            if buffer.len() == 3 {
                None
            } else {
                // The final byte of a CSI sequence can be in the range 64-126, so
                // let's keep reading anything else.
                let last_byte = *buffer.last().unwrap();
                if last_byte < 64 || last_byte > 126 {
                    None
                } else {
                    match buffer[buffer.len() - 1] {
                        b'M' => return parse_csi_rxvt_mouse(buffer),
                        b'~' => return parse_csi_special_key_code(buffer),
                        b'R' => return parse_csi_cursor_position(buffer),
                        _ => return parse_csi_modifier_key_code(buffer),
                    }
                }
            }
        }
        _ => return Err(could_not_parse_event_error()),
    };

    Ok(input_event.map(InternalEvent::Input))
}

pub fn next_parsed<T>(iter: &mut dyn Iterator<Item = &str>) -> Result<T>
where
    T: std::str::FromStr,
{
    iter.next()
        .ok_or_else(could_not_parse_event_error)?
        .parse::<T>()
        .map_err(|_| could_not_parse_event_error())
}

pub fn parse_csi_cursor_position(buffer: &[u8]) -> Result<Option<InternalEvent>> {
    // ESC [ Cy ; Cx R
    //   Cy - cursor row number (starting from 1)
    //   Cx - cursor column number (starting from 1)
    assert!(buffer.starts_with(&[b'\x1B', b'['])); // ESC [
    assert!(buffer.ends_with(&[b'R']));

    let s = std::str::from_utf8(&buffer[2..buffer.len() - 1])
        .map_err(|_| could_not_parse_event_error())?;

    let mut split = s.split(';');

    let y = next_parsed::<u16>(&mut split)? - 1;
    let x = next_parsed::<u16>(&mut split)? - 1;

    Ok(Some(InternalEvent::CursorPosition(x, y)))
}

pub fn parse_csi_modifier_key_code(buffer: &[u8]) -> Result<Option<InternalEvent>> {
    assert!(buffer.starts_with(&[b'\x1B', b'['])); // ESC [

    let modifier = buffer[buffer.len() - 2];
    let key = buffer[buffer.len() - 1];

    let input_event = match (modifier, key) {
        (53, 65) => Event::Key(KeyEvent::CtrlUp),
        (53, 66) => Event::Key(KeyEvent::CtrlDown),
        (53, 67) => Event::Key(KeyEvent::CtrlRight),
        (53, 68) => Event::Key(KeyEvent::CtrlLeft),
        (50, 65) => Event::Key(KeyEvent::ShiftUp),
        (50, 66) => Event::Key(KeyEvent::ShiftDown),
        (50, 67) => Event::Key(KeyEvent::ShiftRight),
        (50, 68) => Event::Key(KeyEvent::ShiftLeft),
        _ => return Err(could_not_parse_event_error()),
    };

    Ok(Some(InternalEvent::Input(input_event)))
}

pub fn parse_csi_special_key_code(buffer: &[u8]) -> Result<Option<InternalEvent>> {
    assert!(buffer.starts_with(&[b'\x1B', b'['])); // ESC [
    assert!(buffer.ends_with(&[b'~']));

    let s = std::str::from_utf8(&buffer[2..buffer.len() - 1])
        .map_err(|_| could_not_parse_event_error())?;
    let mut split = s.split(';');

    // This CSI sequence can be a list of semicolon-separated numbers.
    let first = next_parsed::<u8>(&mut split)?;

    if next_parsed::<u8>(&mut split).is_ok() {
        // TODO: handle multiple values for key modifiers (ex: values [3, 2] means Shift+Delete)
        return Err(could_not_parse_event_error());
    }

    let input_event = match first {
        1 | 7 => Event::Key(KeyEvent::Home),
        2 => Event::Key(KeyEvent::Insert),
        3 => Event::Key(KeyEvent::Delete),
        4 | 8 => Event::Key(KeyEvent::End),
        5 => Event::Key(KeyEvent::PageUp),
        6 => Event::Key(KeyEvent::PageDown),
        v @ 11..=15 => Event::Key(KeyEvent::F(v - 10)),
        v @ 17..=21 => Event::Key(KeyEvent::F(v - 11)),
        v @ 23..=24 => Event::Key(KeyEvent::F(v - 12)),
        _ => return Err(could_not_parse_event_error()),
    };

    Ok(Some(InternalEvent::Input(input_event)))
}

pub fn parse_csi_rxvt_mouse(buffer: &[u8]) -> Result<Option<InternalEvent>> {
    // rxvt mouse encoding:
    // ESC [ Cb ; Cx ; Cy ; M

    assert!(buffer.starts_with(&[b'\x1B', b'['])); // ESC [
    assert!(buffer.ends_with(&[b'M']));

    let s = std::str::from_utf8(&buffer[2..buffer.len() - 1])
        .map_err(|_| could_not_parse_event_error())?;
    let mut split = s.split(';');

    let cb = next_parsed::<u16>(&mut split)?;
    let cx = next_parsed::<u16>(&mut split)? - 1;
    let cy = next_parsed::<u16>(&mut split)? - 1;

    let mouse_input_event = match cb {
        32 => MouseEvent::Press(MouseButton::Left, cx, cy),
        33 => MouseEvent::Press(MouseButton::Middle, cx, cy),
        34 => MouseEvent::Press(MouseButton::Right, cx, cy),
        35 => MouseEvent::Release(cx, cy),
        64 => MouseEvent::Hold(cx, cy),
        96 | 97 => MouseEvent::Press(MouseButton::WheelUp, cx, cy),
        _ => return Err(could_not_parse_event_error()),
    };

    Ok(Some(InternalEvent::Input(Event::Mouse(mouse_input_event))))
}

pub fn parse_csi_x10_mouse(buffer: &[u8]) -> Result<Option<InternalEvent>> {
    // X10 emulation mouse encoding: ESC [ M CB Cx Cy (6 characters only).
    // NOTE (@imdaveho): cannot find documentation on this

    assert!(buffer.starts_with(&[b'\x1B', b'[', b'M'])); // ESC [ M

    if buffer.len() < 6 {
        return Ok(None);
    }

    let cb = buffer[3] as i8 - 32;
    // See http://www.xfree86.org/current/ctlseqs.html#Mouse%20Tracking
    // The upper left character position on the terminal is denoted as 1,1.
    // Subtract 1 to keep it synced with cursor
    let cx = u16::from(buffer[4].saturating_sub(32)) - 1;
    let cy = u16::from(buffer[5].saturating_sub(32)) - 1;

    let mouse_input_event = match cb & 0b11 {
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
        _ => return Err(could_not_parse_event_error()),
    };

    Ok(Some(InternalEvent::Input(Event::Mouse(mouse_input_event))))
}

pub fn parse_csi_xterm_mouse(buffer: &[u8]) -> Result<Option<InternalEvent>> {
    // ESC [ < Cb ; Cx ; Cy (;) (M or m)

    assert!(buffer.starts_with(&[b'\x1B', b'[', b'<'])); // ESC [ <

    if !buffer.ends_with(&[b'm']) && !buffer.ends_with(&[b'M']) {
        return Ok(None);
    }

    let s = std::str::from_utf8(&buffer[3..buffer.len() - 1])
        .map_err(|_| could_not_parse_event_error())?;
    let mut split = s.split(';');

    let cb = next_parsed::<u16>(&mut split)?;

    // See http://www.xfree86.org/current/ctlseqs.html#Mouse%20Tracking
    // The upper left character position on the terminal is denoted as 1,1.
    // Subtract 1 to keep it synced with cursor
    let cx = next_parsed::<u16>(&mut split)? - 1;
    let cy = next_parsed::<u16>(&mut split)? - 1;

    let input_event = match cb {
        0..=2 | 64..=65 => {
            let button = match cb {
                0 => MouseButton::Left,
                1 => MouseButton::Middle,
                2 => MouseButton::Right,
                64 => MouseButton::WheelUp,
                65 => MouseButton::WheelDown,
                _ => unreachable!(),
            };
            match buffer.last().unwrap() {
                b'M' => Event::Mouse(MouseEvent::Press(button, cx, cy)),
                b'm' => Event::Mouse(MouseEvent::Release(cx, cy)),
                _ => return Err(could_not_parse_event_error()),
            }
        }
        // TODO 1.0: Add MouseButton to Hold and report which button is pressed
        // 33 - middle, 34 - right
        32 => Event::Mouse(MouseEvent::Hold(cx, cy)),
        3 => Event::Mouse(MouseEvent::Release(cx, cy)),
        _ => return Err(could_not_parse_event_error()),
    };

    Ok(Some(InternalEvent::Input(input_event)))
}

pub fn parse_utf8_char(buffer: &[u8]) -> Result<Option<char>> {
    match std::str::from_utf8(buffer) {
        Ok(s) => {
            let ch = s.chars().next().ok_or_else(could_not_parse_event_error)?;

            Ok(Some(ch))
        }
        Err(_) => {
            // from_utf8 failed, but we have to check if we need more bytes for code point
            // and if all the bytes we have no are valid

            let required_bytes = match buffer[0] {
                // https://en.wikipedia.org/wiki/UTF-8#Description
                (0x00..=0x7F) => 1, // 0xxxxxxx
                (0xC0..=0xDF) => 2, // 110xxxxx 10xxxxxx
                (0xE0..=0xEF) => 3, // 1110xxxx 10xxxxxx 10xxxxxx
                (0xF0..=0xF7) => 4, // 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx
                (0x80..=0xBF) | (0xF8..=0xFF) => return Err(could_not_parse_event_error()),
            };

            // More than 1 byte, check them for 10xxxxxx pattern
            if required_bytes > 1 && buffer.len() > 1 {
                for byte in &buffer[1..] {
                    if byte & !0b0011_1111 != 0b1000_0000 {
                        return Err(could_not_parse_event_error());
                    }
                }
            }

            if buffer.len() < required_bytes {
                // All bytes looks good so far, but we need more of them
                Ok(None)
            } else {
                Err(could_not_parse_event_error())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::input::{MouseButton, MouseEvent};

    use super::*;

    #[test]
    fn test_esc_key() {
        assert_eq!(
            parse_event("\x1B".as_bytes(), false).unwrap(),
            Some(InternalEvent::Input(Event::Key(KeyEvent::Esc))),
        );
    }

    #[test]
    fn test_possible_esc_sequence() {
        assert_eq!(parse_event("\x1B".as_bytes(), true).unwrap(), None,);
    }

    #[test]
    fn test_alt_key() {
        assert_eq!(
            parse_event("\x1Bc".as_bytes(), false).unwrap(),
            Some(InternalEvent::Input(Event::Key(KeyEvent::Alt('c')))),
        );
    }

    #[test]
    fn test_parse_event_subsequent_calls() {
        // The main purpose of this test is to check if we're passing
        // correct slice to other parse_ functions.

        // parse_csi_cursor_position
        assert_eq!(
            parse_event("\x1B[20;10R".as_bytes(), false).unwrap(),
            Some(InternalEvent::CursorPosition(9, 19))
        );

        // parse_csi
        assert_eq!(
            parse_event("\x1B[D".as_bytes(), false).unwrap(),
            Some(InternalEvent::Input(Event::Key(KeyEvent::Left))),
        );

        // parse_csi_modifier_key_code
        assert_eq!(
            parse_event("\x1B[2D".as_bytes(), false).unwrap(),
            Some(InternalEvent::Input(Event::Key(KeyEvent::ShiftLeft))),
        );

        // parse_csi_special_key_code
        assert_eq!(
            parse_event("\x1B[3~".as_bytes(), false).unwrap(),
            Some(InternalEvent::Input(Event::Key(KeyEvent::Delete))),
        );

        // parse_csi_rxvt_mouse
        assert_eq!(
            parse_event("\x1B[32;30;40;M".as_bytes(), false).unwrap(),
            Some(InternalEvent::Input(Event::Mouse(MouseEvent::Press(
                MouseButton::Left,
                29,
                39
            ))))
        );

        // parse_csi_x10_mouse
        assert_eq!(
            parse_event("\x1B[M0\x60\x70".as_bytes(), false).unwrap(),
            Some(InternalEvent::Input(Event::Mouse(MouseEvent::Press(
                MouseButton::Left,
                63,
                79
            ))))
        );

        // parse_csi_xterm_mouse
        assert_eq!(
            parse_event("\x1B[<0;20;10;M".as_bytes(), false).unwrap(),
            Some(InternalEvent::Input(Event::Mouse(MouseEvent::Press(
                MouseButton::Left,
                19,
                9
            ))))
        );

        // parse_utf8_char
        assert_eq!(
            parse_event("Ž".as_bytes(), false).unwrap(),
            Some(InternalEvent::Input(Event::Key(KeyEvent::Char('Ž')))),
        );
    }

    #[test]
    fn test_parse_event() {
        assert_eq!(
            parse_event("\t".as_bytes(), false).unwrap(),
            Some(InternalEvent::Input(Event::Key(KeyEvent::Tab))),
        );
    }

    #[test]
    fn test_parse_csi_cursor_position() {
        assert_eq!(
            parse_csi_cursor_position("\x1B[20;10R".as_bytes()).unwrap(),
            Some(InternalEvent::CursorPosition(9, 19))
        );
    }

    #[test]
    fn test_parse_csi() {
        assert_eq!(
            parse_csi("\x1B[D".as_bytes()).unwrap(),
            Some(InternalEvent::Input(Event::Key(KeyEvent::Left))),
        );
    }

    #[test]
    fn test_parse_csi_modifier_key_code() {
        assert_eq!(
            parse_csi_modifier_key_code("\x1B[2D".as_bytes()).unwrap(),
            Some(InternalEvent::Input(Event::Key(KeyEvent::ShiftLeft))),
        );
    }

    #[test]
    fn test_parse_csi_special_key_code() {
        assert_eq!(
            parse_csi_special_key_code("\x1B[3~".as_bytes()).unwrap(),
            Some(InternalEvent::Input(Event::Key(KeyEvent::Delete))),
        );
    }

    #[test]
    fn test_parse_csi_special_key_code_multiple_values_not_supported() {
        assert!(parse_csi_special_key_code("\x1B[3;2~".as_bytes()).is_err());
    }

    #[test]
    fn test_parse_csi_rxvt_mouse() {
        assert_eq!(
            parse_csi_rxvt_mouse("\x1B[32;30;40;M".as_bytes()).unwrap(),
            Some(InternalEvent::Input(Event::Mouse(MouseEvent::Press(
                MouseButton::Left,
                29,
                39
            ))))
        );
    }

    #[test]
    fn test_parse_csi_x10_mouse() {
        assert_eq!(
            parse_csi_x10_mouse("\x1B[M0\x60\x70".as_bytes()).unwrap(),
            Some(InternalEvent::Input(Event::Mouse(MouseEvent::Press(
                MouseButton::Left,
                63,
                79
            ))))
        );
    }

    #[test]
    fn test_parse_csi_xterm_mouse() {
        assert_eq!(
            parse_csi_xterm_mouse("\x1B[<0;20;10;M".as_bytes()).unwrap(),
            Some(InternalEvent::Input(Event::Mouse(MouseEvent::Press(
                MouseButton::Left,
                19,
                9
            ))))
        );
        assert_eq!(
            parse_csi_xterm_mouse("\x1B[<0;20;10M".as_bytes()).unwrap(),
            Some(InternalEvent::Input(Event::Mouse(MouseEvent::Press(
                MouseButton::Left,
                19,
                9
            ))))
        );
        assert_eq!(
            parse_csi_xterm_mouse("\x1B[<0;20;10;m".as_bytes()).unwrap(),
            Some(InternalEvent::Input(Event::Mouse(MouseEvent::Release(
                19, 9
            ))))
        );
        assert_eq!(
            parse_csi_xterm_mouse("\x1B[<0;20;10m".as_bytes()).unwrap(),
            Some(InternalEvent::Input(Event::Mouse(MouseEvent::Release(
                19, 9
            ))))
        );
    }

    #[test]
    fn test_utf8() {
        // https://www.php.net/manual/en/reference.pcre.pattern.modifiers.php#54805

        // 'Valid ASCII' => "a",
        assert_eq!(parse_utf8_char("a".as_bytes()).unwrap(), Some('a'),);

        // 'Valid 2 Octet Sequence' => "\xc3\xb1",
        assert_eq!(parse_utf8_char(&[0xC3, 0xB1]).unwrap(), Some('ñ'),);

        // 'Invalid 2 Octet Sequence' => "\xc3\x28",
        assert!(parse_utf8_char(&[0xC3, 0x28]).is_err());

        // 'Invalid Sequence Identifier' => "\xa0\xa1",
        assert!(parse_utf8_char(&[0xA0, 0xA1]).is_err());

        // 'Valid 3 Octet Sequence' => "\xe2\x82\xa1",
        assert_eq!(
            parse_utf8_char(&[0xE2, 0x81, 0xA1]).unwrap(),
            Some('\u{2061}'),
        );

        // 'Invalid 3 Octet Sequence (in 2nd Octet)' => "\xe2\x28\xa1",
        assert!(parse_utf8_char(&[0xE2, 0x28, 0xA1]).is_err());

        // 'Invalid 3 Octet Sequence (in 3rd Octet)' => "\xe2\x82\x28",
        assert!(parse_utf8_char(&[0xE2, 0x82, 0x28]).is_err());

        // 'Valid 4 Octet Sequence' => "\xf0\x90\x8c\xbc",
        assert_eq!(
            parse_utf8_char(&[0xF0, 0x90, 0x8C, 0xBC]).unwrap(),
            Some('𐌼'),
        );

        // 'Invalid 4 Octet Sequence (in 2nd Octet)' => "\xf0\x28\x8c\xbc",
        assert!(parse_utf8_char(&[0xF0, 0x28, 0x8C, 0xBC]).is_err());

        // 'Invalid 4 Octet Sequence (in 3rd Octet)' => "\xf0\x90\x28\xbc",
        assert!(parse_utf8_char(&[0xF0, 0x90, 0x28, 0xBC]).is_err());

        // 'Invalid 4 Octet Sequence (in 4th Octet)' => "\xf0\x28\x8c\x28",
        assert!(parse_utf8_char(&[0xF0, 0x28, 0x8C, 0x28]).is_err());
    }
}
