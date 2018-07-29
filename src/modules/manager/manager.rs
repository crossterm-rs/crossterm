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

#[cfg(target_os = "windows")]
use winapi::um::winnt::HANDLE;

/// Struct that stores an specific platform implementation for screen related actions.
pub struct ScreenManager {
    screen_manager: Box<IScreenManager>,
}

impl ScreenManager {
    /// Create new screen manager instance whereon screen related actions can be performed.
    pub fn new() -> ScreenManager {
        #[cfg(target_os = "windows")]
        let screen_manager = functions::get_module::<Box<IScreenManager>>(
            Box::from(WinApiScreenManager::new()),
            Box::from(AnsiScreenManager::new()),
        ).unwrap();

        #[cfg(not(target_os = "windows"))]
        let screen_manager = Box::from(AnsiScreenManager::new()) as Box<IScreenManager>;

        ScreenManager {
            screen_manager,
        }
    }

    /// Set whether screen is raw screen.
    pub fn set_is_raw_screen(&mut self, value: bool) {
        self.screen_manager.set_is_raw_screen(value);
    }

    /// Set whether the current screen is alternate screen.
    pub fn set_is_alternate_screen(&mut self, value: bool) {
        self.screen_manager.set_is_alternate_screen(value);
    }

    /// Check if the current screen is in rawscreen modes
    pub fn is_raw_screen(&self) -> bool {
        self.screen_manager.is_raw_screen()
    }

    /// Check if the current screen is in alternate modes.
    pub fn is_alternate_screen(&self) -> bool {
        self.screen_manager.is_alternate_screen()
    }

    /// Write String to the current screen.
    pub fn write_string(&self, string: String) -> io::Result<usize> {
        self.screen_manager.write_str(string.as_str())
    }

    /// Flush the current screen.
    pub fn flush(&self) -> io::Result<()>
    {
        self.screen_manager.flush()
    }

    /// Write &str to the current screen.
    pub fn write_str(&self, string: &str) -> io::Result<usize> {
        self.screen_manager.write_str(string)
    }

    /// Can be used to get an reference to an specific implementation used for the current platform.
    pub fn as_any(&self) -> &Any {
        self.screen_manager.as_any()
    }

    /// Can be used to get an mutable reference to an specific implementation used for the current platform.
    pub fn as_any_mut(&mut self) -> &mut Any {
        self.screen_manager.as_any_mut()
    }
}
