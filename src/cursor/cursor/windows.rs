//! This is a WINAPI specific implementation for cursor related actions.
//! This module is used for Windows terminals that do not support ANSI escape codes.
//! Note that the cursor position is 0 based. This means that we start counting at 0 when setting the cursor position.

use crate::utils::Result;

use super::{super::sys::windows::ScreenBufferCursor, Cursor};

/// This struct is a windows implementation for cursor related actions.
pub(crate) struct WinApiCursor;

impl WinApiCursor {
    pub(crate) fn new() -> WinApiCursor {
        WinApiCursor
    }
}

impl Cursor for WinApiCursor {
    fn goto(&self, x: u16, y: u16) -> Result<()> {
        let cursor = ScreenBufferCursor::new()?;
        cursor.goto(x as i16, y as i16)?;
        Ok(())
    }

    fn pos(&self) -> Result<(u16, u16)> {
        let cursor = ScreenBufferCursor::new()?;
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
        ScreenBufferCursor::save_cursor_pos()?;
        Ok(())
    }

    fn restore_position(&self) -> Result<()> {
        ScreenBufferCursor::restore_cursor_pos()?;
        Ok(())
    }

    fn hide(&self) -> Result<()> {
        ScreenBufferCursor::new()?.set_visibility(false)?;
        Ok(())
    }

    fn show(&self) -> Result<()> {
        ScreenBufferCursor::new()?.set_visibility(true)?;
        Ok(())
    }

    fn blink(&self, _blink: bool) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Cursor, WinApiCursor};

    #[test]
    fn test_goto() {
        let cursor = WinApiCursor::new();

        let (saved_x, saved_y) = cursor.pos().unwrap();

        cursor.goto(saved_x + 1, saved_y + 1).unwrap();
        assert_eq!(cursor.pos().unwrap(), (saved_x + 1, saved_y + 1));

        cursor.goto(saved_x, saved_y).unwrap();
        assert_eq!(cursor.pos().unwrap(), (saved_x, saved_y));
    }

    #[test]
    fn test_save_restore_position() {
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
