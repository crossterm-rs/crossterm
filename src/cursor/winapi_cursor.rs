//! This is an WINAPI specific implementation for cursor related action.
//! This module is used for windows terminals that do not support ANSI escape codes.
//! Note that the cursor position is 0 based. This means that we start counting at 0 when setting the cursor position ect.
use super::super::manager::{IScreenManager, ScreenManager, WinApiScreenManager};
use super::ITerminalCursor;

use kernel::windows_kernel::{cursor, kernel};

use std::rc::Rc;
use std::sync::Mutex;

/// This struct is an windows implementation for cursor related actions.
pub struct WinApiCursor;

impl WinApiCursor {
    pub fn new() -> Box<WinApiCursor> {
        Box::from(WinApiCursor { })
    }
}

impl ITerminalCursor for WinApiCursor {
    fn goto(&self, x: u16, y: u16, screen_manager: &ScreenManager) {
        cursor::set_console_cursor_position(x as i16, y as i16, &self.screen_manager);
    }

    fn pos(&self, screen_manager: &ScreenManager) -> (u16, u16) {
        cursor::pos(&self.screen_manager)
    }

    fn move_up(&self, count: u16, screen_manager: &ScreenManager) {
        let (xpos, ypos) = self.pos();
        self.goto(xpos, ypos - count);
    }

    fn move_right(&self, count: u16, screen_manager: &ScreenManager) {
        let (xpos, ypos) = self.pos();
        self.goto(xpos + count, ypos);
    }

    fn move_down(&self, count: u16, screen_manager: &ScreenManager) {
        let (xpos, ypos) = self.pos();
        self.goto(xpos, ypos + count);
    }

    fn move_left(&self, count: u16, screen_manager: &ScreenManager) {
        let (xpos, ypos) = self.pos();
        self.goto(xpos - count, ypos);
    }

    fn save_position(&self, screen_manager: &ScreenManager) {
        cursor::save_cursor_pos(&self.screen_manager);
    }

    fn reset_position(&self, screen_manager: &ScreenManager) {
        cursor::reset_to_saved_position(&self.screen_manager);
    }

    fn hide(&self, screen_manager: &ScreenManager) {
        cursor::cursor_visibility(false, &self.screen_manager);
    }

    fn show(&self, screen_manager: &ScreenManager) {
        cursor::cursor_visibility(true, &self.screen_manager);
    }

    fn blink(&self, blink: bool, screen_manager: &ScreenManager) {}
}
