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

lazy_static! {
    static ref SAVED_CURSOR_POS: Mutex<Option<(i16, i16)>> = Mutex::new(None);
}

/// Returns the cursor position (column, row).
/// The counting starts from 0 were column 0 and row 0 is the top left.
pub fn position() -> Result<(u16, u16)> {
    let cursor = ScreenBufferCursor::new()?;
    Ok(cursor.position()?.into())
}

pub(crate) fn show_cursor(show_cursor: bool) -> Result<()> {
    ScreenBufferCursor::from(Handle::current_out_handle()?).set_visibility(show_cursor)
}

pub(crate) fn move_to(column: u16, row: u16) -> Result<()> {
    let cursor = ScreenBufferCursor::new()?;
    cursor.move_to(column as i16, row as i16)?;
    Ok(())
}

pub(crate) fn move_up(count: u16) -> Result<()> {
    let (column, row) = position()?;
    move_to(column, row - count)?;
    Ok(())
}

pub(crate) fn move_right(count: u16) -> Result<()> {
    let (column, row) = position()?;
    move_to(column + count, row)?;
    Ok(())
}

pub(crate) fn move_down(count: u16) -> Result<()> {
    let (column, row) = position()?;
    move_to(column, row + count)?;
    Ok(())
}

pub(crate) fn move_left(count: u16) -> Result<()> {
    let (column, row) = position()?;
    move_to(column - count, row)?;
    Ok(())
}

pub(crate) fn save_position() -> Result<()> {
    ScreenBufferCursor::new()?.save_position()?;
    Ok(())
}

pub(crate) fn restore_position() -> Result<()> {
    ScreenBufferCursor::new()?.restore_position()?;
    Ok(())
}

/// WinApi wrapper over terminal cursor behaviour.
struct ScreenBufferCursor {
    screen_buffer: ScreenBuffer,
}

impl ScreenBufferCursor {
    fn new() -> Result<ScreenBufferCursor> {
        Ok(ScreenBufferCursor {
            screen_buffer: ScreenBuffer::from(Handle::new(HandleType::CurrentOutputHandle)?),
        })
    }

    fn position(&self) -> Result<Coord> {
        Ok(self.screen_buffer.info()?.cursor_pos())
    }

    fn move_to(&self, x: i16, y: i16) -> Result<()> {
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

    fn set_visibility(&self, visible: bool) -> Result<()> {
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

    fn restore_position(&self) -> Result<()> {
        if let Some((x, y)) = *SAVED_CURSOR_POS.lock().unwrap() {
            self.move_to(x, y)?;
        }

        Ok(())
    }

    fn save_position(&self) -> Result<()> {
        let position = self.position()?;

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
