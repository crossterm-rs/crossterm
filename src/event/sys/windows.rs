//! This is a WINDOWS specific implementation for input related action.

use std::io;
use std::io::ErrorKind;
use std::ptr;
use std::sync::Mutex;
use std::time::Duration;

use crossterm_winapi::{
    ButtonState, ConsoleMode, ControlKeyState, EventFlags, Handle, KeyEventRecord, MouseEvent,
    ScreenBuffer,
};
use winapi::shared::winerror::WAIT_TIMEOUT;
use winapi::um::{
    handleapi::CloseHandle,
    synchapi::{CreateSemaphoreW, ReleaseSemaphore, WaitForMultipleObjects},
    winbase::{INFINITE, WAIT_ABANDONED_0, WAIT_FAILED, WAIT_OBJECT_0},
    winnt::HANDLE,
};
use winapi::um::{
    wincon::{
        LEFT_ALT_PRESSED, LEFT_CTRL_PRESSED, RIGHT_ALT_PRESSED, RIGHT_CTRL_PRESSED, SHIFT_PRESSED,
    },
    winuser::{
        VK_BACK, VK_CONTROL, VK_DELETE, VK_DOWN, VK_END, VK_ESCAPE, VK_F1, VK_F24, VK_HOME,
        VK_INSERT, VK_LEFT, VK_MENU, VK_NEXT, VK_PRIOR, VK_RETURN, VK_RIGHT, VK_SHIFT, VK_UP,
    },
};

//  VK_F2, VK_F3, VK_F4, VK_F5, VK_F6, VK_F7, VK_F8, VK_F9, VK_F10, VK_F11, VK_F12
use lazy_static::lazy_static;

use crate::{
    event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseButton},
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
        let shift = (state.has_state(SHIFT_PRESSED));
        let alt = (state.has_state((LEFT_ALT_PRESSED | RIGHT_ALT_PRESSED)));
        let control = (state.has_state(LEFT_CTRL_PRESSED | RIGHT_CTRL_PRESSED));

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

    // todo implement
    let shift = modifiers.contains(KeyModifiers::SHIFT);
    let ctrl = modifiers.contains(KeyModifiers::CONTROL);
    let alt = modifiers.contains(KeyModifiers::ALT);

    // NOTE (@imdaveho): xterm emulation takes the digits of the coords and passes them
    // individually as bytes into a buffer; the below cxbs and cybs replicates that and
    // mimicks the behavior; additionally, in xterm, mouse move is only handled when a
    // mouse button is held down (ie. mouse drag)
    let xpos = event.mouse_position.x as u16;
    let ypos = parse_relative_y(event.mouse_position.y)? as u16;

    Ok(match event.event_flags {
        EventFlags::PressOrRelease => {
            // Single click
            match event.button_state {
                ButtonState::Release => Some(crate::event::MouseEvent::Release(xpos, ypos)),
                ButtonState::FromLeft1stButtonPressed => {
                    // left click
                    Some(crate::event::MouseEvent::Press(
                        MouseButton::Left,
                        xpos,
                        ypos,
                    ))
                }
                ButtonState::RightmostButtonPressed => {
                    // right click
                    Some(crate::event::MouseEvent::Press(
                        MouseButton::Right,
                        xpos,
                        ypos,
                    ))
                }
                ButtonState::FromLeft2ndButtonPressed => {
                    // middle click
                    Some(crate::event::MouseEvent::Press(
                        MouseButton::Middle,
                        xpos,
                        ypos,
                    ))
                }
                _ => None,
            }
        }
        EventFlags::MouseMoved => {
            // Click + Move
            // NOTE (@imdaveho) only register when mouse is not released
            if event.button_state != ButtonState::Release {
                Some(crate::event::MouseEvent::Hold(xpos, ypos))
            } else {
                None
            }
        }
        EventFlags::MouseWheeled => {
            // Vertical scroll
            // NOTE (@imdaveho) from https://docs.microsoft.com/en-us/windows/console/mouse-event-record-str
            // if `button_state` is negative then the wheel was rotated backward, toward the user.
            if event.button_state != ButtonState::Negative {
                Some(crate::event::MouseEvent::Press(
                    MouseButton::WheelUp,
                    xpos,
                    ypos,
                ))
            } else {
                Some(crate::event::MouseEvent::Press(
                    MouseButton::WheelDown,
                    xpos,
                    ypos,
                ))
            }
        }
        EventFlags::DoubleClick => None, // NOTE (@imdaveho): double click not supported by unix terminals
        EventFlags::MouseHwheeled => None, // NOTE (@imdaveho): horizontal scroll not supported by unix terminals
                                           // TODO: Handle Ctrl + Mouse, Alt + Mouse, etc.
    })
}

pub(crate) struct WinApiPoll {
    semaphore: Option<Semaphore>,
}

impl WinApiPoll {
    pub(crate) fn new() -> Result<WinApiPoll> {
        Ok(WinApiPoll { semaphore: None })
    }
}

impl WinApiPoll {
    pub fn poll(&mut self, timeout: Option<Duration>) -> Result<Option<bool>> {
        let dw_millis = if let Some(duration) = timeout {
            duration.as_millis() as u32
        } else {
            INFINITE
        };

        let semaphore = Semaphore::new()?;
        let console_handle = Handle::current_in_handle()?;
        let handles = &[*console_handle, semaphore.handle()];

        self.semaphore = Some(semaphore);

        let output =
            unsafe { WaitForMultipleObjects(handles.len() as u32, handles.as_ptr(), 0, dw_millis) };

        let result = match output {
            output if output == WAIT_OBJECT_0 + 0 => {
                // input handle triggered
                Ok(Some(true))
            }
            output if output == WAIT_OBJECT_0 + 1 => {
                // semaphore handle triggered
                Ok(None)
            }
            WAIT_TIMEOUT | WAIT_ABANDONED_0 => {
                // timeout elapsed
                Ok(None)
            }
            WAIT_FAILED => return Err(io::Error::last_os_error())?,
            _ => Err(io::Error::new(
                ErrorKind::Other,
                "WaitForMultipleObjects returned unexpected result.",
            ))?,
        };

        self.semaphore = None;

        result
    }

    pub fn cancel(&self) -> Result<()> {
        if let Some(semaphore) = &self.semaphore {
            semaphore.release()?
        }

        Ok(())
    }
}

// HANDLE can be send
unsafe impl Send for WinApiPoll {}
// HANDLE can be sync
unsafe impl Sync for WinApiPoll {}

//// TODO, maybe move to crossterm_winapi
struct Semaphore(HANDLE);

impl Semaphore {
    /// Construct a new semaphore.
    pub fn new() -> io::Result<Self> {
        let handle = unsafe { CreateSemaphoreW(ptr::null_mut(), 0, 1, ptr::null_mut()) };

        if handle == ptr::null_mut() {
            return Err(io::Error::last_os_error());
        }

        Ok(Self(handle))
    }

    /// Release a permit on the semaphore.
    pub fn release(&self) -> io::Result<()> {
        let result = unsafe { ReleaseSemaphore(self.0, 1, ptr::null_mut()) };

        if result == 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    /// Access the underlying handle to the semaphore.
    pub fn handle(&self) -> HANDLE {
        self.0
    }
}

impl Drop for Semaphore {
    fn drop(&mut self) {
        assert!(
            unsafe { CloseHandle(self.0) } != 0,
            "failed to close handle"
        );
    }
}

unsafe impl Send for Semaphore {}

unsafe impl Sync for Semaphore {}
