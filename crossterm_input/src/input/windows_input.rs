//! This is a WINDOWS specific implementation for input related action.

use super::*;

use crossterm_winapi::{
    ButtonState, Console, ConsoleMode, EventFlags, Handle, InputEventType, KeyEventRecord,
    MouseEvent,
};

use winapi::um::{
    wincon::{
        LEFT_ALT_PRESSED, LEFT_CTRL_PRESSED, RIGHT_ALT_PRESSED, RIGHT_CTRL_PRESSED, SHIFT_PRESSED,
    },
    winnt::INT,
    winuser::{
        VK_BACK, VK_CONTROL, VK_DELETE, VK_DOWN, VK_END, VK_ESCAPE, VK_F1, VK_F10, VK_F11, VK_F12,
        VK_F2, VK_F3, VK_F4, VK_F5, VK_F6, VK_F7, VK_F8, VK_F9, VK_HOME, VK_INSERT, VK_LEFT,
        VK_MENU, VK_NEXT, VK_PRIOR, VK_RETURN, VK_RIGHT, VK_SHIFT, VK_TAB, VK_UP,
    },
};

use std::sync::atomic::Ordering;
use std::time::Duration;
use std::{char, io, thread};

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
    fn read_char(&self) -> io::Result<char> {
        // _getwch is without echo and _getwche is with echo
        let pressed_char = unsafe { _getwche() };

        // we could return error but maybe option to keep listening until valid character is inputted.
        if pressed_char == 0 || pressed_char == 0xe0 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Given input char is not a valid char, mostly occurs when pressing special keys",
            ));
        }

        match char::from_u32(pressed_char as u32) {
            Some(c) => {
                return Ok(c);
            }
            None => Err(io::Error::new(
                io::ErrorKind::Other,
                "Could not parse given input to char",
            )),
        }
    }

    fn read_async(&self) -> AsyncReader {
        AsyncReader::new(Box::new(move |event_tx, cancellation_token| loop {
            for i in into_virtual_terminal_sequence().unwrap().1 {
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
            for i in into_virtual_terminal_sequence().unwrap().1 {
                if i == delimiter || cancellation_token.load(Ordering::SeqCst) {
                    return;
                } else {
                    if event_tx.send(i).is_err() {
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

/// This type allows you to read input synchronously, which means that reading calls will block.
///
/// This type is an iterator, and can be used to iterate over input events.
///
/// If you don't want to block your calls use [AsyncReader](./LINK), which will read input on the background and queue it for you to read.
pub struct SyncReader;

impl Iterator for SyncReader {
    type Item = InputEvent;

    /// Read input from the user.
    ///
    /// If there are no keys pressed, this will be a blocking call until there is one.
    /// This will return `None` in case of a failure and `Some(InputEvent)` in case of an occurred input event.
    fn next(&mut self) -> Option<Self::Item> {
        let mut iterator = into_virtual_terminal_sequence().unwrap().1.into_iter();

        match iterator.next() {
            None => None,
            Some(byte) => {
                if let Ok(event) = parse_event(byte, &mut iterator) {
                    Some(event)
                } else {
                    None
                }
            }
        }
    }
}

extern "C" {
    fn _getwche() -> INT;
    fn _getwch() -> INT;
}

/// partially inspired by: https://github.com/retep998/wio-rs/blob/master/src/console.rs#L130
fn into_virtual_terminal_sequence() -> Result<(u32, Vec<u8>)> {
    let console = Console::from(Handle::current_in_handle()?);

    let mut vts: Vec<u8> = Vec::new();

    let result = console.read_console_input()?;

    for input in result.1 {
        unsafe {
            match input.event_type {
                InputEventType::KeyEvent => {
                    let key_event = KeyEventRecord::from(*input.event.KeyEvent());
                    if key_event.key_down {
                        // NOTE (@imdaveho): only handle key down, this is because unix limits key events to key press
                        continue;
                    }
                    handle_key_event(&key_event, &mut vts);
                }
                InputEventType::MouseEvent => {
                    let mouse_event = MouseEvent::from(*input.event.MouseEvent());
                    // TODO: handle mouse events
                    handle_mouse_event(&mouse_event, &mut vts);
                }
                // NOTE (@imdaveho): ignore below
                InputEventType::WindowBufferSizeEvent => (),
                InputEventType::FocusEvent => (),
                InputEventType::MenuEvent => (),
            }
        }
    }

    return Ok((result.0, vts));
}

fn handle_key_event(key_event: &KeyEventRecord, seq: &mut Vec<u8>) {
    match key_event.virtual_key_code as i32 {
        VK_SHIFT | VK_CONTROL | VK_MENU => {
            // ignore SHIFT, CTRL, ALT standalone presses
        }
        VK_BACK => {
            seq.push(b'\x7F');
        }
        VK_ESCAPE => {
            seq.push(b'\x1B');
        }
        VK_RETURN => {
            seq.push(b'\n');
        }
        VK_F1 | VK_F2 | VK_F3 | VK_F4 => {
            // F1 - F4 are support by default VT100
            seq.push(b'\x1B');
            seq.push(b'O');
            seq.push([b'P', b'Q', b'R', b'S'][(key_event.virtual_key_code - 0x70) as usize]);
        }
        VK_F5 | VK_F6 | VK_F7 | VK_F8 => {
            // NOTE: F Key Escape Codes:
            // http://aperiodic.net/phil/archives/Geekery/term-function-keys.html
            // https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences
            // F5 - F8
            seq.push(b'\x1B');
            seq.push(b'[');
            seq.push(b'1');
            seq.push([b'5', b'7', b'8', b'9'][(key_event.virtual_key_code - 0x74) as usize]);
            seq.push(b'~');
        }
        VK_F9 | VK_F10 | VK_F11 | VK_F12 => {
            seq.push(b'\x1B');
            seq.push(b'[');
            seq.push(b'2');
            seq.push([b'0', b'1', b'3', b'4'][(key_event.virtual_key_code - 0x78) as usize]);
            seq.push(b'~');
        }
        VK_LEFT | VK_UP | VK_RIGHT | VK_DOWN => {
            seq.push(b'\x1B');
            seq.push(b'[');

            // Modifier Keys (Ctrl, Shift) Support
            let key_state = &key_event.control_key_state;
            if key_state.has_state(RIGHT_CTRL_PRESSED | LEFT_CTRL_PRESSED) {
                seq.push(53);
            } else if key_state.has_state(SHIFT_PRESSED) {
                seq.push(50);
            }

            seq.push([b'D', b'A', b'C', b'B'][(key_event.virtual_key_code - 0x25) as usize]);
        }
        VK_PRIOR | VK_NEXT => {
            seq.push(b'\x1B');
            seq.push(b'[');
            seq.push([b'5', b'6'][(key_event.virtual_key_code - 0x21) as usize]);
            seq.push(b'~');
        }
        VK_END | VK_HOME => {
            seq.push(b'\x1B');
            seq.push(b'[');
            seq.push([b'F', b'H'][(key_event.virtual_key_code - 0x23) as usize]);
        }
        VK_DELETE => {
            seq.push(b'\x1B');
            seq.push(b'[');
            seq.push([b'2', b'3'][(key_event.virtual_key_code - 0x2D) as usize]);
            seq.push(b'~');
        }
        VK_INSERT => {
            seq.push(b'\x1B');
            seq.push(b'[');
            seq.push(b'2');
            seq.push(b'~');
        }
        VK_TAB => {
            let key_state = &key_event.control_key_state;
            if key_state.has_state(SHIFT_PRESSED) {
                seq.push(b'\x1B');
                seq.push(b'[');
                seq.push(b'Z');
            } else {
                seq.push(b'\t');
            }
        }
        _ => {
            // Modifier Keys (Ctrl, Alt, Shift) Support
            // NOTE (@imdaveho): test to check if characters outside of
            // alphabet or alphanumerics are supported
            let character = { (unsafe { *key_event.u_char.UnicodeChar() } as u16) };

            if character < 255 {
                let character = character as u8 as char;

                let key_state = &key_event.control_key_state;

                if key_state.has_state(LEFT_ALT_PRESSED | RIGHT_ALT_PRESSED) {
                    seq.push(b'\x1B');
                    // If the ALT key is held down, pressing the A key produces ALT+A, which the system does not treat as a character at all, but rather as a system command.
                    // The pressed command is stored in `virtual_key_code`.
                    let command = key_event.virtual_key_code as u8 as char;

                    if (command).is_alphabetic() {
                        seq.push(command as u8);
                    }
                } else if key_state.has_state(LEFT_CTRL_PRESSED | RIGHT_CTRL_PRESSED) {
                    seq.push(character as u8);
                } else if key_state.has_state(SHIFT_PRESSED) {
                    // Shift + key press, essentially the same as single key press
                    // Separating to be explicit about the Shift press.
                    seq.push(character as u8);
                } else {
                    seq.push(character as u8);
                }
            }
        }
    }
}

fn handle_mouse_event(event: &MouseEvent, seq: &mut Vec<u8>) {
    // NOTE (@imdaveho): xterm emulation takes the digits of the coords and passes them
    // individually as bytes into a buffer; the below cxbs and cybs replicates that and
    // mimicks the behavior; additionally, in xterm, mouse move is only handled when a
    // mouse button is held down (ie. mouse drag)

    let cxbs: Vec<u8> =
        (event.mouse_position.x + 1) /* windows positions are 0 based and ansi codes 1. */
            .to_string()
            .chars()
            .map(|d| d as u8)
            .collect();
    let cybs: Vec<u8> =
        (event.mouse_position.y + 1) /* windows positions are 0 based and ansi codes 1. */
            .to_string()
            .chars()
            .map(|d| d as u8)
            .collect();

    // TODO (@imdaveho): check if linux only provides coords for visible terminal window vs the total buffer

    match event.event_flags {
        EventFlags::PressOrRelease => {
            // Single click
            match event.button_state {
                ButtonState::Release => {
                    seq.append(&mut vec![b'\x1B', b'[', b'<', b'3', b';']);
                    for x in cxbs {
                        seq.push(x);
                    }
                    seq.push(b';');
                    for y in cybs {
                        seq.push(y);
                    }
                    seq.push(b';');
                    seq.push(b'm');
                }
                ButtonState::FromLeft1stButtonPressed => {
                    // left click
                    seq.append(&mut vec![b'\x1B', b'[', b'<', b'0', b';']);
                    for x in cxbs {
                        seq.push(x);
                    }
                    seq.push(b';');
                    for y in cybs {
                        seq.push(y);
                    }
                    seq.push(b';');
                    seq.push(b'M');
                }
                ButtonState::RightmostButtonPressed => {
                    // right click
                    seq.append(&mut vec![b'\x1B', b'[', b'<', b'2', b';']);
                    for x in cxbs {
                        seq.push(x);
                    }
                    seq.push(b';');
                    for y in cybs {
                        seq.push(y);
                    }
                    seq.push(b';');
                    seq.push(b'M');
                }
                ButtonState::FromLeft2ndButtonPressed => {
                    // middle click
                    seq.append(&mut vec![b'\x1B', b'[', b'<', b'1', b';']);
                    for x in cxbs {
                        seq.push(x);
                    }
                    seq.push(b';');
                    for y in cybs {
                        seq.push(y);
                    }
                    seq.push(b';');
                    seq.push(b'M');
                }
                _ => (),
            }
        }
        EventFlags::MouseMoved => {
            // Click + Move
            // NOTE (@imdaveho) only register when mouse is not released
            if event.button_state != ButtonState::Release {
                seq.append(&mut vec![b'\x1B', b'[', b'<', b'3', b'2', b';']);
                for x in cxbs {
                    seq.push(x);
                }
                seq.push(b';');
                for y in cybs {
                    seq.push(y);
                }
                seq.push(b';');
                seq.push(b'M');
            } else {
                ()
            }
        }
        EventFlags::MouseWheeled => {
            // Vertical scroll
            // NOTE (@imdaveho) from https://docs.microsoft.com/en-us/windows/console/mouse-event-record-str
            // if `button_state` is negative then the wheel was rotated backward, toward the user.
            if event.button_state != ButtonState::Negative {
                seq.append(&mut vec![b'\x1B', b'[', b'<', b'6', b'4', b';']);
                for x in cxbs {
                    seq.push(x);
                }
                seq.push(b';');
                for y in cybs {
                    seq.push(y);
                }
                seq.push(b';');
                seq.push(b'M');
            } else {
                seq.append(&mut vec![b'\x1B', b'[', b'<', b'6', b'5', b';']);
                for x in cxbs {
                    seq.push(x);
                }
                seq.push(b';');
                for y in cybs {
                    seq.push(y);
                }
                seq.push(b';');
                seq.push(b'M');
            }
        }
        EventFlags::DoubleClick => (), // NOTE (@imdaveho): double click not supported by unix terminals
        EventFlags::MouseHwheeled => (), // NOTE (@imdaveho): horizontal scroll not supported by unix terminals
                                         // TODO: Handle Ctrl + Mouse, Alt + Mouse, etc.
    };
}
