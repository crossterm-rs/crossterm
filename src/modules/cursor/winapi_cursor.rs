//! This is a WINAPI specific implementation for cursor related actions.
//! This module is used for Windows terminals that do not support ANSI escape codes.
//! Note that the cursor position is 0 based. This means that we start counting at 0 when setting the cursor position.

use kernel::windows_kernel::Cursor;

use super::*;

/// This struct is a windows implementation for cursor related actions.
pub struct WinApiCursor;

impl WinApiCursor {
    pub fn new() -> Box<WinApiCursor> {
        Box::from(WinApiCursor)
    }
}

impl ITerminalCursor for WinApiCursor {
    fn goto(&self, x: u16, y: u16, _stdout: &Option<&Arc<TerminalOutput>>) {
        let cursor = Cursor::new().unwrap();
        cursor.goto(x as i16, y as i16);
    }

    fn pos(&self) -> (u16, u16) {
        let cursor = Cursor::new().unwrap();
        cursor.position().unwrap().into()
    }

    fn move_up(&self, count: u16, _stdout: &Option<&Arc<TerminalOutput>>) {
        let (xpos, ypos) = self.pos();
        self.goto(xpos, ypos - count, _stdout);
    }

    fn move_right(&self, count: u16, _stdout: &Option<&Arc<TerminalOutput>>) {
        let (xpos, ypos) = self.pos();
        self.goto(xpos + count, ypos, _stdout);
    }

    fn move_down(&self, count: u16, _stdout: &Option<&Arc<TerminalOutput>>) {
        let (xpos, ypos) = self.pos();
        self.goto(xpos, ypos + count, _stdout);
    }

    fn move_left(&self, count: u16, _stdout: &Option<&Arc<TerminalOutput>>) {
        let (xpos, ypos) = self.pos();
        self.goto(xpos - count, ypos, _stdout);
    }

    fn save_position(&self, _stdout: &Option<&Arc<TerminalOutput>>) {
        Cursor::save_cursor_pos();
    }

    fn reset_position(&self, _stdout: &Option<&Arc<TerminalOutput>>) {
        Cursor::reset_to_saved_position();
    }

    fn hide(&self, _stdout: &Option<&Arc<TerminalOutput>>) {
        Cursor::new().unwrap().set_visibility(false);
    }

    fn show(&self, _stdout: &Option<&Arc<TerminalOutput>>) {
        Cursor::new().unwrap().set_visibility(true);
    }

    fn blink(&self, _blink: bool, _stdout: &Option<&Arc<TerminalOutput>>) {}
}
