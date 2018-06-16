//! This module provides an interface for working with the sceen. With that I mean that you can get or wirte to the handle of the current screen. stdout.
//! Because crossterm can work with alternate screen, we need a place that holds the handle to the current screen. And this module provides this place.

use super::IScreenManager;
use super::ansi_manager::AnsiScreenManager;

use std::io::Write;

/// Struct that stores an specific platform implementation for screen related actions.
pub struct ScreenManager
{
    screen_manager: Box<IScreenManager<Box<Write>>>
}

impl ScreenManager
{
    /// Create new screen manager instance whereon screen related actions can be performed.
    pub fn new() -> ScreenManager {
        //        #[cfg(target_os = "windows")]
        //        let cursor = functions::get_module::<Box<ITerminalCursor>>(WinApiCursor::new(), AnsiCursor::new());
        //
        //        #[cfg(not(target_os = "windows"))]

        ScreenManager
        {
            screen_manager: Box::new(AnsiScreenManager::new()),
        }
    }

    /// Get the stdout of the current screen
    pub fn stdout(&mut self) -> &mut Box<Write>
    {
        self.screen_manager.stdout()
    }

    pub fn toggle_is_alternate_screen(&mut self,is_alternate_screen: bool)
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
}