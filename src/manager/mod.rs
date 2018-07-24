//! This module provides one place to work with the screen.
//!
//!   In Rust we can call `stdout()` to get an handle to the current default console handle.
//!   For example when in unix systems you want to print something to the main screen you can use the following code:
//!
//!   ```
//!   write!(std::io::stdout(), "{}", "some text").
//!   ```
//!
//!   But things change when we are in alternate screen modes.
//!   We can not simply use `stdout()` to get a handle to the alternate screen, since this call returns the current default console handle (mainscreen).
//!
//!   Instead we need to store an handle to the screen output.
//!   This handle could be used to put into alternate screen modes and back into main screen modes.
//!   Through this stored handle Crossterm can execute its command on the current screen whether it be alternate screen or main screen.
//!
//!   For unix systems we store the handle gotten from `stdout()` for windows systems that are not supporting ANSI escape codes we store WinApi `HANDLE` struct witch will provide access to the current screen.
//!
//! This is the reason why this module exits. It is to provide access to the current terminal screen whether it will be the alternate screen and main screen.

pub mod manager;

mod ansi_manager;
#[cfg(target_os = "windows")]
mod win_manager;

pub use self::ansi_manager::AnsiScreenManager;
#[cfg(target_os = "windows")]
pub use self::win_manager::WinApiScreenManager;

pub use self::manager::ScreenManager;
use std::any::Any;
use std::io;

pub trait IScreenManager {
    fn set_is_raw_screen(&mut self, value: bool);
    fn set_is_alternate_screen(&mut self, value: bool);

    fn is_raw_screen(&self) -> bool;
    fn is_alternate_screen(&self) -> bool;

    /// Write ansi code as String to the current stdout.
    fn write_string(&mut self, string: String) -> io::Result<usize>;
    /// Write a &str to the current stdout.
    fn write_str(&mut self, string: &str) -> io::Result<usize>;
    /// Write [u8] buffer to console.
    fn write(&mut self, buf: &[u8]) -> io::Result<usize>;
    /// Flush the current output.
    fn flush(&mut self) -> io::Result<()>;
    /// Can be used to convert to an specific IScreenManager implementation.
    fn as_any(&mut self) -> &mut Any;
}
