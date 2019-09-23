//! This module handles some logic for cursor interaction in the windows console.

use std::io;
use std::sync::Mutex;

use lazy_static::lazy_static;
use winapi::{
    shared::minwindef::{FALSE, TRUE},
    um::wincon::{SetConsoleCursorInfo, SetConsoleCursorPosition, CONSOLE_CURSOR_INFO, COORD},
    um::winnt::HANDLE,
};

use crossterm_utils::Result;
use crossterm_winapi::{is_true, Coord, Handle, HandleType, ScreenBuffer};

pub(crate) fn get_cursor_position() -> Result<(u16, u16)> {
    let cursor = ScreenBufferCursor::new()?;
    Ok(cursor.position()?.into())
}

pub(crate) fn show_cursor(show_cursor: bool) -> Result<()> {
    ScreenBufferCursor::from(Handle::current_out_handle()?).set_visibility(show_cursor)
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
    pub(crate) fn set_visibility(&self, visable: bool) -> Result<()> {
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
