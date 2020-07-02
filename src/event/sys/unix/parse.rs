use crate::{
    event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent},
    ErrorKind, Result,
};

use super::super::super::InternalEvent;
use std::io;

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
        "Could not parse an event.",
    ))
}

pub(crate) fn parse_event(buffer: &[u8], input_available: bool) -> Result<Option<InternalEvent>> {
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
                    Ok(Some(InternalEvent::Event(Event::Key(KeyCode::Esc.into()))))
                }
            } else {
                match buffer[1] {
                    b'O' => {
                        if buffer.len() == 2 {
                            Ok(None)
                        } else {
                            match buffer[2] {
                                // F1-F4
                                val @ b'P'..=b'S' => Ok(Some(InternalEvent::Event(Event::Key(
                                    KeyCode::F(1 + val - b'P').into(),
                                )))),
                                _ => Err(could_not_parse_event_error()),
                            }
                        }
                    }
                    b'[' => parse_csi(buffer),
                    b'\x1B' => Ok(Some(InternalEvent::Event(Event::Key(KeyCode::Esc.into())))),
                    _ => parse_utf8_char(&buffer[1..]).map(|maybe_char| {
                        maybe_char
                            .map(KeyCode::Char)
                            .map(|code| KeyEvent::new(code, KeyModifiers::ALT))
                            .map(Event::Key)
                            .map(InternalEvent::Event)
                    }),
                }
            }
        }
        b'\r' => Ok(Some(InternalEvent::Event(Event::Key(
            KeyCode::Enter.into(),
        )))),
        // Issue #371: \n = 0xA, which is also the keycode for Ctrl+J. The only reason we get
        // newlines as input is because the terminal converts \r into \n for us. When we
        // enter raw mode, we disable that, so \n no longer has any meaning - it's better to
        // use Ctrl+J. Waiting to handle it here means it gets picked up later
        b'\n' if !crate::terminal::sys::is_raw_mode_enabled() => Ok(Some(InternalEvent::Event(
            Event::Key(KeyCode::Enter.into()),
        ))),
        b'\t' => Ok(Some(InternalEvent::Event(Event::Key(KeyCode::Tab.into())))),
        b'\x7F' => Ok(Some(InternalEvent::Event(Event::Key(
            KeyCode::Backspace.into(),
        )))),
        c @ b'\x01'..=b'\x1A' => Ok(Some(InternalEvent::Event(Event::Key(KeyEvent::new(
            KeyCode::Char((c as u8 - 0x1 + b'a') as char),
            KeyModifiers::CONTROL,
        ))))),
        c @ b'\x1C'..=b'\x1F' => Ok(Some(InternalEvent::Event(Event::Key(KeyEvent::new(
            KeyCode::Char((c as u8 - 0x1C + b'4') as char),
            KeyModifiers::CONTROL,
        ))))),
        b'\0' => Ok(Some(InternalEvent::Event(Event::Key(KeyEvent::new(
            KeyCode::Char(' '),
            KeyModifiers::CONTROL,
        ))))),
        _ => parse_utf8_char(buffer).map(|maybe_char| {
            maybe_char
                .map(KeyCode::Char)
                .map(char_code_to_event)
                .map(Event::Key)
                .map(InternalEvent::Event)
        }),
    }
}

// converts KeyCode to KeyEvent (adds shift modifier in case of uppercase characters)
fn char_code_to_event(code: KeyCode) -> KeyEvent {
    let modifiers = match code {
        KeyCode::Char(c) if c.is_uppercase() => KeyModifiers::SHIFT,
        _ => KeyModifiers::empty(),
    };
    KeyEvent::new(code, modifiers)
}

pub(crate) fn parse_csi(buffer: &[u8]) -> Result<Option<InternalEvent>> {
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
                    val @ b'A'..=b'E' => Some(Event::Key(KeyCode::F(1 + val - b'A').into())),
                    _ => return Err(could_not_parse_event_error()),
                }
            }
        }
        b'D' => Some(Event::Key(KeyCode::Left.into())),
        b'C' => Some(Event::Key(KeyCode::Right.into())),
        b'A' => Some(Event::Key(KeyCode::Up.into())),
        b'B' => Some(Event::Key(KeyCode::Down.into())),
        b'H' => Some(Event::Key(KeyCode::Home.into())),
        b'F' => Some(Event::Key(KeyCode::End.into())),
        b'Z' => Some(Event::Key(KeyCode::BackTab.into())),
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

    Ok(input_event.map(InternalEvent::Event))
}

pub(crate) fn next_parsed<T>(iter: &mut dyn Iterator<Item = &str>) -> Result<T>
where
    T: std::str::FromStr,
{
    iter.next()
        .ok_or_else(could_not_parse_event_error)?
        .parse::<T>()
        .map_err(|_| could_not_parse_event_error())
}

pub(crate) fn parse_csi_cursor_position(buffer: &[u8]) -> Result<Option<InternalEvent>> {
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

fn parse_modifiers(mask: u8) -> KeyModifiers {
    let modifier_mask = mask.saturating_sub(1);
    let mut modifiers = KeyModifiers::empty();
    if modifier_mask & 1 != 0 {
        modifiers |= KeyModifiers::SHIFT;
    }
    if modifier_mask & 2 != 0 {
        modifiers |= KeyModifiers::ALT;
    }
    if modifier_mask & 4 != 0 {
        modifiers |= KeyModifiers::CONTROL;
    }
    modifiers
}

pub(crate) fn parse_csi_modifier_key_code(buffer: &[u8]) -> Result<Option<InternalEvent>> {
    assert!(buffer.starts_with(&[b'\x1B', b'['])); // ESC [

    let modifier_mask = buffer[buffer.len() - 2];
    let key = buffer[buffer.len() - 1];

    let modifiers = parse_modifiers(modifier_mask);

    let keycode = match key {
        b'A' => KeyCode::Up,
        b'B' => KeyCode::Down,
        b'C' => KeyCode::Right,
        b'D' => KeyCode::Left,
        b'F' => KeyCode::End,
        b'H' => KeyCode::Home,
        b'P' => KeyCode::F(1),
        b'Q' => KeyCode::F(2),
        b'R' => KeyCode::F(3),
        b'S' => KeyCode::F(4),
        _ => return Err(could_not_parse_event_error()),
    };

    let input_event = Event::Key(KeyEvent::new(keycode, modifiers));

    Ok(Some(InternalEvent::Event(input_event)))
}

pub(crate) fn parse_csi_special_key_code(buffer: &[u8]) -> Result<Option<InternalEvent>> {
    assert!(buffer.starts_with(&[b'\x1B', b'['])); // ESC [
    assert!(buffer.ends_with(&[b'~']));

    let s = std::str::from_utf8(&buffer[2..buffer.len() - 1])
        .map_err(|_| could_not_parse_event_error())?;
    let mut split = s.split(';');

    // This CSI sequence can be a list of semicolon-separated numbers.
    let first = next_parsed::<u8>(&mut split)?;

    let modifiers = if let Ok(modifier_mask) = next_parsed::<u8>(&mut split) {
        parse_modifiers(modifier_mask)
    } else {
        KeyModifiers::NONE
    };

    let keycode = match first {
        1 | 7 => KeyCode::Home,
        2 => KeyCode::Insert,
        3 => KeyCode::Delete,
        4 | 8 => KeyCode::End,
        5 => KeyCode::PageUp,
        6 => KeyCode::PageDown,
        v @ 11..=15 => KeyCode::F(v - 10),
        v @ 17..=21 => KeyCode::F(v - 11),
        v @ 23..=24 => KeyCode::F(v - 12),
        _ => return Err(could_not_parse_event_error()),
    };

    let input_event = Event::Key(KeyEvent::new(keycode, modifiers));

    Ok(Some(InternalEvent::Event(input_event)))
}

pub(crate) fn parse_csi_rxvt_mouse(buffer: &[u8]) -> Result<Option<InternalEvent>> {
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

    let mut modifiers = KeyModifiers::empty();

    if cb & 0b0000_0100 == 0b0000_0100 {
        modifiers |= KeyModifiers::SHIFT;
    }

    if cb & 0b0000_1000 == 0b0000_1000 {
        modifiers |= KeyModifiers::ALT;
    }

    if cb & 0b0001_0000 == 0b0001_0000 {
        modifiers |= KeyModifiers::CONTROL;
    }

    let event = if cb & 0b0110_0000 == 0b0110_0000 {
        if cb & 0b0000_0001 == 0b0000_0001 {
            MouseEvent::ScrollDown(cx, cy, modifiers)
        } else {
            MouseEvent::ScrollUp(cx, cy, modifiers)
        }
    } else {
        let drag = cb & 0b0100_0000 == 0b0100_0000;

        match (cb & 0b0000_0011, drag) {
            (0b0000_0000, false) => MouseEvent::Down(MouseButton::Left, cx, cy, modifiers),
            (0b0000_0010, false) => MouseEvent::Down(MouseButton::Right, cx, cy, modifiers),
            (0b0000_0001, false) => MouseEvent::Down(MouseButton::Middle, cx, cy, modifiers),

            (0b0000_0000, true) => MouseEvent::Drag(MouseButton::Left, cx, cy, modifiers),
            (0b0000_0010, true) => MouseEvent::Drag(MouseButton::Right, cx, cy, modifiers),
            (0b0000_0001, true) => MouseEvent::Drag(MouseButton::Middle, cx, cy, modifiers),

            (0b0000_0011, false) => MouseEvent::Up(MouseButton::Left, cx, cy, modifiers),

            _ => return Err(could_not_parse_event_error()),
        }
    };

    Ok(Some(InternalEvent::Event(Event::Mouse(event))))
}

pub(crate) fn parse_csi_x10_mouse(buffer: &[u8]) -> Result<Option<InternalEvent>> {
    // X10 emulation mouse encoding: ESC [ M CB Cx Cy (6 characters only).
    // NOTE (@imdaveho): cannot find documentation on this

    assert!(buffer.starts_with(&[b'\x1B', b'[', b'M'])); // ESC [ M

    if buffer.len() < 6 {
        return Ok(None);
    }

    let cb = buffer[3] - 0x30;
    // See http://www.xfree86.org/current/ctlseqs.html#Mouse%20Tracking
    // The upper left character position on the terminal is denoted as 1,1.
    // Subtract 1 to keep it synced with cursor
    let cx = u16::from(buffer[4].saturating_sub(32)) - 1;
    let cy = u16::from(buffer[5].saturating_sub(32)) - 1;

    let mut modifiers = KeyModifiers::empty();

    if cb & 0b0000_0100 == 0b0000_0100 {
        modifiers |= KeyModifiers::SHIFT;
    }

    if cb & 0b0000_1000 == 0b0000_1000 {
        modifiers |= KeyModifiers::ALT;
    }

    if cb & 0b0001_0000 == 0b0001_0000 {
        modifiers |= KeyModifiers::CONTROL;
    }

    let mouse_input_event = match cb & 0b0000_0011 {
        0 => {
            if cb & 0b0100_0000 == 0b0100_0000 {
                MouseEvent::ScrollUp(cx, cy, modifiers)
            } else {
                MouseEvent::Down(MouseButton::Left, cx, cy, modifiers)
            }
        }
        1 => {
            if cb & 0b0100_0000 == 0b0100_0000 {
                MouseEvent::ScrollDown(cx, cy, modifiers)
            } else {
                MouseEvent::Down(MouseButton::Middle, cx, cy, modifiers)
            }
        }
        2 => MouseEvent::Down(MouseButton::Right, cx, cy, modifiers),
        3 => MouseEvent::Up(MouseButton::Left, cx, cy, modifiers),
        _ => return Err(could_not_parse_event_error()),
    };

    Ok(Some(InternalEvent::Event(Event::Mouse(mouse_input_event))))
}

pub(crate) fn parse_csi_xterm_mouse(buffer: &[u8]) -> Result<Option<InternalEvent>> {
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

    let mut modifiers = KeyModifiers::empty();

    if cb & 0b0000_0100 == 0b0000_0100 {
        modifiers |= KeyModifiers::SHIFT;
    }

    if cb & 0b0000_1000 == 0b0000_1000 {
        modifiers |= KeyModifiers::ALT;
    }

    if cb & 0b0001_0000 == 0b0001_0000 {
        modifiers |= KeyModifiers::CONTROL;
    }

    let event = if cb & 0b0100_0000 == 0b0100_0000 {
        if cb & 0b0000_0001 == 0b0000_0001 {
            MouseEvent::ScrollDown(cx, cy, modifiers)
        } else {
            MouseEvent::ScrollUp(cx, cy, modifiers)
        }
    } else {
        let up = match buffer.last().unwrap() {
            b'm' => true,
            b'M' => false,
            _ => return Err(could_not_parse_event_error()),
        };

        let drag = cb & 0b0010_0000 == 0b0010_0000;

        match (cb & 0b0000_0011, up, drag) {
            (0, true, _) => MouseEvent::Up(MouseButton::Left, cx, cy, modifiers),
            (0, false, false) => MouseEvent::Down(MouseButton::Left, cx, cy, modifiers),
            (0, false, true) => MouseEvent::Drag(MouseButton::Left, cx, cy, modifiers),
            (1, true, _) => MouseEvent::Up(MouseButton::Middle, cx, cy, modifiers),
            (1, false, false) => MouseEvent::Down(MouseButton::Middle, cx, cy, modifiers),
            (1, false, true) => MouseEvent::Drag(MouseButton::Middle, cx, cy, modifiers),
            (2, true, _) => MouseEvent::Up(MouseButton::Right, cx, cy, modifiers),
            (2, false, false) => MouseEvent::Down(MouseButton::Right, cx, cy, modifiers),
            (2, false, true) => MouseEvent::Drag(MouseButton::Right, cx, cy, modifiers),
            _ => return Err(could_not_parse_event_error()),
        }
    };

    Ok(Some(InternalEvent::Event(Event::Mouse(event))))
}

pub(crate) fn parse_utf8_char(buffer: &[u8]) -> Result<Option<char>> {
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
    use crate::event::{KeyModifiers, MouseButton, MouseEvent};

    use super::*;

    #[test]
    fn test_esc_key() {
        assert_eq!(
            parse_event("\x1B".as_bytes(), false).unwrap(),
            Some(InternalEvent::Event(Event::Key(KeyCode::Esc.into()))),
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
            Some(InternalEvent::Event(Event::Key(KeyEvent::new(
                KeyCode::Char('c'),
                KeyModifiers::ALT
            )))),
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
            Some(InternalEvent::Event(Event::Key(KeyCode::Left.into()))),
        );

        // parse_csi_modifier_key_code
        assert_eq!(
            parse_event("\x1B[2D".as_bytes(), false).unwrap(),
            Some(InternalEvent::Event(Event::Key(KeyEvent::new(
                KeyCode::Left,
                KeyModifiers::SHIFT
            ))))
        );

        // parse_csi_special_key_code
        assert_eq!(
            parse_event("\x1B[3~".as_bytes(), false).unwrap(),
            Some(InternalEvent::Event(Event::Key(KeyCode::Delete.into()))),
        );

        // parse_csi_rxvt_mouse
        assert_eq!(
            parse_event("\x1B[32;30;40;M".as_bytes(), false).unwrap(),
            Some(InternalEvent::Event(Event::Mouse(MouseEvent::Down(
                MouseButton::Left,
                29,
                39,
                KeyModifiers::empty(),
            ))))
        );

        // parse_csi_x10_mouse
        assert_eq!(
            parse_event("\x1B[M0\x60\x70".as_bytes(), false).unwrap(),
            Some(InternalEvent::Event(Event::Mouse(MouseEvent::Down(
                MouseButton::Left,
                63,
                79,
                KeyModifiers::empty(),
            ))))
        );

        // parse_csi_xterm_mouse
        assert_eq!(
            parse_event("\x1B[<0;20;10;M".as_bytes(), false).unwrap(),
            Some(InternalEvent::Event(Event::Mouse(MouseEvent::Down(
                MouseButton::Left,
                19,
                9,
                KeyModifiers::empty(),
            ))))
        );

        // parse_utf8_char
        assert_eq!(
            parse_event("Ž".as_bytes(), false).unwrap(),
            Some(InternalEvent::Event(Event::Key(KeyEvent::new(
                KeyCode::Char('Ž'),
                KeyModifiers::SHIFT
            )))),
        );
    }

    #[test]
    fn test_parse_event() {
        assert_eq!(
            parse_event("\t".as_bytes(), false).unwrap(),
            Some(InternalEvent::Event(Event::Key(KeyCode::Tab.into()))),
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
            Some(InternalEvent::Event(Event::Key(KeyCode::Left.into()))),
        );
    }

    #[test]
    fn test_parse_csi_modifier_key_code() {
        assert_eq!(
            parse_csi_modifier_key_code("\x1B[2D".as_bytes()).unwrap(),
            Some(InternalEvent::Event(Event::Key(KeyEvent::new(
                KeyCode::Left,
                KeyModifiers::SHIFT
            )))),
        );
    }

    #[test]
    fn test_parse_csi_special_key_code() {
        assert_eq!(
            parse_csi_special_key_code("\x1B[3~".as_bytes()).unwrap(),
            Some(InternalEvent::Event(Event::Key(KeyCode::Delete.into()))),
        );
    }

    #[test]
    fn test_parse_csi_special_key_code_multiple_values_not_supported() {
        assert_eq!(
            parse_csi_special_key_code("\x1B[3;2~".as_bytes()).unwrap(),
            Some(InternalEvent::Event(Event::Key(KeyEvent::new(
                KeyCode::Delete,
                KeyModifiers::SHIFT
            )))),
        );
    }

    #[test]
    fn test_parse_csi_rxvt_mouse() {
        assert_eq!(
            parse_csi_rxvt_mouse("\x1B[32;30;40;M".as_bytes()).unwrap(),
            Some(InternalEvent::Event(Event::Mouse(MouseEvent::Down(
                MouseButton::Left,
                29,
                39,
                KeyModifiers::empty(),
            ))))
        );
    }

    #[test]
    fn test_parse_csi_x10_mouse() {
        assert_eq!(
            parse_csi_x10_mouse("\x1B[M0\x60\x70".as_bytes()).unwrap(),
            Some(InternalEvent::Event(Event::Mouse(MouseEvent::Down(
                MouseButton::Left,
                63,
                79,
                KeyModifiers::empty(),
            ))))
        );
    }

    #[test]
    fn test_parse_csi_xterm_mouse() {
        assert_eq!(
            parse_csi_xterm_mouse("\x1B[<0;20;10;M".as_bytes()).unwrap(),
            Some(InternalEvent::Event(Event::Mouse(MouseEvent::Down(
                MouseButton::Left,
                19,
                9,
                KeyModifiers::empty(),
            ))))
        );
        assert_eq!(
            parse_csi_xterm_mouse("\x1B[<0;20;10M".as_bytes()).unwrap(),
            Some(InternalEvent::Event(Event::Mouse(MouseEvent::Down(
                MouseButton::Left,
                19,
                9,
                KeyModifiers::empty(),
            ))))
        );
        assert_eq!(
            parse_csi_xterm_mouse("\x1B[<0;20;10;m".as_bytes()).unwrap(),
            Some(InternalEvent::Event(Event::Mouse(MouseEvent::Up(
                MouseButton::Left,
                19,
                9,
                KeyModifiers::empty(),
            ))))
        );
        assert_eq!(
            parse_csi_xterm_mouse("\x1B[<0;20;10m".as_bytes()).unwrap(),
            Some(InternalEvent::Event(Event::Mouse(MouseEvent::Up(
                MouseButton::Left,
                19,
                9,
                KeyModifiers::empty(),
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

    #[test]
    fn test_parse_char_event_lowercase() {
        assert_eq!(
            parse_event("c".as_bytes(), false).unwrap(),
            Some(InternalEvent::Event(Event::Key(KeyEvent::new(
                KeyCode::Char('c'),
                KeyModifiers::empty()
            )))),
        );
    }

    #[test]
    fn test_parse_char_event_uppercase() {
        assert_eq!(
            parse_event("C".as_bytes(), false).unwrap(),
            Some(InternalEvent::Event(Event::Key(KeyEvent::new(
                KeyCode::Char('C'),
                KeyModifiers::SHIFT
            )))),
        );
    }
}
