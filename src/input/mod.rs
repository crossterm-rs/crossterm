use std::io;

pub mod input;

#[cfg(target_os = "windows")]
mod windows_input;
mod unix_input;

#[cfg(target_os = "windows")]
use self::windows_input::WindowsInput;
use self::unix_input::UnixInput;

pub use self::input::{ input, TerminalInput };

trait ITerminalInput
{
    fn read_char(&self) -> io::Result<String>;
    fn read_key(&self) -> io::Result<()>;

    fn read_async(&self);
    fn read_until(&self, delimiter: u8);
}