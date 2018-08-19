//! This is an WINAPI specific implementation for cursor related action.
//! This module is used for windows terminals that do not support ANSI escape codes.
//! Note that the cursor position is 0 based. This means that we start counting at 0 when setting the cursor position ect.

use kernel::windows_kernel::cursor;

use super::*;

/// This struct is an windows implementation for cursor related actions.
pub struct WinApiCursor;

impl WinApiCursor {
    pub fn new() -> Box<WinApiCursor> {
        Box::from(WinApiCursor {})
    }
}

impl ITerminalCursor for WinApiCursor {
    fn goto(&self, x: u16, y: u16, stdout: &Arc<TerminalOutput>) {
        cursor::set_console_cursor_position(x as i16, y as i16, stdout);
    }

    fn pos(&self, stdout: &Arc<TerminalOutput>) -> (u16, u16) {
        cursor::pos(stdout)
    }

    fn move_up(&self, count: u16, stdout: &Arc<TerminalOutput>) {
        let (xpos, ypos) = self.pos(stdout);
        self.goto(xpos, ypos - count, stdout);
    }

    fn move_right(&self, count: u16, stdout: &Arc<TerminalOutput>) {
        let (xpos, ypos) = self.pos(stdout);
        self.goto(xpos + count, ypos, stdout);
    }

    fn move_down(&self, count: u16, stdout: &Arc<TerminalOutput>) {
        let (xpos, ypos) = self.pos(stdout);
        self.goto(xpos, ypos + count, stdout);
    }

    fn move_left(&self, count: u16, stdout: &Arc<TerminalOutput>) {
        let (xpos, ypos) = self.pos(stdout);
        self.goto(xpos - count, ypos, stdout);
    }

    fn save_position(&self, stdout: &Arc<TerminalOutput>) {
        cursor::save_cursor_pos(stdout);
    }

    fn reset_position(&self, stdout: &Arc<TerminalOutput>) {
        cursor::reset_to_saved_position(stdout);
    }

    fn hide(&self, stdout: &Arc<TerminalOutput>) {
        cursor::cursor_visibility(false, stdout);
    }

    fn show(&self, stdout: &Arc<TerminalOutput>) {
        cursor::cursor_visibility(true, stdout);
    }
    fn blink(&self, blink: bool, stdout: &Arc<TerminalOutput>) {}
}
