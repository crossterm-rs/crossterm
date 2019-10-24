//! WinApi related logic to cursor manipulation.

use std::io;
use std::sync::Mutex;

use crossterm_winapi::{is_true, Coord, Handle, HandleType, ScreenBuffer};
use winapi::{
    shared::minwindef::{FALSE, TRUE},
    um::wincon::{SetConsoleCursorInfo, SetConsoleCursorPosition, CONSOLE_CURSOR_INFO, COORD},
    um::winnt::HANDLE,
};

use lazy_static::lazy_static;

use crate::utils::Result;

/// Returns the cursor position (column, row).
/// The counting starts from 0 were column 0 and row 0 is the top left.
pub fn position() -> Result<(u16, u16)> {
    let cursor = ScreenBufferCursor::new()?;
    Ok(cursor.position()?.into())
}

pub(crate) fn show_cursor(show_cursor: bool) -> Result<()> {
    ScreenBufferCursor::from(Handle::current_out_handle()?).set_visibility(show_cursor)
}

pub(crate) fn move_to(x: u16, y: u16) -> Result<()> {
    let cursor = ScreenBufferCursor::new()?;
    cursor.goto(x as i16, y as i16)?;
    Ok(())
}

pub(crate) fn move_up(count: u16) -> Result<()> {
    let (xpos, ypos) = position()?;
    move_to(xpos, ypos - count)?;
    Ok(())
}

pub(crate) fn move_right(count: u16) -> Result<()> {
    let (xpos, ypos) = position()?;
    move_to(xpos + count, ypos)?;
    Ok(())
}

pub(crate) fn move_down(count: u16) -> Result<()> {
    let (xpos, ypos) = position()?;
    move_to(xpos, ypos + count)?;
    Ok(())
}

pub(crate) fn move_left(count: u16) -> Result<()> {
    let (xpos, ypos) = position()?;
    move_to(xpos - count, ypos)?;
    Ok(())
}

pub(crate) fn save_position() -> Result<()> {
    ScreenBufferCursor::save_cursor_pos()?;
    Ok(())
}

pub(crate) fn restore_position() -> Result<()> {
    ScreenBufferCursor::restore_cursor_pos()?;
    Ok(())
}

lazy_static! {
    static ref SAVED_CURSOR_POS: Mutex<Option<(i16, i16)>> = Mutex::new(None);
}

pub(crate) struct ScreenBufferCursor {
    screen_buffer: ScreenBuffer,
}

impl ScreenBufferCursor {
    pub(crate) fn new() -> Result<ScreenBufferCursor> {
        Ok(ScreenBufferCursor {
            screen_buffer: ScreenBuffer::from(Handle::new(HandleType::CurrentOutputHandle)?),
        })
    }

    /// get the current cursor position.
    pub(crate) fn position(&self) -> Result<Coord> {
        Ok(self.screen_buffer.info()?.cursor_pos())
    }

    /// Set the cursor position to the given x and y. Note that this is 0 based.
    pub(crate) fn goto(&self, x: i16, y: i16) -> Result<()> {
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
    pub(crate) fn set_visibility(&self, visible: bool) -> Result<()> {
        let cursor_info = CONSOLE_CURSOR_INFO {
            dwSize: 100,
            bVisible: if visible { TRUE } else { FALSE },
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
    pub(crate) fn restore_cursor_pos() -> Result<()> {
        let cursor = ScreenBufferCursor::new()?;

        if let Some((x, y)) = *SAVED_CURSOR_POS.lock().unwrap() {
            cursor.goto(x, y)?;
        }

        Ok(())
    }

    /// Save current cursor position to recall later.
    pub(crate) fn save_cursor_pos() -> Result<()> {
        let cursor = ScreenBufferCursor::new()?;
        let position = cursor.position()?;

        let mut locked_pos = SAVED_CURSOR_POS.lock().unwrap();
        *locked_pos = Some((position.x, position.y));

        Ok(())
    }
}

impl From<Handle> for ScreenBufferCursor {
    fn from(handle: Handle) -> Self {
        ScreenBufferCursor {
            screen_buffer: ScreenBuffer::from(handle),
        }
    }
}

impl From<HANDLE> for ScreenBufferCursor {
    fn from(handle: HANDLE) -> Self {
        ScreenBufferCursor {
            screen_buffer: ScreenBuffer::from(handle),
        }
    }
}
