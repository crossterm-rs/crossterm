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

mod stdout;

mod ansi_stdout;
#[cfg(target_os = "windows")]
mod winapi_stdout;

pub use self::ansi_stdout::AnsiStdout;
#[cfg(target_os = "windows")]
pub use self::winapi_stdout::WinApiStdout;

pub use self::stdout::Stdout;

use std::any::Any;
use std::io;

use super::{functions};

/// This trait defines the actions that could be preformed on the current screen.
/// This trait can be implemented so that an concrete implementation of the IScreenManager can forfill
/// the wishes to work on an specific platform.
///
/// ## For example:
///
/// This trait is implemented for `WINAPI` (Windows specific) and `ANSI` (Unix specific),
/// so that color related actions can be preformed on both unix and windows systems.
pub trait IStdout {
    /// Write a &str to the current stdout.
    fn write_str(&self, string: &str ) -> io::Result<usize>;
    /// Write [u8] buffer to console.
    fn write(&self, buf: &[u8]) -> io::Result<usize>;
    /// Flush the current output.
    fn flush(&self) -> io::Result<()>;

    fn as_any(&self) -> &Any;

    fn as_any_mut(&mut self) -> &mut Any;
}
