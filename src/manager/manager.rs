use std::io::Write;

use super::IScreenManager;
use super::ansi_manager::AnsiScreenManager;

pub struct ScreenManager
{
    screen_manager: Box<IScreenManager<Box<Write>>>
}

impl ScreenManager
{
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

    pub fn stdout(&mut self) -> &mut Box<Write>
    {
        self.screen_manager.stdout()
    }

    pub fn toggle_is_alternate_screen(&mut self,is_alternate_screen: bool)
    {
        self.screen_manager.toggle_is_alternate_screen(is_alternate_screen);
    }

    pub fn write_ansi(&mut self, string: String)
    {
        self.screen_manager.write_ansi(string);
    }

    pub fn write_ansi_str(&mut self, string: &str)
    {
        self.screen_manager.write_ansi_str(string);
    }
}