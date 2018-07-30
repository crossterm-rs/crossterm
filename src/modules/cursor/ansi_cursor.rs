//! This is an ANSI specific implementation for cursor related action.
//! This module is used for windows 10 terminals and unix terminals by default.
//! Note that the cursor position is 0 based. This means that we start counting at 0 when setting the cursor position ect.

use super::{functions, ITerminalCursor, ScreenManager};

/// This struct is an ansi implementation for cursor related actions.
pub struct AnsiCursor;

impl AnsiCursor {
    pub fn new() -> Box<AnsiCursor> {
        Box::from(AnsiCursor {})
    }
}

impl ITerminalCursor for AnsiCursor {
    fn goto(&self, x: u16, y: u16, screen_manager: &ScreenManager) {
        screen_manager.write_string(format!(csi!("{};{}H"), y + 1, x + 1));
    }

    fn pos(&self, screen_manager: &ScreenManager) -> (u16, u16) {
        functions::get_cursor_position(screen_manager)
    }

    fn move_up(&self, count: u16, screen_manager: &ScreenManager) {
        screen_manager.write_string(format!(csi!("{}A"), count));
    }

    fn move_right(&self, count: u16, screen_manager: &ScreenManager) {
        screen_manager.write_string(format!(csi!("{}C"), count));
    }

    fn move_down(&self, count: u16, screen_manager: &ScreenManager) {
        screen_manager.write_string(format!(csi!("{}B"), count));
    }

    fn move_left(&self, count: u16, screen_manager: &ScreenManager) {
        screen_manager.write_string(format!(csi!("{}D"), count));
    }

    fn save_position(&self, screen_manager: &ScreenManager) {
        screen_manager.write_str(csi!("s"));
    }

    fn reset_position(&self, screen_manager: &ScreenManager) {
        screen_manager.write_str(csi!("u"));
    }

    fn hide(&self, screen_manager: &ScreenManager) {
        screen_manager.write_str(csi!("?25l"));
    }

    fn show(&self, screen_manager: &ScreenManager) {
        screen_manager.write_str(csi!("?25h"));
    }

    fn blink(&self, blink: bool, screen_manager: &ScreenManager) {
        if blink {
            screen_manager.write_str(csi!("?12h"));
        } else {
            screen_manager.write_str(csi!("?12l"));
        }
    }
}
