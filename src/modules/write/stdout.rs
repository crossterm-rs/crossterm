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

use super::*;

use std::any::Any;
use std::fmt::Display;
use std::io::{self, Write};
use std::default::Default;

#[cfg(target_os = "windows")]
use winapi::um::winnt::HANDLE;

use std::sync::Arc;

/// Struct that is an handle to an terminal screen.
/// This handle could be used to write to the current screen
///
/// For unix and windows 10 `stdout()` will be used for handle when on windows systems with versions lower than 10 WinApi `HANDLE` will be used.
pub struct Stdout {
    screen_manager: Box<IStdout + Send>,
    pub is_in_raw_mode:bool,
}

impl Stdout {
    /// Create new screen write instance whereon screen related actions can be performed.
    pub fn new(is_in_raw_mode: bool) -> Self {
        #[cfg(target_os = "windows")]
        let screen_manager = functions::get_module::<Box<IStdout + Send>>(
            Box::from(WinApiStdout::new()),
            Box::from(AnsiStdout::new()),
        ).unwrap();

        #[cfg(not(target_os = "windows"))]
        let screen_manager = Box::from(AnsiStdout::new()) as Box<IStdout + Send>;

        Stdout { screen_manager , is_in_raw_mode}
    }

    /// Write String to the current screen.
    pub fn write_string(&self, string: String) -> io::Result<usize> {
        self.screen_manager.write_str(string.as_str())
    }

    /// Flush the current screen.
    pub fn flush(&self) -> io::Result<()> {
        self.screen_manager.flush()
    }

    /// Write &str to the current screen.
    pub fn write_str(&self, string: &str) -> io::Result<usize> {
        self.screen_manager.write_str(string)
    }

    /// Write buffer to the screen
    pub fn write_buf(&self, buf: &[u8]) -> io::Result<usize> {
        self.screen_manager.write(buf)
    }

    pub fn as_any(&self) -> &Any {
        self.screen_manager.as_any()
    }
    pub fn as_any_mut(&mut self) -> &mut Any {
        self.screen_manager.as_any_mut()
    }
}

impl Default for Stdout
{
    /// Get the default handle to the current screen.
    fn default() -> Self {
        #[cfg(target_os = "windows")]
        let screen_manager = functions::get_module::<Box<IStdout + Send>>(
            Box::from(WinApiStdout::new()),
            Box::from(AnsiStdout::new()),
        ).unwrap();

        #[cfg(not(target_os = "windows"))]
        let screen_manager = Box::from(AnsiStdout::new()) as Box<IStdout + Send>;

        Stdout { screen_manager , is_in_raw_mode: false}
    }
}
