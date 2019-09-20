//! This module handles some logic for cursor interaction in the windows console.

use std::io;

use winapi::{
    shared::minwindef::{FALSE, TRUE},
    um::wincon::{SetConsoleCursorInfo, SetConsoleCursorPosition, CONSOLE_CURSOR_INFO, COORD},
    um::winnt::HANDLE,
};

use crossterm_utils::Result;
pub use crossterm_winapi::{is_true, Coord, Handle, HandleType, ScreenBuffer};

#[cfg(windows)]
pub fn get_cursor_position() -> Result<(u16, u16)> {
    let cursor = Cursor::new()?;
    Ok(cursor.position()?.into())
}

#[cfg(windows)]
pub fn show_cursor(show_cursor: bool) -> Result<()> {
    Cursor::from(Handle::current_out_handle()?).set_visibility(show_cursor)
}

/// This stores the cursor pos, at program level. So it can be recalled later.
static mut SAVED_CURSOR_POS: (u16, u16) = (0, 0);

pub struct Cursor {
    screen_buffer: ScreenBuffer,
}

impl Cursor {
    pub fn new() -> Result<Cursor> {
        Ok(Cursor {
            screen_buffer: ScreenBuffer::from(Handle::new(HandleType::CurrentOutputHandle)?),
        })
    }

    /// get the current cursor position.
    pub fn position(&self) -> Result<Coord> {
        Ok(self.screen_buffer.info()?.cursor_pos())
    }

    /// Set the cursor position to the given x and y. Note that this is 0 based.
    pub fn goto(&self, x: i16, y: i16) -> Result<()> {
        if x < 0 || x >= <i16>::max_value() {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "Argument Out of Range Exception when setting cursor position to X: {}",
                    x
                ),
            ))?;
        }

        if y < 0 || y >= <i16>::max_value() {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "Argument Out of Range Exception when setting cursor position to Y: {}",
                    y
                ),
            ))?;
        }

        let position = COORD { X: x, Y: y };

        unsafe {
            if !is_true(SetConsoleCursorPosition(
                **self.screen_buffer.handle(),
                position,
            )) {
                Err(io::Error::last_os_error())?;
            }
        }
        Ok(())
    }

    /// change the cursor visibility.
    pub fn set_visibility(&self, visable: bool) -> Result<()> {
        let cursor_info = CONSOLE_CURSOR_INFO {
            dwSize: 100,
            bVisible: if visable { TRUE } else { FALSE },
        };

        unsafe {
            if !is_true(SetConsoleCursorInfo(
                **self.screen_buffer.handle(),
                &cursor_info,
            )) {
                Err(io::Error::last_os_error())?;
            }
        }
        Ok(())
    }

    /// Reset to saved cursor position
    pub fn restore_cursor_pos() -> Result<()> {
        let cursor = Cursor::new()?;

        unsafe {
            cursor.goto(SAVED_CURSOR_POS.0 as i16, SAVED_CURSOR_POS.1 as i16)?;
        }

        Ok(())
    }

    /// Save current cursor position to recall later.
    pub fn save_cursor_pos() -> Result<()> {
        let cursor = Cursor::new()?;
        let position = cursor.position()?;

        unsafe {
            SAVED_CURSOR_POS = (position.x as u16, position.y as u16);
        }

        Ok(())
    }
}

impl From<Handle> for Cursor {
    fn from(handle: Handle) -> Self {
        Cursor {
            screen_buffer: ScreenBuffer::from(handle),
        }
    }
}

impl From<HANDLE> for Cursor {
    fn from(handle: HANDLE) -> Self {
        Cursor {
            screen_buffer: ScreenBuffer::from(handle),
        }
    }
}
