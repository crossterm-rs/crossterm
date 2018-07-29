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
//! This is the reason why this module exits: it is to provide access to the current terminal screen whether it will be the alternate screen and main screen.

mod manager;

#[cfg(target_os = "windows")]
mod win_manager;
mod ansi_manager;

#[cfg(target_os = "windows")]
pub use self::win_manager::WinApiScreenManager;
pub use self::ansi_manager::AnsiScreenManager;

pub use self::manager::ScreenManager;
use super::functions;

use std::any::Any;
use std::io;

/// This trait defines the actions that could be preformed on the current screen.
/// This trait can be implemented so that an concrete implementation of the IScreenManager can forfill
/// the wishes to work on an specific platform.
///
/// ## For example:
///
/// This trait is implemented for `WINAPI` (Windows specific) and `ANSI` (Unix specific),
/// so that color related actions can be preformed on both unix and windows systems.
pub trait IScreenManager {
    fn set_is_raw_screen(&mut self, value: bool);
    fn set_is_alternate_screen(&mut self, value: bool);

    fn is_raw_screen(&self) -> bool;
    fn is_alternate_screen(&self) -> bool;

    /// Write a &str to the current stdout.
    fn write_str(&self, string: &str) -> io::Result<usize>;
    /// Write [u8] buffer to console.
    fn write(&self, buf: &[u8]) -> io::Result<usize>;
    /// Flush the current output.
    fn flush(&self) -> io::Result<()>;
    /// Can be used to convert to an specific IScreenManager implementation.
    fn as_any(&self) -> &Any;
    /// Can be used to convert to an specific mutable IScreenManager implementation.
    fn as_any_mut(&mut self) -> &mut Any;
}
