//! This module provides an interface for working with the sceen. With that I mean that you can get or wirte to the handle of the current screen. stdout.
//! Because crossterm can work with alternate screen, we need a place that holds the handle to the current screen. And this module provides this place.
use super::{AnsiScreenManager, WinApiScreenManager, IScreenManager };
use super::super::shared::functions;
use std::any::Any;

use std::fmt::Display;
use std::io::{ self, Write };

#[cfg(target_os = "windows")]
use winapi::um::winnt::HANDLE;

/// Struct that stores an specific platform implementation for screen related actions.
pub struct ScreenManager
{
    screen_manager: Box<IScreenManager>
}

impl ScreenManager
{
    /// Create new screen manager instance whereon screen related actions can be performed.
    pub fn new() -> ScreenManager {
//        #[cfg(target_os = "windows")]
//        let screen_manager = functions::get_module::<Box<IScreenManager>>(Box::from(WinApiScreenManager::new()), Box::from(AnsiScreenManager::new())).unwrap();
//
//        #[cfg(not(target_os = "windows"))]
//        let screen_manager = Box::new(AnsiScreenManager::new());

        ScreenManager
        {
            screen_manager: Box::from(WinApiScreenManager::new()),
        }
    }

    pub fn toggle_is_alternate_screen(&mut self, is_alternate_screen: bool)
    {
        self.screen_manager.toggle_is_alternate_screen(is_alternate_screen);
    }

    /// Write an ANSI code as String.
    pub fn write_ansi(&mut self, string: String)
    {
        self.screen_manager.write_ansi(string);
    }

    /// Write an ANSI code as &str
    pub fn write_ansi_str(&mut self, string: &str)
    {
        self.screen_manager.write_ansi_str(string);
    }

    pub fn as_any(&mut self) -> &mut Any { self.screen_manager.as_any() }
}

impl Write for ScreenManager
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.screen_manager.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.screen_manager.flush()
    }
}
