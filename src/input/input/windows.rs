//! This is a WINDOWS specific implementation for input related action.

use std::{char, collections::VecDeque, io, sync::Mutex};

use crossterm_winapi::{
    ButtonState, Console, ConsoleMode, EventFlags, Handle, InputEventType, KeyEventRecord,
    MouseEvent, ScreenBuffer,
};
use winapi::um::{
    wincon::{
        LEFT_ALT_PRESSED, LEFT_CTRL_PRESSED, RIGHT_ALT_PRESSED, RIGHT_CTRL_PRESSED, SHIFT_PRESSED,
    },
    winnt::INT,
    winuser::{
        VK_BACK, VK_CONTROL, VK_DELETE, VK_DOWN, VK_END, VK_ESCAPE, VK_F1, VK_F10, VK_F11, VK_F12,
        VK_F2, VK_F3, VK_F4, VK_F5, VK_F6, VK_F7, VK_F8, VK_F9, VK_HOME, VK_INSERT, VK_LEFT,
        VK_MENU, VK_NEXT, VK_PRIOR, VK_RETURN, VK_RIGHT, VK_SHIFT, VK_UP,
    },
};

use lazy_static::lazy_static;

use crate::input::{input::Input, InputEvent, KeyEvent, MouseButton};
use crate::utils::Result;

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

pub(crate) struct WindowsInput;

impl WindowsInput {
    pub fn new() -> WindowsInput {
        WindowsInput
    }
}

impl Input for WindowsInput {
    fn read_char(&self) -> Result<char> {
        // _getwch is without echo and _getwche is with echo
        let pressed_char = unsafe { _getwche() };

        // we could return error but maybe option to keep listening until valid character is inputted.
        if pressed_char == 0 || pressed_char == 0xe0 {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "Given input char is not a valid char, mostly occurs when pressing special keys",
            ))?;
        }

        let ch = char::from_u32(pressed_char as u32).ok_or_else(|| {
            io::Error::new(io::ErrorKind::Other, "Could not parse given input to char")
        })?;

        Ok(ch)
    }

    fn read_async(&self) -> AsyncReader {
        let handle = Handle::current_in_handle().expect("failed to create console input handle");
        let console = Console::from(handle);
        AsyncReader::new(console, None)
    }

    fn read_until_async(&self, delimiter: u8) -> AsyncReader {
        let handle = Handle::current_in_handle().expect("failed to create console input handle");
        let console = Console::from(handle);
        AsyncReader::new(console, Some(delimiter))
    }

    fn read_sync(&self) -> SyncReader {
        SyncReader
    }

    fn enable_mouse_mode(&self) -> Result<()> {
        let mode = ConsoleMode::from(Handle::current_in_handle()?);

        init_original_console_mode(mode.mode()?);
        mode.set_mode(ENABLE_MOUSE_MODE)?;

        Ok(())
    }

    fn disable_mouse_mode(&self) -> Result<()> {
        let mode = ConsoleMode::from(Handle::current_in_handle()?);
        mode.set_mode(original_console_mode())?;
        Ok(())
    }
}

/// A synchronous input reader (blocking).
///
/// `SyncReader` implements the [`Iterator`](https://doc.rust-lang.org/std/iter/index.html#iterator)
/// trait. Documentation says:
///
/// > An iterator has a method, `next`, which when called, returns an `Option<Item>`. `next` will return
/// > `Some(Item)` as long as there are elements, and once they've all been exhausted, will return `None`
/// > to indicate that iteration is finished. Individual iterators may choose to resume iteration, and
/// > so calling `next` again may or may not eventually start returning `Some(Item)` again at some point.
///
/// `SyncReader` is an individual iterator and it doesn't use `None` to indicate that the iteration is
/// finished. You can expect additional `Some(InputEvent)` after calling `next` even if you have already
/// received `None`. Unfortunately, `None` means that an error occurred, but you're free to call `next`
/// again. This behavior will be changed in the future to avoid errors consumption.  
///
/// # Notes
///
/// * It requires enabled raw mode (see the
///   [`crossterm_screen`](https://docs.rs/crossterm_screen/) crate documentation to learn more).
/// * See the [`AsyncReader`](struct.AsyncReader.html) if you want a non blocking reader.
///
/// # Examples
///
/// ```no_run
/// use std::{thread, time::Duration};
///
/// use crossterm::{screen::RawScreen,  input::{input, InputEvent, KeyEvent}};
///
/// fn main() {
///     println!("Press 'ESC' to quit.");
///
///     // Enable raw mode and keep the `_raw` around otherwise the raw mode will be disabled
///     let _raw = RawScreen::into_raw_mode();
///
///     // Create an input from our screen
///     let input = input();
///
///     // Create a sync reader
///     let mut reader = input.read_sync();
///
///     loop {
///         if let Some(event) = reader.next() { // Blocking call
///             match event {
///                 InputEvent::Keyboard(KeyEvent::Esc) => {
///                     println!("Program closing ...");
///                     break;
///                  }
///                  InputEvent::Mouse(event) => { /* Mouse event */ }
///                  _ => { /* Other events */ }
///             }
///         }
///         thread::sleep(Duration::from_millis(50));
///     }
/// } // `_raw` dropped <- raw mode disabled
/// ```
pub struct SyncReader;

impl Iterator for SyncReader {
    type Item = InputEvent;

    /// Tries to read the next input event (blocking).
    ///
    /// `None` doesn't mean that the iteration is finished. See the
    /// [`SyncReader`](struct.SyncReader.html) documentation for more information.
    fn next(&mut self) -> Option<Self::Item> {
        // This synces the behaviour with the unix::SyncReader (& documentation) where
        // None is returned in case of error.
        read_single_event().unwrap_or(None)
    }
}

/// An asynchronous input reader (not blocking).
///
/// `AsyncReader` implements the [`Iterator`](https://doc.rust-lang.org/std/iter/index.html#iterator)
/// trait. Documentation says:
///
/// > An iterator has a method, `next`, which when called, returns an `Option<Item>`. `next` will return
/// > `Some(Item)` as long as there are elements, and once they've all been exhausted, will return `None`
/// > to indicate that iteration is finished. Individual iterators may choose to resume iteration, and
/// > so calling `next` again may or may not eventually start returning `Some(Item)` again at some point.
///
/// `AsyncReader` is an individual iterator and it doesn't use `None` to indicate that the iteration is
/// finished. You can expect additional `Some(InputEvent)` after calling `next` even if you have already
/// received `None`.
///
/// # Notes
///
/// * It requires enabled raw mode (see the
///   [`crossterm_screen`](https://docs.rs/crossterm_screen/) crate documentation to learn more).
/// * A thread is spawned to read the input.
/// * The reading thread is cleaned up when you drop the `AsyncReader`.
/// * See the [`SyncReader`](struct.SyncReader.html) if you want a blocking,
///   or a less resource hungry reader.
///
/// # Examples
///
/// ```no_run
/// use std::{thread, time::Duration};
///
/// use crossterm::{screen::RawScreen,  input::{input, InputEvent, KeyEvent}};
///
/// fn main() {
///     println!("Press 'ESC' to quit.");
///
///     // Enable raw mode and keep the `_raw` around otherwise the raw mode will be disabled
///     let _raw = RawScreen::into_raw_mode();
///
///     // Create an input from our screen
///     let input = input();
///
///     // Create an async reader
///     let mut reader = input.read_async();
///
///     loop {
///         if let Some(event) = reader.next() { // Not a blocking call
///             match event {
///                 InputEvent::Keyboard(KeyEvent::Esc) => {
///                     println!("Program closing ...");
///                     break;
///                  }
///                  InputEvent::Mouse(event) => { /* Mouse event */ }
///                  _ => { /* Other events */ }
///             }
///         }
///         thread::sleep(Duration::from_millis(50));
///     }
/// } // `reader` dropped <- thread cleaned up, `_raw` dropped <- raw mode disabled
/// ```
pub struct AsyncReader {
    console: Console,
    buffer: VecDeque<InputEvent>,
    delimiter: Option<u8>,
}

impl AsyncReader {
    // TODO Should the new() really be public?
    /// Creates a new `AsyncReader`.
    ///
    /// # Notes
    ///
    /// * A thread is spawned to read the input.
    /// * The reading thread is cleaned up when you drop the `AsyncReader`.
    pub fn new(console: Console, delimiter: Option<u8>) -> AsyncReader {
        AsyncReader {
            console,
            buffer: VecDeque::new(),
            delimiter,
        }
    }
}

impl Iterator for AsyncReader {
    type Item = InputEvent;

    /// Tries to read the next input event (not blocking).
    ///
    /// `None` doesn't mean that the iteration is finished. See the
    /// [`AsyncReader`](struct.AsyncReader.html) documentation for more information.
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.buffer.is_empty() {
                let (_, events) = read_input_events(&self.console).expect("read failed");

                if events.is_empty() {
                    return None;
                }

                self.buffer.extend(events);
            }

            if let Some(delimiter) = self.delimiter {
                while let Some(e) = self.buffer.pop_front() {
                    if let InputEvent::Keyboard(KeyEvent::Char(key)) = e {
                        if (key as u8) == delimiter {
                            return Some(e);
                        }
                    }
                }

                continue;
            }

            return self.buffer.pop_front();
        }
    }
}

extern "C" {
    fn _getwche() -> INT;
}

fn read_single_event() -> Result<Option<InputEvent>> {
    let console = Console::from(Handle::current_in_handle()?);

    let input = console.read_single_input_event()?;

    match input.event_type {
        InputEventType::KeyEvent => {
            handle_key_event(unsafe { KeyEventRecord::from(*input.event.KeyEvent()) })
        }
        InputEventType::MouseEvent => {
            handle_mouse_event(unsafe { MouseEvent::from(*input.event.MouseEvent()) })
        }
        // NOTE (@imdaveho): ignore below
        InputEventType::WindowBufferSizeEvent => return Ok(None), // TODO implement terminal resize event
        InputEventType::FocusEvent => Ok(None),
        InputEventType::MenuEvent => Ok(None),
    }
}

/// partially inspired by: https://github.com/retep998/wio-rs/blob/master/src/console.rs#L130
fn read_input_events(console: &Console) -> Result<(u32, Vec<InputEvent>)> {
    let result = console.read_console_input()?;

    let mut input_events = Vec::with_capacity(result.0 as usize);

    for input in result.1 {
        match input.event_type {
            InputEventType::KeyEvent => {
                if let Ok(Some(event)) =
                    handle_key_event(unsafe { KeyEventRecord::from(*input.event.KeyEvent()) })
                {
                    input_events.push(event)
                }
            }
            InputEventType::MouseEvent => {
                if let Ok(Some(event)) =
                    handle_mouse_event(unsafe { MouseEvent::from(*input.event.MouseEvent()) })
                {
                    input_events.push(event)
                }
            }
            // NOTE (@imdaveho): ignore below
            InputEventType::WindowBufferSizeEvent => (), // TODO implement terminal resize event
            InputEventType::FocusEvent => (),
            InputEventType::MenuEvent => (),
        }
    }

    return Ok((result.0, input_events));
}

fn handle_mouse_event(mouse_event: MouseEvent) -> Result<Option<InputEvent>> {
    if let Ok(Some(event)) = parse_mouse_event_record(&mouse_event) {
        return Ok(Some(InputEvent::Mouse(event)));
    }
    Ok(None)
}

fn handle_key_event(key_event: KeyEventRecord) -> Result<Option<InputEvent>> {
    if key_event.key_down {
        if let Some(event) = parse_key_event_record(&key_event) {
            return Ok(Some(InputEvent::Keyboard(event)));
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

fn parse_mouse_event_record(event: &MouseEvent) -> Result<Option<crate::input::MouseEvent>> {
    // NOTE (@imdaveho): xterm emulation takes the digits of the coords and passes them
    // individually as bytes into a buffer; the below cxbs and cybs replicates that and
    // mimicks the behavior; additionally, in xterm, mouse move is only handled when a
    // mouse button is held down (ie. mouse drag)

    let window_size = ScreenBuffer::current()?.info()?.terminal_window();

    let xpos = event.mouse_position.x;
    let mut ypos = event.mouse_position.y;

    // The 'y' position of a mouse event is not relative to the window but absolute to screen buffer.
    // This means that when the mouse cursor is at the top left it will be x: 0, y: 2295 (e.g. y = number of cells counting from the absolute buffer height) instead of relative x: 0, y: 0 to the window.

    ypos = ypos - window_size.top;

    Ok(match event.event_flags {
        EventFlags::PressOrRelease => {
            // Single click
            match event.button_state {
                ButtonState::Release => {
                    Some(crate::input::MouseEvent::Release(xpos as u16, ypos as u16))
                }
                ButtonState::FromLeft1stButtonPressed => {
                    // left click
                    Some(crate::input::MouseEvent::Press(
                        MouseButton::Left,
                        xpos as u16,
                        ypos as u16,
                    ))
                }
                ButtonState::RightmostButtonPressed => {
                    // right click
                    Some(crate::input::MouseEvent::Press(
                        MouseButton::Right,
                        xpos as u16,
                        ypos as u16,
                    ))
                }
                ButtonState::FromLeft2ndButtonPressed => {
                    // middle click
                    Some(crate::input::MouseEvent::Press(
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
                Some(crate::input::MouseEvent::Hold(xpos as u16, ypos as u16))
            } else {
                None
            }
        }
        EventFlags::MouseWheeled => {
            // Vertical scroll
            // NOTE (@imdaveho) from https://docs.microsoft.com/en-us/windows/console/mouse-event-record-str
            // if `button_state` is negative then the wheel was rotated backward, toward the user.
            if event.button_state != ButtonState::Negative {
                Some(crate::input::MouseEvent::Press(
                    MouseButton::WheelUp,
                    xpos as u16,
                    ypos as u16,
                ))
            } else {
                Some(crate::input::MouseEvent::Press(
                    MouseButton::WheelDown,
                    xpos as u16,
                    ypos as u16,
                ))
            }
        }
        EventFlags::DoubleClick => None, // NOTE (@imdaveho): double click not supported by unix terminals
        EventFlags::MouseHwheeled => None, // NOTE (@imdaveho): horizontal scroll not supported by unix terminals
                                           // TODO: Handle Ctrl + Mouse, Alt + Mouse, etc.
    })
}
