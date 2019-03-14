//! This is a WINDOWS specific implementation for input related action.

use super::*;

use crossterm_utils::TerminalOutput;
use crossterm_winapi::{
    ButtonState, Console, ConsoleMode, EventFlags, Handle, InputEventType, KeyEventRecord,
    MouseEvent,
};
use std::thread;
use std::{char, io};
use winapi::um::winnt::INT;

use std::io::{Error, ErrorKind};
use std::mem::zeroed;
use std::sync::atomic::Ordering;
use std::sync::mpsc::Receiver;
use winapi::shared::minwindef::DWORD;
use winapi::um::{
    consoleapi::{GetConsoleMode, ReadConsoleInputW, SetConsoleMode},
    wincon::{
        FOCUS_EVENT, INPUT_RECORD, KEY_EVENT, KEY_EVENT_RECORD, MENU_EVENT, MOUSE_EVENT,
        MOUSE_EVENT_RECORD, WINDOW_BUFFER_SIZE_EVENT,
    },
};

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
    fn read_char(&self, stdout: &Option<&Arc<TerminalOutput>>) -> io::Result<char> {
        let is_raw_screen = match stdout {
            Some(output) => output.is_in_raw_mode,
            None => false,
        };

        // _getwch is without echo and _getwche is with echo
        let pressed_char = unsafe {
            if is_raw_screen {
                _getwch()
            } else {
                _getwche()
            }
        };

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

    fn read_async(&self, _stdout: &Option<&Arc<TerminalOutput>>) -> AsyncReader {
        AsyncReader::new(Box::new(move |event_tx| {
            for i in into_virtual_terminal_sequence().unwrap() {
                if event_tx.send(i).is_err() {
                    return;
                }
            }
        }))
    }

    fn read_until_async(
        &self,
        delimiter: u8,
        _stdout: &Option<&Arc<TerminalOutput>>,
    ) -> AsyncReader {
        AsyncReader::new(Box::new(move |event_tx| {
            for i in into_virtual_terminal_sequence().unwrap() {
                if i == delimiter {
                    return;
                } else {
                    if event_tx.send(i).is_err() {
                        return;
                    }
                }
            }
        }))
    }

    fn enable_mouse_mode(&self, __stdout: &Option<&Arc<TerminalOutput>>) -> io::Result<()> {
        let mode = ConsoleMode::from(Handle::current_in_handle()?);

        unsafe {
            ORIG_MODE = mode.mode()?;
            mode.set_mode(ENABLE_MOUSE_MODE)?;
        }
        Ok(())
    }

    fn disable_mouse_mode(&self, __stdout: &Option<&Arc<TerminalOutput>>) -> io::Result<()> {
        let mode = ConsoleMode::from(Handle::current_in_handle()?);
        mode.set_mode(unsafe { ORIG_MODE })
    }
}

extern "C" {
    fn _getwche() -> INT;
    fn _getwch() -> INT;
}

/// https://github.com/retep998/wio-rs/blob/master/src/console.rs#L130
fn into_virtual_terminal_sequence() -> Result<Vec<u8>> {
    let console = Console::from(Handle::current_in_handle()?);

    let mut vts: Vec<u8> = Vec::new();

    for input in console.read_console_input()? {
        unsafe {
            match input.event_type {
                InputEventType::KeyEvent => {
                    let key_event = KeyEventRecord::from(*input.event.KeyEvent());
                    if key_event.key_down {
                        // NOTE (@imdaveho): only handle key down
                        // this is because unix limits key events to key press
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

    return Ok(vts);
}

fn handle_key_event(key_event: &KeyEventRecord, seq: &mut Vec<u8>) {
    match key_event.virtual_key_code {
        0x10 | 0x11 | 0x12 => {
            // ignore SHIFT, CTRL, ALT standalone presses
            seq.push(b'\x00');
        }
        0x08 => {
            // BACKSPACE
            seq.push(b'\x7F');
        }
        0x1B => {
            // ESC
            seq.push(b'\x1B');
        }
        0x0D => {
            // ENTER
            seq.push(b'\n');
        }
        0x70 | 0x71 | 0x72 | 0x73 => {
            // F1 - F4 are support by default VT100
            seq.push(b'\x1B');
            seq.push(b'O');
            seq.push([b'P', b'Q', b'R', b'S'][(key_event.virtual_key_code - 0x70) as usize]);
        }
        0x74 | 0x75 | 0x76 | 0x77 => {
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
        0x78 | 0x79 | 0x7A | 0x7B => {
            // F9 - F12
            seq.push(b'\x1B');
            seq.push(b'[');
            seq.push(b'2');
            seq.push([b'0', b'1', b'3', b'4'][(key_event.virtual_key_code - 0x78) as usize]);
            seq.push(b'~');
        }
        0x25 | 0x26 | 0x27 | 0x28 => {
            // LEFT, UP, RIGHT, DOWN
            seq.push(b'\x1B');
            seq.push(b'[');
            seq.push([b'D', b'A', b'C', b'B'][(key_event.virtual_key_code - 0x25) as usize]);
        }
        0x21 | 0x22 => {
            // PAGEUP, PAGEDOWN
            seq.push(b'\x1B');
            seq.push(b'[');
            seq.push([b'5', b'6'][(key_event.virtual_key_code - 0x21) as usize]);
            seq.push(b'~');
        }
        0x23 | 0x24 => {
            // END, HOME
            seq.push(b'\x1B');
            seq.push(b'[');
            seq.push([b'F', b'H'][(key_event.virtual_key_code - 0x23) as usize]);
        }
        0x2D | 0x2E => {
            // INSERT, DELETE
            seq.push(b'\x1B');
            seq.push(b'[');
            seq.push([b'2', b'3'][(key_event.virtual_key_code - 0x2D) as usize]);
            seq.push(b'~');
        }
        _ => {
            // Modifier Keys (Ctrl, Alt, Shift) Support
            // NOTE (@imdaveho): test to check if characters outside of
            // alphabet or alphanumerics are supported
            let chars: [u8; 2] =
                { (unsafe { *key_event.u_char.UnicodeChar() } as u16).to_ne_bytes() };
            match key_event.control_key_state {
                0x0002 | 0x0101 | 0x0001 => {
                    // Alt + chr support
                    seq.push(b'\x1B');
                    for ch in chars.iter() {
                        seq.push(*ch);
                    }
                }
                0x0008 | 0x0104 | 0x0004 => {
                    // Ctrl + key support (only Ctrl + {a-z})
                    // NOTE (@imdaveho): Ctrl + Shift + key support has same output
                    let alphabet: Vec<u8> = (b'\x01'..b'\x1B').collect();
                    for ch in chars.iter() {
                        // Constrain to only Aa-Zz keys
                        if alphabet.contains(&ch) {
                            seq.push(*ch);
                        } else {
                            seq.push(b'\x00');
                        }
                    }
                }
                0x000A | 0x0105 | 0x0005 => {
                    // TODO: Alt + Ctrl + Key support
                    // mainly updating the Alt section of parse_event()
                    // and updating parse_utf8_char()
                    seq.push(b'\x00');
                }
                0x001A | 0x0115 | 0x0015 => {
                    // TODO: Alt + Ctrl + Shift Key support
                    // mainly updating the Alt section of parse_event()
                    // and updating parse_utf8_char()
                    seq.push(b'\x00');
                }
                0x0000 => {
                    // Single key press
                    for ch in chars.iter() {
                        seq.push(*ch);
                    }
                }
                0x0010 => {
                    // Shift + key press
                    // Essentially the same as single key press
                    // separating to be explicit about the Shift press
                    // for Event enum
                    for ch in chars.iter() {
                        seq.push(*ch);
                    }
                }
                _ => {
                    seq.push(b'\x00');
                }
            }
        }
    };
}

fn handle_mouse_event(event: &MouseEvent, seq: &mut Vec<u8>) {
    // NOTE (@imdaveho): xterm emulation takes the digits of the coords and passes them
    // individually as bytes into a buffer; the below cxbs and cybs replicates that and
    // mimicks the behavior; additionally, in xterm, mouse move is only handled when a
    // mouse button is held down (ie. mouse drag)

    let cxbs: Vec<u8> = event
        .mouse_position
        .x
        .to_string()
        .chars()
        .map(|d| d as u8)
        .collect();
    let cybs: Vec<u8> = event
        .mouse_position
        .x
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
            // MouseWheeled events are positive if wheeled up and negative when wheeled down...
            // from testing it looks like getting the "high word" or (button >> 16) as a signed int
            if ((event.button_state as u32 >> 16) as i16) >= 0 {
                // WheelUp
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
                // WheelDown
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
