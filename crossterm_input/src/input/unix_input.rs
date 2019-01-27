//! This is a UNIX specific implementation for input related action.

use super::*;
use crate::sys::unix::{get_tty, read_char};

use crossterm_utils::TerminalOutput;
use std::char;
use std::thread;

pub struct UnixInput;

impl UnixInput {
    pub fn new() -> UnixInput {
        UnixInput {}
    }
}

impl ITerminalInput for UnixInput {
    fn read_char(&self, __stdout: &Option<&Arc<TerminalOutput>>) -> io::Result<char> {
        read_char()
    }

    fn read_async(&self, __stdout: &Option<&Arc<TerminalOutput>>) -> AsyncReader {
        let (send, recv) = mpsc::channel();

        thread::spawn(move || {
            for i in get_tty().unwrap().bytes() {
                if send.send(i).is_err() {
                    return;
                }
            }
        });

        AsyncReader { recv }
    }

    fn read_until_async(
        &self,
        delimiter: u8,
        __stdout: &Option<&Arc<TerminalOutput>>,
    ) -> AsyncReader {
        let (send, recv) = mpsc::channel();

        thread::spawn(move || {
            for i in get_tty().unwrap().bytes() {
                match i {
                    Ok(byte) => {
                        let end_of_stream = byte == delimiter;
                        let send_error = send.send(Ok(byte)).is_err();

                        if end_of_stream || send_error {
                            return;
                        }
                    }
                    Err(_) => {
                        return;
                    }
                }
            }
        });

        AsyncReader { recv }
    }
}
