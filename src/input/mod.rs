use std::io;

pub mod input;

#[cfg(target_os = "windows")]
use self::windows_input::WindowsInput;
#[cfg(target_os = "windows")]
mod windows_input;

#[cfg(not(target_os = "windows"))]
use self::unix_input::UnixInput;
#[cfg(not(target_os = "windows"))]
mod unix_input;


pub use self::input::{ input, TerminalInput };

use std::io::Read;
use std::sync::mpsc;

trait ITerminalInput
{
    fn read_line(&self) -> io::Result<String>;

    fn read_char(&self) -> io::Result<char>;
    fn read_pressed_key(&self) -> io::Result<Key>;

    fn read_async(&self) -> AsyncReader;
    fn read_until_async(&self, delimiter: u8) -> AsyncReader;
}

pub struct AsyncReader
{
    recv: mpsc::Receiver<io::Result<u8>>
}

impl Read for AsyncReader {
    /// Read from the byte stream.
    ///
    /// This will never block, but try to drain the event queue until empty. If the total number of
    /// bytes written is lower than the buffer's length, the event queue is empty or that the event
    /// stream halted.
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut total = 0;

        loop {
            if total >= buf.len() {
                break;
            }

            match self.recv.try_recv() {
                Ok(Ok(b)) => {
                    buf[total] = b;
                    total += 1;
                }
                Ok(Err(e)) => return Err(e),
                Err(_) => break,
            }
        }

        Ok(total)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Key {
    Unknown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,
    Enter,
    Escape,
    Char(char),
    #[doc(hidden)]
    __More,
}