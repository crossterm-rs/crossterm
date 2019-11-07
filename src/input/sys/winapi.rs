//! This is a WINDOWS specific implementation for input related action.

use std::sync::Mutex;

use crossterm_winapi::{ButtonState, ConsoleMode, EventFlags, Handle, KeyEventRecord, MouseEvent};
use winapi::um::{
    wincon::{
        LEFT_ALT_PRESSED, LEFT_CTRL_PRESSED, RIGHT_ALT_PRESSED, RIGHT_CTRL_PRESSED, SHIFT_PRESSED,
    },
    winuser::{
        VK_BACK, VK_CONTROL, VK_DELETE, VK_DOWN, VK_END, VK_ESCAPE, VK_F1, VK_F10, VK_F11, VK_F12,
        VK_F2, VK_F3, VK_F4, VK_F5, VK_F6, VK_F7, VK_F8, VK_F9, VK_HOME, VK_INSERT, VK_LEFT,
        VK_MENU, VK_NEXT, VK_PRIOR, VK_RETURN, VK_RIGHT, VK_SHIFT, VK_UP,
    },
};

use lazy_static::lazy_static;

use crate::{
    input::{self, Event, KeyEvent, MouseButton},
    Result,
};

const ENABLE_MOUSE_MODE: u32 = 0x0010 | 0x0080 | 0x0008;

lazy_static! {
    static ref ORIGINAL_CONSOLE_MODE: Mutex<Option<u32>> = Mutex::new(None);
}

/// Initializes the default console color. It will will be skipped if it has already been initialized.
fn init_original_console_mode(original_mode: u32) {
    let mut lock = ORIGINAL_CONSOLE_MODE.lock().unwrap();

    if lock.is_none() {
        *lock = Some(original_mode);
    }
}

/// Returns the original console color, make sure to call `init_console_color` before calling this function. Otherwise this function will panic.
fn original_console_mode() -> u32 {
    // safe unwrap, initial console color was set with `init_console_color` in `WinApiColor::new()`
    ORIGINAL_CONSOLE_MODE
        .lock()
        .unwrap()
        .expect("Original console mode not set")
}

pub(crate) fn enable_mouse_capture() -> Result<()> {
    let mode = ConsoleMode::from(Handle::current_in_handle()?);
    init_original_console_mode(mode.mode()?);
    mode.set_mode(ENABLE_MOUSE_MODE)?;

    Ok(())
}

pub(crate) fn disable_mouse_capture() -> Result<()> {
    let mode = ConsoleMode::from(Handle::current_in_handle()?);
    mode.set_mode(original_console_mode())?;
    Ok(())
}

pub(crate) fn handle_mouse_event(mouse_event: MouseEvent) -> Result<Option<Event>> {
    if let Some(event) = parse_mouse_event_record(&mouse_event) {
        return Ok(Some(Event::Mouse(event)));
    }
    Ok(None)
}

pub(crate) fn handle_key_event(key_event: KeyEventRecord) -> Result<Option<Event>> {
    if key_event.key_down {
        if let Some(event) = parse_key_event_record(&key_event) {
            return Ok(Some(Event::Key(event)));
        }
    }

    return Ok(None);
}

fn parse_key_event_record(key_event: &KeyEventRecord) -> Option<KeyEvent> {
    let key_code = key_event.virtual_key_code as i32;
    match key_code {
        VK_SHIFT | VK_CONTROL | VK_MENU => None,
        VK_BACK => Some(KeyEvent::Backspace),
        VK_ESCAPE => Some(KeyEvent::Esc),
        VK_RETURN => Some(KeyEvent::Enter),
        VK_F1 | VK_F2 | VK_F3 | VK_F4 | VK_F5 | VK_F6 | VK_F7 | VK_F8 | VK_F9 | VK_F10 | VK_F11
        | VK_F12 => Some(KeyEvent::F((key_event.virtual_key_code - 111) as u8)),
        VK_LEFT | VK_UP | VK_RIGHT | VK_DOWN => {
            // Modifier Keys (Ctrl, Shift) Support
            let key_state = &key_event.control_key_state;
            let ctrl_pressed = key_state.has_state(RIGHT_CTRL_PRESSED | LEFT_CTRL_PRESSED);
            let shift_pressed = key_state.has_state(SHIFT_PRESSED);

            let event = match key_code {
                VK_LEFT => {
                    if ctrl_pressed {
                        Some(KeyEvent::CtrlLeft)
                    } else if shift_pressed {
                        Some(KeyEvent::ShiftLeft)
                    } else {
                        Some(KeyEvent::Left)
                    }
                }
                VK_UP => {
                    if ctrl_pressed {
                        Some(KeyEvent::CtrlUp)
                    } else if shift_pressed {
                        Some(KeyEvent::ShiftUp)
                    } else {
                        Some(KeyEvent::Up)
                    }
                }
                VK_RIGHT => {
                    if ctrl_pressed {
                        Some(KeyEvent::CtrlRight)
                    } else if shift_pressed {
                        Some(KeyEvent::ShiftRight)
                    } else {
                        Some(KeyEvent::Right)
                    }
                }
                VK_DOWN => {
                    if ctrl_pressed {
                        Some(KeyEvent::CtrlDown)
                    } else if shift_pressed {
                        Some(KeyEvent::ShiftDown)
                    } else {
                        Some(KeyEvent::Down)
                    }
                }
                _ => None,
            };

            event
        }
        VK_PRIOR | VK_NEXT => {
            if key_code == VK_PRIOR {
                Some(KeyEvent::PageUp)
            } else if key_code == VK_NEXT {
                Some(KeyEvent::PageDown)
            } else {
                None
            }
        }
        VK_END | VK_HOME => {
            if key_code == VK_HOME {
                Some(KeyEvent::Home)
            } else if key_code == VK_END {
                Some(KeyEvent::End)
            } else {
                None
            }
        }
        VK_DELETE => Some(KeyEvent::Delete),
        VK_INSERT => Some(KeyEvent::Insert),
        _ => {
            // Modifier Keys (Ctrl, Alt, Shift) Support
            let character_raw = { (unsafe { *key_event.u_char.UnicodeChar() } as u16) };

            if character_raw < 255 {
                let character = character_raw as u8 as char;

                let key_state = &key_event.control_key_state;

                if key_state.has_state(LEFT_ALT_PRESSED | RIGHT_ALT_PRESSED) {
                    // If the ALT key is held down, pressing the A key produces ALT+A, which the system does not treat as a character at all, but rather as a system command.
                    // The pressed command is stored in `virtual_key_code`.
                    let command = key_event.virtual_key_code as u8 as char;

                    if (command).is_alphabetic() {
                        Some(KeyEvent::Alt(command))
                    } else {
                        None
                    }
                } else if key_state.has_state(LEFT_CTRL_PRESSED | RIGHT_CTRL_PRESSED) {
                    match character_raw as u8 {
                        c @ b'\x01'..=b'\x1A' => {
                            Some(KeyEvent::Ctrl((c as u8 - 0x1 + b'a') as char))
                        }
                        c @ b'\x1C'..=b'\x1F' => {
                            Some(KeyEvent::Ctrl((c as u8 - 0x1C + b'4') as char))
                        }
                        _ => None,
                    }
                } else if key_state.has_state(SHIFT_PRESSED) && character == '\t' {
                    Some(KeyEvent::BackTab)
                } else {
                    if character == '\t' {
                        Some(KeyEvent::Tab)
                    } else {
                        // Shift + key press, essentially the same as single key press
                        // Separating to be explicit about the Shift press.
                        Some(KeyEvent::Char(character))
                    }
                }
            } else {
                None
            }
        }
    }
}

fn parse_mouse_event_record(event: &MouseEvent) -> Option<input::MouseEvent> {
    // NOTE (@imdaveho): xterm emulation takes the digits of the coords and passes them
    // individually as bytes into a buffer; the below cxbs and cybs replicates that and
    // mimicks the behavior; additionally, in xterm, mouse move is only handled when a
    // mouse button is held down (ie. mouse drag)

    // Windows returns (0, 0) for upper/left
    let xpos = event.mouse_position.x;
    let ypos = event.mouse_position.y;

    // TODO (@imdaveho): check if linux only provides coords for visible terminal window vs the total buffer

    match event.event_flags {
        EventFlags::PressOrRelease => {
            // Single click
            match event.button_state {
                ButtonState::Release => Some(input::MouseEvent::Release(xpos as u16, ypos as u16)),
                ButtonState::FromLeft1stButtonPressed => {
                    // left click
                    Some(input::MouseEvent::Press(
                        MouseButton::Left,
                        xpos as u16,
                        ypos as u16,
                    ))
                }
                ButtonState::RightmostButtonPressed => {
                    // right click
                    Some(input::MouseEvent::Press(
                        MouseButton::Right,
                        xpos as u16,
                        ypos as u16,
                    ))
                }
                ButtonState::FromLeft2ndButtonPressed => {
                    // middle click
                    Some(input::MouseEvent::Press(
                        MouseButton::Middle,
                        xpos as u16,
                        ypos as u16,
                    ))
                }
                _ => None,
            }
        }
        EventFlags::MouseMoved => {
            // Click + Move
            // NOTE (@imdaveho) only register when mouse is not released
            if event.button_state != ButtonState::Release {
                Some(input::MouseEvent::Hold(xpos as u16, ypos as u16))
            } else {
                None
            }
        }
        EventFlags::MouseWheeled => {
            // Vertical scroll
            // NOTE (@imdaveho) from https://docs.microsoft.com/en-us/windows/console/mouse-event-record-str
            // if `button_state` is negative then the wheel was rotated backward, toward the user.
            if event.button_state != ButtonState::Negative {
                Some(input::MouseEvent::Press(
                    MouseButton::WheelUp,
                    xpos as u16,
                    ypos as u16,
                ))
            } else {
                Some(input::MouseEvent::Press(
                    MouseButton::WheelDown,
                    xpos as u16,
                    ypos as u16,
                ))
            }
        }
        EventFlags::DoubleClick => None, // NOTE (@imdaveho): double click not supported by unix terminals
        EventFlags::MouseHwheeled => None, // NOTE (@imdaveho): horizontal scroll not supported by unix terminals
                                           // TODO: Handle Ctrl + Mouse, Alt + Mouse, etc.
    }
}
