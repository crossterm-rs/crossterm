//! This is an WINAPI specific implementation for cursor related action.
//! This module is used for windows terminals that do not support ANSI escape codes.
//! Note that the cursor position is 0 based. This means that we start counting at 0 when setting the cursor position ect.
use super::super::manager::{IScreenManager, ScreenManager, WinApiScreenManager};
use super::ITerminalCursor;

use kernel::windows_kernel::{cursor, kernel};

use std::rc::Rc;
use std::sync::Mutex;

/// This struct is an windows implementation for cursor related actions.
pub struct WinApiCursor {
    screen_manager: Rc<Mutex<ScreenManager>>,
}

impl WinApiCursor {
    pub fn new(screen_manager: Rc<Mutex<ScreenManager>>) -> Box<WinApiCursor> {
        Box::from(WinApiCursor { screen_manager })
    }
}

impl ITerminalCursor for WinApiCursor {
    fn goto(&self, x: u16, y: u16) {
        cursor::set_console_cursor_position(x as i16, y as i16, &self.screen_manager);
    }

    fn pos(&self) -> (u16, u16) {
        cursor::pos(&self.screen_manager)
    }

    fn absolute_pos(&self) -> (u16, u16)
    {
        cursor::absolute_cursor_pos(&self.screen_manager)
    }

    fn move_up(&self, count: u16) {
        let (xpos, ypos) = self.pos();
        self.goto(xpos, ypos - count);
    }

    fn move_right(&self, count: u16) {
        let (xpos, ypos) = self.pos();
        self.goto(xpos + count, ypos);
    }

    fn move_down(&self, count: u16) {
        let (xpos, ypos) = self.pos();
        self.goto(xpos, ypos + count);
    }

    fn move_left(&self, count: u16) {
        let (xpos, ypos) = self.pos();
        self.goto(xpos - count, ypos);
    }

    fn save_position(&self) {
        cursor::save_cursor_pos(&self.screen_manager);
    }

    fn reset_position(&self) {
        cursor::reset_to_saved_position(&self.screen_manager);
    }

    fn hide(&self) {
        cursor::cursor_visibility(false, &self.screen_manager);
    }

    fn show(&self) {
        cursor::cursor_visibility(true, &self.screen_manager);
    }

    fn blink(&self, blink: bool) {}
}
