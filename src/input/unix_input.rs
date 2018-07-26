use std::char;
use std::io;
use std::io::Read;
use std::io::Write;
use std::sync::mpsc;
use std::thread;

use super::super::kernel::unix_kernel::terminal::{get_tty, read_char};
use super::super::terminal::terminal;
use super::{AsyncReader, ITerminalInput, Key};

pub struct UnixInput;

impl UnixInput {
    pub fn new() -> UnixInput {
        UnixInput {}
    }
}

impl ITerminalInput for UnixInput {
    fn read_line(&self) -> io::Result<String> {
        let mut rv = String::new();
        io::stdin().read_line(&mut rv)?;
        let len = rv.trim_right_matches(&['\r', '\n'][..]).len();
        rv.truncate(len);
        Ok(rv)
    }

    fn read_char(&self) -> io::Result<char> {
        read_char()
    }

    fn read_pressed_key(&self) -> io::Result<Key> {
        Ok(Key::Unknown)
    }

    fn read_async(&self) -> AsyncReader {
        let (send, recv) = mpsc::channel();

        thread::spawn(move || for i in get_tty().unwrap().bytes() {
            if send.send(i).is_err() {
                return;
            }
        });

        AsyncReader { recv: recv }
    }

    fn read_until_async(&self, delimiter: u8) -> AsyncReader {
        let (send, recv) = mpsc::channel();

        thread::spawn(move || for i in get_tty().unwrap().bytes() {

            match i {
                Ok(byte) => {
                    let end_of_stream = &byte == &delimiter;
                    let send_error = send.send(Ok(byte)).is_err();

                    if end_of_stream || send_error { return; }
                },
                Err(_) => { return; }
            }
        });

        AsyncReader { recv: recv }
    }
}
