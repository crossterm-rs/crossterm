//! This is a WINDOWS specific implementation for input related action.

use std::{io, io::ErrorKind, sync::Mutex, time::Duration};

use crossterm_winapi::{
    ConsoleMode, ControlKeyState, EventFlags, Handle, KeyEventRecord, MouseEvent, ScreenBuffer,
};
use winapi::{
    shared::winerror::WAIT_TIMEOUT,
    um::{
        synchapi::WaitForMultipleObjects,
        winbase::{INFINITE, WAIT_ABANDONED_0, WAIT_FAILED, WAIT_OBJECT_0},
        wincon::{
            LEFT_ALT_PRESSED, LEFT_CTRL_PRESSED, RIGHT_ALT_PRESSED, RIGHT_CTRL_PRESSED,
            SHIFT_PRESSED,
        },
        winuser::{
            VK_BACK, VK_CONTROL, VK_DELETE, VK_DOWN, VK_END, VK_ESCAPE, VK_F1, VK_F24, VK_HOME,
            VK_INSERT, VK_LEFT, VK_MENU, VK_NEXT, VK_PRIOR, VK_RETURN, VK_RIGHT, VK_SHIFT, VK_UP,
        },
    },
};

use lazy_static::lazy_static;

use crate::{
    event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseButton},
    Result,
};
#[cfg(feature = "event-stream")]
pub(crate) use waker::Waker;

#[cfg(feature = "event-stream")]
mod waker;

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
    if let Ok(Some(event)) = parse_mouse_event_record(&mouse_event) {
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

    Ok(None)
}

impl From<ControlKeyState> for KeyModifiers {
    fn from(state: ControlKeyState) -> Self {
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

fn parse_key_event_record(key_event: &KeyEventRecord) -> Option<KeyEvent> {
    let modifiers = KeyModifiers::from(key_event.control_key_state);

    let key_code = key_event.virtual_key_code as i32;

    let parse_result = match key_code {
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
        _ => {
            // Modifier Keys (Ctrl, Alt, Shift) Support
            let character_raw = { (unsafe { *key_event.u_char.UnicodeChar() } as u16) };

            if character_raw < 255 {
                let mut character = character_raw as u8 as char;

                if modifiers.contains(KeyModifiers::ALT) {
                    // If the ALT key is held down, pressing the A key produces ALT+A, which the system does not treat as a character at all, but rather as a system command.
                    // The pressed command is stored in `virtual_key_code`.
                    let command = key_event.virtual_key_code as u8 as char;

                    if command.is_alphabetic() {
                        character = command;
                    } else {
                        return None;
                    }
                } else if modifiers.contains(KeyModifiers::CONTROL) {
                    // we need to do some parsing
                    character = match character_raw as u8 {
                        c @ b'\x01'..=b'\x1A' => (c as u8 - 0x1 + b'a') as char,
                        c @ b'\x1C'..=b'\x1F' => (c as u8 - 0x1C + b'4') as char,
                        _ => return None,
                    }
                }

                if modifiers.contains(KeyModifiers::SHIFT) && character == '\t' {
                    Some(KeyCode::BackTab)
                } else if character == '\t' {
                    Some(KeyCode::Tab)
                } else {
                    Some(KeyCode::Char(character))
                }
            } else {
                None
            }
        }
    };

    if let Some(key_code) = parse_result {
        return Some(KeyEvent::new(key_code, modifiers));
    }

    None
}

// The 'y' position of a mouse event or resize event is not relative to the window but absolute to screen buffer.
// This means that when the mouse cursor is at the top left it will be x: 0, y: 2295 (e.g. y = number of cells conting from the absolute buffer height) instead of relative x: 0, y: 0 to the window.
pub fn parse_relative_y(y: i16) -> Result<i16> {
    let window_size = ScreenBuffer::current()?.info()?.terminal_window();
    Ok(y - window_size.top)
}

fn parse_mouse_event_record(event: &MouseEvent) -> Result<Option<crate::event::MouseEvent>> {
    let modifiers = KeyModifiers::from(event.control_key_state);

    let xpos = event.mouse_position.x as u16;
    let ypos = parse_relative_y(event.mouse_position.y)? as u16;

    let button_state = event.button_state;
    let button = if button_state.right_button() {
        MouseButton::Right
    } else if button_state.middle_button() {
        MouseButton::Middle
    } else {
        MouseButton::Left
    };

    Ok(match event.event_flags {
        EventFlags::PressOrRelease => {
            if button_state.release_button() {
                // in order to read the up button type, we have to check the last down input record.
                Some(crate::event::MouseEvent::Up(
                    MouseButton::Left,
                    xpos,
                    ypos,
                    modifiers,
                ))
            } else {
                Some(crate::event::MouseEvent::Down(
                    button, xpos, ypos, modifiers,
                ))
            }
        }
        EventFlags::MouseMoved => {
            // Click + Move
            // Only register when mouse is not released
            // because unix systems share this behaviour.
            if !button_state.release_button() {
                Some(crate::event::MouseEvent::Drag(
                    button, xpos, ypos, modifiers,
                ))
            } else {
                None
            }
        }
        EventFlags::MouseWheeled => {
            // Vertical scroll
            // from https://docs.microsoft.com/en-us/windows/console/mouse-event-record-str
            // if `button_state` is negative then the wheel was rotated backward, toward the user.
            if button_state.scroll_down() {
                Some(crate::event::MouseEvent::ScrollDown(xpos, ypos, modifiers))
            } else if button_state.scroll_up() {
                Some(crate::event::MouseEvent::ScrollUp(xpos, ypos, modifiers))
            } else {
                None
            }
        }
        EventFlags::DoubleClick => None, // double click not supported by unix terminals
        EventFlags::MouseHwheeled => None, // horizontal scroll not supported by unix terminals
    })
}

pub(crate) struct WinApiPoll {
    #[cfg(feature = "event-stream")]
    waker: Waker,
}

impl WinApiPoll {
    #[cfg(not(feature = "event-stream"))]
    pub(crate) fn new() -> Result<WinApiPoll> {
        Ok(WinApiPoll {})
    }

    #[cfg(feature = "event-stream")]
    pub(crate) fn new() -> Result<WinApiPoll> {
        Ok(WinApiPoll {
            waker: Waker::new()?,
        })
    }
}

impl WinApiPoll {
    pub fn poll(&mut self, timeout: Option<Duration>) -> Result<Option<bool>> {
        let dw_millis = if let Some(duration) = timeout {
            duration.as_millis() as u32
        } else {
            INFINITE
        };

        let console_handle = Handle::current_in_handle()?;

        cfg_if::cfg_if! {
            if #[cfg(feature = "event-stream")] {
                let semaphore = self.waker.semaphore();
                let handles = &[*console_handle, **semaphore.handle()];
            } else {
                let handles = &[*console_handle];
            }
        }
        let output =
            unsafe { WaitForMultipleObjects(handles.len() as u32, handles.as_ptr(), 0, dw_millis) };

        match output {
            output if output == WAIT_OBJECT_0 => {
                // input handle triggered
                Ok(Some(true))
            }
            #[cfg(feature = "event-stream")]
            output if output == WAIT_OBJECT_0 + 1 => {
                // semaphore handle triggered
                let _ = self.waker.reset();
                Ok(None)
            }
            WAIT_TIMEOUT | WAIT_ABANDONED_0 => {
                // timeout elapsed
                Ok(None)
            }
            WAIT_FAILED => Err(io::Error::last_os_error().into()),
            _ => Err(io::Error::new(
                ErrorKind::Other,
                "WaitForMultipleObjects returned unexpected result.",
            )
            .into()),
        }
    }

    #[cfg(feature = "event-stream")]
    pub fn poll_waker(&self) -> Waker {
        self.waker.clone()
    }
}
