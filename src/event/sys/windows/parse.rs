use crossterm_winapi::{ControlKeyState, EventFlags, KeyEventRecord, ScreenBuffer};
use winapi::um::{
    wincon::{
        CAPSLOCK_ON, LEFT_ALT_PRESSED, LEFT_CTRL_PRESSED, RIGHT_ALT_PRESSED, RIGHT_CTRL_PRESSED,
        SHIFT_PRESSED,
    },
    winuser::{
        GetForegroundWindow, GetKeyboardLayout, GetWindowThreadProcessId, ToUnicodeEx, VK_BACK,
        VK_CONTROL, VK_DELETE, VK_DOWN, VK_END, VK_ESCAPE, VK_F1, VK_F24, VK_HOME, VK_INSERT,
        VK_LEFT, VK_MENU, VK_NEXT, VK_NUMPAD0, VK_NUMPAD9, VK_PRIOR, VK_RETURN, VK_RIGHT, VK_SHIFT,
        VK_TAB, VK_UP,
    },
};

use crate::event::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};

#[derive(Default)]
pub struct MouseButtonsPressed {
    pub(crate) left: bool,
    pub(crate) right: bool,
    pub(crate) middle: bool,
}

pub(crate) fn handle_mouse_event(
    mouse_event: crossterm_winapi::MouseEvent,
    buttons_pressed: &MouseButtonsPressed,
) -> Option<Event> {
    if let Ok(Some(event)) = parse_mouse_event_record(&mouse_event, buttons_pressed) {
        return Some(Event::Mouse(event));
    }

    None
}

enum WindowsKeyEvent {
    KeyEvent(KeyEvent),
    Surrogate(u16),
}

pub(crate) fn handle_key_event(
    key_event: KeyEventRecord,
    surrogate_buffer: &mut Option<u16>,
) -> Option<Event> {
    let windows_key_event = parse_key_event_record(&key_event)?;
    match windows_key_event {
        WindowsKeyEvent::KeyEvent(key_event) => {
            // Discard any buffered surrogate value if another valid key event comes before the
            // next surrogate value.
            *surrogate_buffer = None;
            Some(Event::Key(key_event))
        }
        WindowsKeyEvent::Surrogate(new_surrogate) => {
            let ch = handle_surrogate(surrogate_buffer, new_surrogate)?;
            let modifiers = KeyModifiers::from(&key_event.control_key_state);
            let key_event = KeyEvent::new(KeyCode::Char(ch), modifiers);
            Some(Event::Key(key_event))
        }
    }
}

fn handle_surrogate(surrogate_buffer: &mut Option<u16>, new_surrogate: u16) -> Option<char> {
    match *surrogate_buffer {
        Some(buffered_surrogate) => {
            *surrogate_buffer = None;
            std::char::decode_utf16([buffered_surrogate, new_surrogate])
                .next()
                .unwrap()
                .ok()
        }
        None => {
            *surrogate_buffer = Some(new_surrogate);
            None
        }
    }
}

impl From<&ControlKeyState> for KeyModifiers {
    fn from(state: &ControlKeyState) -> Self {
        let shift = state.has_state(SHIFT_PRESSED);
        let alt = state.has_state(LEFT_ALT_PRESSED | RIGHT_ALT_PRESSED);
        let control = state.has_state(LEFT_CTRL_PRESSED | RIGHT_CTRL_PRESSED);

        let mut modifier = KeyModifiers::empty();

        if shift {
            modifier |= KeyModifiers::SHIFT;
        }
        if control {
            modifier |= KeyModifiers::CONTROL;
        }
        if alt {
            modifier |= KeyModifiers::ALT;
        }

        modifier
    }
}

enum CharCase {
    LowerCase,
    UpperCase,
}

fn try_ensure_char_case(ch: char, desired_case: CharCase) -> char {
    match desired_case {
        CharCase::LowerCase if ch.is_uppercase() => {
            let mut iter = ch.to_lowercase();
            // Unwrap is safe; iterator yields one or more chars.
            let ch_lower = iter.next().unwrap();
            if iter.next().is_none() { ch_lower } else { ch }
        }
        CharCase::UpperCase if ch.is_lowercase() => {
            let mut iter = ch.to_uppercase();
            // Unwrap is safe; iterator yields one or more chars.
            let ch_upper = iter.next().unwrap();
            if iter.next().is_none() { ch_upper } else { ch }
        }
        _ => ch,
    }
}

// Attempts to return the character for a key event accounting for the user's keyboard layout.
// The returned character (if any) is capitalized (if applicable) based on shift and capslock state.
// Returns None if the key doesn't map to a character or if it is a dead key.
// We use the *currently* active keyboard layout (if it can be determined). This layout may not
// correspond to the keyboard layout that was active when the user typed their input, since console
// applications get their input asynchronously from the terminal. By the time a console application
// can process a key input, the user may have changed the active layout. In this case, the character
// returned might not correspond to what the user expects, but there is no way for a console
// application to know what the keyboard layout actually was for a key event, so this is our best
// effort. If a console application processes input in a timely fashion, then it is unlikely that a
// user has time to change their keyboard layout before a key event is processed.
fn get_char_for_key(key_event: &KeyEventRecord) -> Option<char> {
    let virtual_key_code = key_event.virtual_key_code as u32;
    let virtual_scan_code = key_event.virtual_scan_code as u32;
    let key_state = [0u8; 256];
    let mut utf16_buf = [0u16; 16];
    let dont_change_kernel_keyboard_state = 0x4;

    // Best-effort attempt at determining the currently active keyboard layout.
    // At the time of writing, this works for a console application running in Windows Terminal, but
    // doesn't work under a Conhost terminal. For Conhost, the window handle returned by
    // GetForegroundWindow() does not appear to actually be the foreground window which has the
    // keyboard layout associated with it (or perhaps it is, but also has special protection that
    // doesn't allow us to query it).
    // When this determination fails, the returned keyboard layout handle will be null, which is an
    // acceptable input for ToUnicodeEx, as that argument is optional. In this case ToUnicodeEx
    // appears to use the keyboard layout associated with the current thread, which will be the
    // layout that was inherited when the console application started (or possibly when the current
    // thread was spawned). This is then unfortunately not updated when the user changes their
    // keyboard layout in the terminal, but it's what we get.
    let active_keyboard_layout = unsafe {
        let foreground_window = GetForegroundWindow();
        let foreground_thread = GetWindowThreadProcessId(foreground_window, std::ptr::null_mut());
        GetKeyboardLayout(foreground_thread)
    };

    let ret = unsafe {
        ToUnicodeEx(
            virtual_key_code,
            virtual_scan_code,
            key_state.as_ptr(),
            utf16_buf.as_mut_ptr(),
            utf16_buf.len() as i32,
            dont_change_kernel_keyboard_state,
            active_keyboard_layout,
        )
    };

    // -1 indicates a dead key.
    // 0 indicates no character for this key.
    if ret < 1 {
        return None;
    }

    let mut ch_iter = std::char::decode_utf16(utf16_buf.into_iter().take(ret as usize));
    let mut ch = ch_iter.next()?.ok()?;
    if ch_iter.next().is_some() {
        // Key doesn't map to a single char.
        return None;
    }

    let is_shift_pressed = key_event.control_key_state.has_state(SHIFT_PRESSED);
    let is_capslock_on = key_event.control_key_state.has_state(CAPSLOCK_ON);
    let desired_case = if is_shift_pressed ^ is_capslock_on {
        CharCase::UpperCase
    } else {
        CharCase::LowerCase
    };
    ch = try_ensure_char_case(ch, desired_case);
    Some(ch)
}

/// Returns the Ctrl+letter key code for a letter index, from 0 for A to 25 for Z.
///
/// Only Shift selects the uppercase letter. Caps Lock changes neither the control code nor the
/// virtual key, and on Unix Ctrl+letter with Caps Lock arrives as the same control byte.
fn ctrl_letter(index: u8, modifiers: KeyModifiers) -> KeyCode {
    let letter = char::from(b'a' + index);
    if modifiers.contains(KeyModifiers::SHIFT) {
        KeyCode::Char(letter.to_ascii_uppercase())
    } else {
        KeyCode::Char(letter)
    }
}

fn parse_key_event_record(key_event: &KeyEventRecord) -> Option<WindowsKeyEvent> {
    let modifiers = KeyModifiers::from(&key_event.control_key_state);
    let virtual_key_code = key_event.virtual_key_code as i32;

    // We normally ignore all key release events, but we will make an exception for an Alt key
    // release if it carries a u_char value, as this indicates an Alt code.
    let is_alt_code = virtual_key_code == VK_MENU && !key_event.key_down && key_event.u_char != 0;
    if is_alt_code {
        let utf16 = key_event.u_char;
        match utf16 {
            surrogate @ 0xD800..=0xDFFF => {
                return Some(WindowsKeyEvent::Surrogate(surrogate));
            }
            unicode_scalar_value => {
                // Unwrap is safe: We tested for surrogate values above and those are the only
                // u16 values that are invalid when directly interpreted as unicode scalar
                // values.
                let ch = std::char::from_u32(unicode_scalar_value as u32).unwrap();
                let key_code = KeyCode::Char(ch);
                let kind = if key_event.key_down {
                    KeyEventKind::Press
                } else {
                    KeyEventKind::Release
                };
                let key_event = KeyEvent::new_with_kind(key_code, modifiers, kind);
                return Some(WindowsKeyEvent::KeyEvent(key_event));
            }
        }
    }

    // Don't generate events for numpad key presses when they're producing Alt codes.
    let is_numpad_numeric_key = (VK_NUMPAD0..=VK_NUMPAD9).contains(&virtual_key_code);
    let is_only_alt_modifier = modifiers.contains(KeyModifiers::ALT)
        && !modifiers.contains(KeyModifiers::SHIFT | KeyModifiers::CONTROL);
    if is_only_alt_modifier && is_numpad_numeric_key {
        return None;
    }

    let parse_result = match virtual_key_code {
        VK_SHIFT | VK_CONTROL | VK_MENU => None,
        VK_BACK => Some(KeyCode::Backspace),
        VK_ESCAPE => Some(KeyCode::Esc),
        VK_RETURN => Some(KeyCode::Enter),
        VK_F1..=VK_F24 => Some(KeyCode::F((key_event.virtual_key_code - 111) as u8)),
        VK_LEFT => Some(KeyCode::Left),
        VK_UP => Some(KeyCode::Up),
        VK_RIGHT => Some(KeyCode::Right),
        VK_DOWN => Some(KeyCode::Down),
        VK_PRIOR => Some(KeyCode::PageUp),
        VK_NEXT => Some(KeyCode::PageDown),
        VK_HOME => Some(KeyCode::Home),
        VK_END => Some(KeyCode::End),
        VK_DELETE => Some(KeyCode::Delete),
        VK_INSERT => Some(KeyCode::Insert),
        VK_TAB if modifiers.contains(KeyModifiers::SHIFT) => Some(KeyCode::BackTab),
        VK_TAB => Some(KeyCode::Tab),
        _ => {
            let utf16 = key_event.u_char;
            // Windows reports AltGr as Ctrl+Alt, and AltGr types layout characters, so only Ctrl
            // without Alt makes a Ctrl+letter shortcut.
            let ctrl_without_alt =
                modifiers.contains(KeyModifiers::CONTROL) && !modifiers.contains(KeyModifiers::ALT);
            match utf16 {
                // Windows reports Ctrl+A to Ctrl+Z as the control codes 0x01..=0x1a. Looking the
                // key up in the active layout instead would report that layout's character, such
                // as Cyrillic 'з' for Ctrl+Z.
                control_code @ 0x01..=0x1a if ctrl_without_alt => {
                    Some(ctrl_letter((control_code - 0x01) as u8, modifiers))
                }
                // Some layouts, such as Armenian, report Ctrl+Shift+letter with no character. The
                // virtual key still names the letter: 0x41..=0x5a are VK_A to VK_Z.
                0x00 if ctrl_without_alt && (0x41..=0x5a).contains(&virtual_key_code) => {
                    Some(ctrl_letter((virtual_key_code - 0x41) as u8, modifiers))
                }
                0x00..=0x1f => {
                    // Some key combinations generate either no u_char value or generate control
                    // codes. To deliver back a KeyCode::Char(...) event we want to know which
                    // character the key normally maps to on the user's keyboard layout.
                    // The keys that intentionally generate control codes (ESC, ENTER, TAB, etc.)
                    // are handled by their virtual key codes above.
                    get_char_for_key(key_event).map(KeyCode::Char)
                }
                surrogate @ 0xD800..=0xDFFF => {
                    return Some(WindowsKeyEvent::Surrogate(surrogate));
                }
                unicode_scalar_value => {
                    // Unwrap is safe: We tested for surrogate values above and those are the only
                    // u16 values that are invalid when directly interpreted as unicode scalar
                    // values.
                    let ch = std::char::from_u32(unicode_scalar_value as u32).unwrap();
                    Some(KeyCode::Char(ch))
                }
            }
        }
    };

    if let Some(key_code) = parse_result {
        let kind = if key_event.key_down {
            KeyEventKind::Press
        } else {
            KeyEventKind::Release
        };
        let key_event = KeyEvent::new_with_kind(key_code, modifiers, kind);
        return Some(WindowsKeyEvent::KeyEvent(key_event));
    }

    None
}

// The 'y' position of a mouse event or resize event is not relative to the window but absolute to
// screen buffer. This means that when the mouse cursor is at the top left it will be x: 0, y: 2295
// (e.g. y = number of cells conting from the absolute buffer height) instead of relative x: 0, y: 0
// to the window.
pub fn parse_relative_y(y: i16) -> std::io::Result<i16> {
    let window_size = ScreenBuffer::current()?.info()?.terminal_window();
    Ok(y - window_size.top)
}

fn parse_mouse_event_record(
    event: &crossterm_winapi::MouseEvent,
    buttons_pressed: &MouseButtonsPressed,
) -> std::io::Result<Option<MouseEvent>> {
    let modifiers = KeyModifiers::from(&event.control_key_state);

    let xpos = event.mouse_position.x as u16;
    let ypos = parse_relative_y(event.mouse_position.y)? as u16;

    let button_state = event.button_state;

    let kind = match event.event_flags {
        EventFlags::PressOrRelease | EventFlags::DoubleClick => {
            if button_state.left_button() && !buttons_pressed.left {
                Some(MouseEventKind::Down(MouseButton::Left))
            } else if !button_state.left_button() && buttons_pressed.left {
                Some(MouseEventKind::Up(MouseButton::Left))
            } else if button_state.right_button() && !buttons_pressed.right {
                Some(MouseEventKind::Down(MouseButton::Right))
            } else if !button_state.right_button() && buttons_pressed.right {
                Some(MouseEventKind::Up(MouseButton::Right))
            } else if button_state.middle_button() && !buttons_pressed.middle {
                Some(MouseEventKind::Down(MouseButton::Middle))
            } else if !button_state.middle_button() && buttons_pressed.middle {
                Some(MouseEventKind::Up(MouseButton::Middle))
            } else {
                None
            }
        }
        EventFlags::MouseMoved => {
            let button = if button_state.right_button() {
                MouseButton::Right
            } else if button_state.middle_button() {
                MouseButton::Middle
            } else {
                MouseButton::Left
            };
            if button_state.release_button() {
                Some(MouseEventKind::Moved)
            } else {
                Some(MouseEventKind::Drag(button))
            }
        }
        EventFlags::MouseWheeled => {
            // Vertical scroll
            // from https://docs.microsoft.com/en-us/windows/console/mouse-event-record-str
            // if `button_state` is negative then the wheel was rotated backward, toward the user.
            if button_state.scroll_down() {
                Some(MouseEventKind::ScrollDown)
            } else if button_state.scroll_up() {
                Some(MouseEventKind::ScrollUp)
            } else {
                None
            }
        }
        EventFlags::MouseHwheeled => {
            if button_state.scroll_left() {
                Some(MouseEventKind::ScrollLeft)
            } else if button_state.scroll_right() {
                Some(MouseEventKind::ScrollRight)
            } else {
                None
            }
        }
        _ => None,
    };

    Ok(kind.map(|kind| MouseEvent {
        kind,
        column: xpos,
        row: ypos,
        modifiers,
    }))
}

#[cfg(test)]
mod tests {
    use crossterm_winapi::InputRecord;
    use winapi::um::wincon::{INPUT_RECORD, KEY_EVENT};

    use super::*;

    const VK_A: u16 = 0x41;
    const VK_C: u16 = 0x43;
    const VK_Z: u16 = 0x5a;

    fn key_record(
        virtual_key_code: u16,
        u_char: u16,
        control_key_state: u32,
        key_down: bool,
    ) -> KeyEventRecord {
        // SAFETY: INPUT_RECORD is plain old data, so all zeros is a valid value.
        let mut record: INPUT_RECORD = unsafe { std::mem::zeroed() };
        record.EventType = KEY_EVENT;
        // SAFETY: EventType selects the KeyEvent member of the union.
        unsafe {
            let key = record.Event.KeyEvent_mut();
            key.bKeyDown = i32::from(key_down);
            key.wRepeatCount = 1;
            key.wVirtualKeyCode = virtual_key_code;
            *key.uChar.UnicodeChar_mut() = u_char;
            key.dwControlKeyState = control_key_state;
        }
        let InputRecord::KeyEvent(key) = InputRecord::from(record) else {
            panic!("expected a key input record");
        };
        key
    }

    fn char_event(code: char, modifiers: KeyModifiers, key_down: bool) -> Option<Event> {
        let kind = if key_down {
            KeyEventKind::Press
        } else {
            KeyEventKind::Release
        };
        Some(Event::Key(KeyEvent::new_with_kind(
            KeyCode::Char(code),
            modifiers,
            kind,
        )))
    }

    #[test]
    fn ctrl_letter_control_codes_report_the_letter_on_any_layout() {
        // Windows reports Ctrl+A to Ctrl+Z as 0x01..=0x1a on every keyboard layout. Virtual key 0
        // has no layout character, so those records fail if the layout lookup is used instead,
        // whatever layout the test machine has. ConPTY reports these codes with virtual key
        // `code + 0x40`, which is the letter's own virtual key.
        let ctrl_shift = KeyModifiers::CONTROL | KeyModifiers::SHIFT;
        for control_code in 0x01..=0x1a_u16 {
            let letter = char::from(b'a' + (control_code - 1) as u8);
            let upper = letter.to_ascii_uppercase();
            let cases = [
                (0, letter, KeyModifiers::CONTROL),
                (CAPSLOCK_ON, letter, KeyModifiers::CONTROL),
                (SHIFT_PRESSED, upper, ctrl_shift),
                (SHIFT_PRESSED | CAPSLOCK_ON, upper, ctrl_shift),
            ];
            for (extra_state, code, modifiers) in cases {
                for ctrl in [LEFT_CTRL_PRESSED, RIGHT_CTRL_PRESSED] {
                    let state = ctrl | extra_state;
                    for virtual_key in [0, VK_A + control_code - 1] {
                        for key_down in [true, false] {
                            let record = key_record(virtual_key, control_code, state, key_down);
                            assert_eq!(
                                handle_key_event(record, &mut None),
                                char_event(code, modifiers, key_down),
                                "code {control_code:#x}, vk {virtual_key:#x}, state {state:#x}",
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn ctrl_letter_keys_without_a_character_report_the_virtual_key_letter() {
        // Some layouts, such as Armenian, report Ctrl+Shift+letter with no character. The virtual
        // key still names the letter. On a Latin layout the lookup returns the same letter, so
        // only the Caps Lock records fail if the layout lookup is used instead.
        let ctrl_shift = KeyModifiers::CONTROL | KeyModifiers::SHIFT;
        for virtual_key in VK_A..=VK_Z {
            let letter = char::from(b'a' + (virtual_key - VK_A) as u8);
            let upper = letter.to_ascii_uppercase();
            let cases = [
                (LEFT_CTRL_PRESSED, letter, KeyModifiers::CONTROL),
                (
                    RIGHT_CTRL_PRESSED | CAPSLOCK_ON,
                    letter,
                    KeyModifiers::CONTROL,
                ),
                (LEFT_CTRL_PRESSED | SHIFT_PRESSED, upper, ctrl_shift),
                (
                    RIGHT_CTRL_PRESSED | SHIFT_PRESSED | CAPSLOCK_ON,
                    upper,
                    ctrl_shift,
                ),
            ];
            for (state, code, modifiers) in cases {
                for key_down in [true, false] {
                    let record = key_record(virtual_key, 0, state, key_down);
                    assert_eq!(
                        handle_key_event(record, &mut None),
                        char_event(code, modifiers, key_down),
                        "vk {virtual_key:#x}, state {state:#x}",
                    );
                }
            }
        }
    }

    #[test]
    fn ctrl_alt_control_codes_keep_the_layout_lookup() {
        // Windows reports AltGr as Ctrl+Alt, and AltGr types layout characters, so Ctrl+Alt must
        // not take the Ctrl+letter paths. Virtual key 0 has no layout character, so the layout
        // lookup reports no event. This checks which path is taken, not a desired result.
        for state in [
            LEFT_CTRL_PRESSED | RIGHT_ALT_PRESSED,
            RIGHT_CTRL_PRESSED | LEFT_ALT_PRESSED,
        ] {
            for u_char in [0x00, 0x03] {
                assert_eq!(
                    handle_key_event(key_record(0, u_char, state, true), &mut None),
                    None,
                    "u_char {u_char:#x}, state {state:#x}",
                );
            }
        }
    }

    #[test]
    fn layout_characters_with_ctrl_are_reported_unchanged() {
        // Only control codes identify a Ctrl+letter. A record that carries the layout character
        // reports that character, including AltGr characters such as Polish 'ą'.
        let ctrl_alt = KeyModifiers::CONTROL | KeyModifiers::ALT;
        for (virtual_key, character) in [
            (VK_C, 'c'),
            (VK_C, '\u{441}'),
            (VK_C, '\u{581}'),
            (VK_A, '\u{105}'),
        ] {
            for (state, modifiers) in [
                (0, KeyModifiers::NONE),
                (LEFT_CTRL_PRESSED, KeyModifiers::CONTROL),
                (LEFT_CTRL_PRESSED | RIGHT_ALT_PRESSED, ctrl_alt),
            ] {
                let record = key_record(virtual_key, character as u16, state, true);
                assert_eq!(
                    handle_key_event(record, &mut None),
                    char_event(character, modifiers, true),
                    "character {character:?}, state {state:#x}",
                );
            }
        }
    }
}
