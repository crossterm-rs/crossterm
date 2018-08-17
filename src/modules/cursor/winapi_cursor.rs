//! This is an WINAPI specific implementation for cursor related action.
//! This module is used for windows terminals that do not support ANSI escape codes.
//! Note that the cursor position is 0 based. This means that we start counting at 0 when setting the cursor position ect.

use super::super::super::write::{Stdout, WinApiStdout};
use super::ITerminalCursor;

use kernel::windows_kernel::{cursor, kernel};

use std::sync::Arc;

/// This struct is an windows implementation for cursor related actions.
pub struct WinApiCursor;

impl WinApiCursor {
    pub fn new() -> Box<WinApiCursor> {
        Box::from(WinApiCursor {})
    }
}

impl ITerminalCursor for WinApiCursor {
    fn goto(&self, x: u16, y: u16, screen_manager: &Arc<Stdout>) {
        cursor::set_console_cursor_position(x as i16, y as i16, screen_manager);
    }

    fn pos(&self, screen_manager: &Arc<Stdout>) -> (u16, u16) {
        cursor::pos(screen_manager)
    }

    fn move_up(&self, count: u16, screen_manager: &Arc<Stdout>) {
        let (xpos, ypos) = self.pos(screen_manager);
        self.goto(xpos, ypos - count, screen_manager);
    }

    fn move_right(&self, count: u16, screen_manager: &Arc<Stdout>) {
        let (xpos, ypos) = self.pos(screen_manager);
        self.goto(xpos + count, ypos, screen_manager);
    }

    fn move_down(&self, count: u16, screen_manager: &Arc<Stdout>) {
        let (xpos, ypos) = self.pos(screen_manager);
        self.goto(xpos, ypos + count, screen_manager);
    }

    fn move_left(&self, count: u16, screen_manager: &Arc<Stdout>) {
        let (xpos, ypos) = self.pos(screen_manager);
        self.goto(xpos - count, ypos, screen_manager);
    }

    fn save_position(&self, screen_manager: &Arc<Stdout>) {
        cursor::save_cursor_pos(screen_manager);
    }

    fn reset_position(&self, screen_manager: &Arc<Stdout>) {
        cursor::reset_to_saved_position(screen_manager);
    }

    fn hide(&self, screen_manager: &Arc<Stdout>) {
        cursor::cursor_visibility(false, screen_manager);
    }

    fn show(&self, screen_manager: &Arc<Stdout>) {
        cursor::cursor_visibility(true, screen_manager);
    }
    fn blink(&self, blink: bool, screen_manager: &Arc<Stdout>) {}
}
