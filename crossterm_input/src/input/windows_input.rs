//! This is a WINDOWS specific implementation for input related action.

use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::{self, Receiver, Sender},
    Arc,
};
use std::time::Duration;
use std::{char, io, thread};

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

use crossterm_utils::Result;
use crossterm_winapi::{
    ButtonState, Console, ConsoleMode, EventFlags, Handle, InputEventType, KeyEventRecord,
    MouseEvent,
};

use super::{ITerminalInput, InputEvent, KeyEvent, MouseButton};

pub struct WindowsInput;

impl WindowsInput {
    pub fn new() -> WindowsInput {
        WindowsInput
    }
}

const ENABLE_MOUSE_MODE: u32 = 0x0010 | 0x0080 | 0x0008;

// NOTE (@imdaveho): this global var is terrible -> move it elsewhere...
static mut ORIG_MODE: u32 = 0;

impl ITerminalInput for WindowsInput {
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
        AsyncReader::new(Box::new(move |event_tx, cancellation_token| loop {
            for i in read_input_events().unwrap().1 {
                if event_tx.send(i).is_err() {
                    return;
                }
            }

            if cancellation_token.load(Ordering::SeqCst) {
                return;
            }

            thread::sleep(Duration::from_millis(1));
        }))
    }

    fn read_until_async(&self, delimiter: u8) -> AsyncReader {
        AsyncReader::new(Box::new(move |event_tx, cancellation_token| loop {
            for event in read_input_events().unwrap().1 {
                if let InputEvent::Keyboard(KeyEvent::Char(key)) = event {
                    if (key as u8) == delimiter {
                        return;
                    }
                }

                if cancellation_token.load(Ordering::SeqCst) {
                    return;
                } else {
                    if event_tx.send(event).is_err() {
                        return;
                    }
                }

                thread::sleep(Duration::from_millis(1));
            }
        }))
    }

    fn read_sync(&self) -> SyncReader {
        SyncReader
    }

    fn enable_mouse_mode(&self) -> Result<()> {
        let mode = ConsoleMode::from(Handle::current_in_handle()?);

        unsafe {
            ORIG_MODE = mode.mode()?;
            mode.set_mode(ENABLE_MOUSE_MODE)?;
        }
        Ok(())
    }

    fn disable_mouse_mode(&self) -> Result<()> {
        let mode = ConsoleMode::from(Handle::current_in_handle()?);
        mode.set_mode(unsafe { ORIG_MODE })?;
        Ok(())
    }
}

/// This type allows you to read input synchronously, which means that reading call will be blocking ones.
///
/// This type is an iterator, and could be used to iterate over input events.
///
/// If you don't want to block your calls use [AsyncReader](./LINK), which will read input on the background and queue it for you to read.
pub struct SyncReader;

impl Iterator for SyncReader {
    type Item = InputEvent;

    /// Read input from the user.
    ///
    /// If there are no keys pressed this will be a blocking call until there are.
    /// This will return `None` in case of a failure and `Some(InputEvent) in case of an occurred input event.`
    fn next(&mut self) -> Option<Self::Item> {
        read_single_event().unwrap()
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
    event_rx: Receiver<InputEvent>,
    shutdown: Arc<AtomicBool>,
}

impl AsyncReader {
    /// Construct a new instance of the `AsyncReader`.
    /// The reading will immediately start when calling this function.
    pub fn new(function: Box<dyn Fn(&Sender<InputEvent>, &Arc<AtomicBool>) + Send>) -> AsyncReader {
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

impl Drop for AsyncReader {
    fn drop(&mut self) {
        self.stop();
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
    /// - When calling this method to fast after each other the reader might not have read a full byte sequence of some pressed key.
    /// Make sure that you have some delay of a few ms when calling this method.
    fn next(&mut self) -> Option<Self::Item> {
        let mut iterator = self.event_rx.try_iter();
        iterator.next()
    }
}

extern "C" {
    fn _getwche() -> INT;
}

fn read_single_event() -> Result<Option<InputEvent>> {
    let console = Console::from(Handle::current_in_handle()?);

    let input = match console.read_single_input_event()? {
        Some(event) => event,
        None => return Ok(None),
    };

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
fn read_input_events() -> Result<(u32, Vec<InputEvent>)> {
    let console = Console::from(Handle::current_in_handle()?);

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
    if let Some(event) = parse_mouse_event_record(&mouse_event) {
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
                } else if key_state.has_state(SHIFT_PRESSED) {
                    // Shift + key press, essentially the same as single key press
                    // Separating to be explicit about the Shift press.
                    if character == '\t' {
                        Some(KeyEvent::BackTab)
                    } else {
                        Some(KeyEvent::Tab)
                    }
                } else {
                    Some(KeyEvent::Char(character))
                }
            } else {
                None
            }
        }
    }
}

fn parse_mouse_event_record(event: &MouseEvent) -> Option<super::MouseEvent> {
    // NOTE (@imdaveho): xterm emulation takes the digits of the coords and passes them
    // individually as bytes into a buffer; the below cxbs and cybs replicates that and
    // mimicks the behavior; additionally, in xterm, mouse move is only handled when a
    // mouse button is held down (ie. mouse drag)

    let xpos = event.mouse_position.x + 1;
    let ypos = event.mouse_position.y + 1;

    // TODO (@imdaveho): check if linux only provides coords for visible terminal window vs the total buffer

    match event.event_flags {
        EventFlags::PressOrRelease => {
            // Single click
            match event.button_state {
                ButtonState::Release => Some(super::MouseEvent::Release(xpos as u16, ypos as u16)),
                ButtonState::FromLeft1stButtonPressed => {
                    // left click
                    Some(super::MouseEvent::Press(
                        MouseButton::Left,
                        xpos as u16,
                        ypos as u16,
                    ))
                }
                ButtonState::RightmostButtonPressed => {
                    // right click
                    Some(super::MouseEvent::Press(
                        MouseButton::Right,
                        xpos as u16,
                        ypos as u16,
                    ))
                }
                ButtonState::FromLeft2ndButtonPressed => {
                    // middle click
                    Some(super::MouseEvent::Press(
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
                Some(super::MouseEvent::Hold(xpos as u16, ypos as u16))
            } else {
                None
            }
        }
        EventFlags::MouseWheeled => {
            // Vertical scroll
            // NOTE (@imdaveho) from https://docs.microsoft.com/en-us/windows/console/mouse-event-record-str
            // if `button_state` is negative then the wheel was rotated backward, toward the user.
            if event.button_state != ButtonState::Negative {
                Some(super::MouseEvent::Press(
                    MouseButton::WheelUp,
                    xpos as u16,
                    ypos as u16,
                ))
            } else {
                Some(super::MouseEvent::Press(
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
