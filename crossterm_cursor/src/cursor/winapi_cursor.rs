//! This is a WINAPI specific implementation for cursor related actions.
//! This module is used for Windows terminals that do not support ANSI escape codes.
//! Note that the cursor position is 0 based. This means that we start counting at 0 when setting the cursor position.

use super::ITerminalCursor;
use crate::sys::winapi::{Cursor, Handle};
use crossterm_utils::Result;

/// This struct is a windows implementation for cursor related actions.
pub struct WinApiCursor;

impl WinApiCursor {
    pub fn new() -> Box<WinApiCursor> {
        Box::from(WinApiCursor)
    }
}

impl ITerminalCursor for WinApiCursor {
    fn goto(&self, x: u16, y: u16) -> Result<()> {
        let cursor = Cursor::new()?;
        cursor.goto(x as i16, y as i16)?;
        Ok(())
    }

    fn pos(&self) -> (u16, u16) {
        let cursor = Cursor::new().unwrap();
        cursor.position().unwrap().into()
    }

    fn move_up(&self, count: u16) -> Result<()> {
        let (xpos, ypos) = self.pos();
        self.goto(xpos, ypos - count)?;
        Ok(())
    }

    fn move_right(&self, count: u16) -> Result<()> {
        let (xpos, ypos) = self.pos();
        self.goto(xpos + count, ypos)?;
        Ok(())
    }

    fn move_down(&self, count: u16) -> Result<()> {
        let (xpos, ypos) = self.pos();
        self.goto(xpos, ypos + count)?;
        Ok(())
    }

    fn move_left(&self, count: u16) -> Result<()> {
        let (xpos, ypos) = self.pos();
        self.goto(xpos - count, ypos)?;
        Ok(())
    }

    fn save_position(&self) -> Result<()> {
        Cursor::save_cursor_pos()?;
        Ok(())
    }

    fn reset_position(&self) -> Result<()> {
        Cursor::reset_to_saved_position()?;
        Ok(())
    }

    fn hide(&self) -> Result<()> {
        Cursor::from(Handle::current_out_handle()?).set_visibility(false)?;
        Ok(())
    }

    fn show(&self) -> Result<()> {
        Cursor::from(Handle::current_out_handle()?).set_visibility(true)?;
        Ok(())
    }

    fn blink(&self, _blink: bool) -> Result<()> {
        Ok(())
    }
}
