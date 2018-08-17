//! This module provides a way to work with an handle to an screen on different platforms.

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

use super::functions;

/// This trait defines represents an stdout of an screen.
/// This trait can be implemented so that an concrete implementation of the IStdout can forfill
/// the wishes to work on an specific platform.
///
/// ## For example:
///
/// This trait is implemented for `WINAPI` (Windows specific) and `ANSI` (Unix specific),
/// so that color related actions can be preformed on both unix and windows systems.
pub trait IStdout: Send+Sync {
    /// Write an &str to the current stdout and flush the screen.
    fn write_str(&self, string: &str) -> io::Result<usize>;
    /// Write [u8] buffer to console.
    fn write(&self, buf: &[u8]) -> io::Result<usize>;
    /// Flush the current output.
    fn flush(&self) -> io::Result<()>;

    fn as_any(&self) -> &Any;
    fn as_any_mut(&mut self) -> &mut Any;
}
