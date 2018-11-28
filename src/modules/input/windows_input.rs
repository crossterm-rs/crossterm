//! This is a WINDOWS specific implementation for input related action.

use super::*;

use winapi::um::winnt::INT;

use std::char;
use std::thread;

pub struct WindowsInput;

impl WindowsInput {
    pub fn new() -> WindowsInput {
        WindowsInput {}
    }
}

impl ITerminalInput for WindowsInput {
    fn read_line(&self, stdout: &Option<&Arc<TerminalOutput>>) -> io::Result<String> {
        let mut chars: Vec<char> = Vec::new();

        loop {
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

            // if 0 or 0xe0 we need to listen again because the next key will be an special key
            if pressed_char != 0 || pressed_char != 0xe0 {
                match char::from_u32(pressed_char as u32) {
                    Some(c) => {
                        if is_line_end(c) {
                            break;
                        } else {
                            chars.push(c);
                        }
                    }
                    None => panic!("Some error needs to be returned"),
                };
            }
        }

        return Ok(chars.into_iter().collect());
    }

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

    fn read_async(&self, stdout: &Option<&Arc<TerminalOutput>>) -> AsyncReader {
        let (tx, rx) = mpsc::channel();

        let is_raw_screen = match stdout {
            Some(output) => output.is_in_raw_mode,
            None => false,
        };

        thread::spawn(move || {
            loop {
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
                    return;
                }

                tx.send(Ok(pressed_char as u8));
            }
        });

        AsyncReader { recv: rx }
    }

    fn read_until_async(
        &self,
        delimiter: u8,
        stdout: &Option<&Arc<TerminalOutput>>,
    ) -> AsyncReader {
        let (tx, rx) = mpsc::channel();

        let is_raw_screen = match stdout {
            Some(output) => output.is_in_raw_mode,
            None => false,
        };

        thread::spawn(move || {
            loop {
                // _getwch is without echo and _getwche is with echo
                let pressed_char = unsafe {
                    if is_raw_screen {
                        _getwch()
                    } else {
                        _getwche()
                    }
                } as u8;

                let end_of_stream = pressed_char == delimiter;

                // we could return error but maybe option to keep listening until valid character is inputted.
                if pressed_char == 0 || pressed_char == 0xe0 || end_of_stream {
                    return;
                }

                tx.send(Ok(pressed_char as u8));
            }
        });

        AsyncReader { recv: rx }
    }
}

fn is_line_end(key: char) -> bool {
    if key as u8 == 13 {
        return true;
    }

    return false;
}

extern "C" {
    fn _getwche() -> INT;
    fn _getwch() -> INT;
}
