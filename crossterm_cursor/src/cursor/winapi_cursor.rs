//! This is a WINAPI specific implementation for cursor related actions.
//! This module is used for Windows terminals that do not support ANSI escape codes.
//! Note that the cursor position is 0 based. This means that we start counting at 0 when setting the cursor position.

use crossterm_utils::Result;

use crate::sys::winapi::{Cursor, Handle};

use super::ITerminalCursor;

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

    fn pos(&self) -> Result<(u16, u16)> {
        let cursor = Cursor::new()?;
        Ok(cursor.position()?.into())
    }

    fn move_up(&self, count: u16) -> Result<()> {
        let (xpos, ypos) = self.pos()?;
        self.goto(xpos, ypos - count)?;
        Ok(())
    }

    fn move_right(&self, count: u16) -> Result<()> {
        let (xpos, ypos) = self.pos()?;
        self.goto(xpos + count, ypos)?;
        Ok(())
    }

    fn move_down(&self, count: u16) -> Result<()> {
        let (xpos, ypos) = self.pos()?;
        self.goto(xpos, ypos + count)?;
        Ok(())
    }

    fn move_left(&self, count: u16) -> Result<()> {
        let (xpos, ypos) = self.pos()?;
        self.goto(xpos - count, ypos)?;
        Ok(())
    }

    fn save_position(&self) -> Result<()> {
        Cursor::save_cursor_pos()?;
        Ok(())
    }

    fn restore_position(&self) -> Result<()> {
        Cursor::restore_cursor_pos()?;
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

#[cfg(test)]
mod tests {
    use super::{ITerminalCursor, WinApiCursor};

    #[test]
    fn test_winapi_goto() {
        let cursor = WinApiCursor::new();

        let (saved_x, saved_y) = cursor.pos().unwrap();

        cursor.goto(saved_x + 1, saved_y + 1).unwrap();
        assert_eq!(cursor.pos().unwrap(), (saved_x + 1, saved_y + 1));

        cursor.goto(saved_x, saved_y).unwrap();
        assert_eq!(cursor.pos().unwrap(), (saved_x, saved_y));
    }

    #[test]
    fn test_winapi_save_and_restore() {
        let cursor = WinApiCursor::new();

        let (saved_x, saved_y) = cursor.pos().unwrap();

        cursor.save_position().unwrap();
        cursor.goto(saved_x + 1, saved_y + 1).unwrap();
        cursor.restore_position().unwrap();

        let (x, y) = cursor.pos().unwrap();

        assert_eq!(x, saved_x);
        assert_eq!(y, saved_y);
    }
}
