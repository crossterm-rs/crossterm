//! This contains the logic for working with the console buffer.

use super::{is_true, Handle, HandleType, ScreenBufferInfo};

use winapi::{
    shared::minwindef::TRUE,
    shared::ntdef::NULL,
    um::{
        minwinbase::SECURITY_ATTRIBUTES,
        wincon::{
            CreateConsoleScreenBuffer, GetConsoleScreenBufferInfo, SetConsoleActiveScreenBuffer,
            SetConsoleScreenBufferSize, CONSOLE_TEXTMODE_BUFFER, COORD,
        },
        winnt::{FILE_SHARE_READ, FILE_SHARE_WRITE, GENERIC_READ, GENERIC_WRITE, HANDLE},
    },
};

use std::io::{Error, Result};
use std::mem::size_of;

pub struct ScreenBuffer {
    handle: Handle,
}

impl ScreenBuffer {
    /// Create an instance of `ScreenBuffer` where the `HANDLE`, used for the functions this type wraps, is the current output handle.
    pub fn current() -> Result<ScreenBuffer> {
        Ok(ScreenBuffer {
            handle: Handle::new(HandleType::CurrentOutputHandle)?,
        })
    }

    /// Create new console screen buffer.
    ///
    /// Wraps the underlying function call: [CreateConsoleScreenBuffer]
    /// link: [https://docs.microsoft.com/en-us/windows/console/createconsolescreenbuffer]
    pub fn create() -> ScreenBuffer {
        let mut security_attr: SECURITY_ATTRIBUTES = SECURITY_ATTRIBUTES {
            nLength: size_of::<SECURITY_ATTRIBUTES>() as u32,
            lpSecurityDescriptor: NULL,
            bInheritHandle: TRUE,
        };

        unsafe {
            let new_screen_buffer = CreateConsoleScreenBuffer(
                GENERIC_READ |           // read/write access
                    GENERIC_WRITE,
                FILE_SHARE_READ | FILE_SHARE_WRITE, // shared
                &mut security_attr,                 // default security attributes
                CONSOLE_TEXTMODE_BUFFER,            // must be TEXTMODE
                NULL,
            );
            ScreenBuffer {
                handle: Handle::from(new_screen_buffer),
            }
        }
    }

    /// This will make this `ScreenBuffer` the active one.
    ///
    /// Wraps the underlying function call: [SetConsoleActiveScreenBuffer]
    /// link: [https://docs.microsoft.com/en-us/windows/console/setconsoleactivescreenbuffer]
    pub fn show(&self) -> Result<()> {
        unsafe {
            if !is_true(SetConsoleActiveScreenBuffer(*self.handle)) {
                return Err(Error::last_os_error());
            }
        }
        Ok(())
    }

    /// Get the screen buffer information like terminal size, cursor position, buffer size.
    ///
    /// Wraps the underlying function call: [GetConsoleScreenBufferInfo]
    /// link: [https://docs.microsoft.com/en-us/windows/console/getconsolescreenbufferinfo]
    pub fn info(&self) -> Result<ScreenBufferInfo> {
        let mut csbi = ScreenBufferInfo::new();

        unsafe {
            if !is_true(GetConsoleScreenBufferInfo(*self.handle, &mut csbi.0)) {
                return Err(Error::last_os_error());
            }
        }

        Ok(csbi)
    }

    /// Set the console screen buffer size to the given size.
    ///
    /// Wraps the underlying function call: [SetConsoleScreenBufferSize]
    /// link: [https://docs.microsoft.com/en-us/windows/console/setconsolescreenbuffersize]
    pub fn set_size(&self, x: i16, y: i16) -> Result<()> {
        unsafe {
            if !is_true(SetConsoleScreenBufferSize(
                *self.handle,
                COORD { X: x, Y: y },
            )) {
                return Err(Error::last_os_error());
            }
        }
        Ok(())
    }

    /// Get the underlining raw `HANDLE` used by this type to execute whit.
    pub fn get_handle(&self) -> &Handle {
        return &self.handle;
    }
}

impl From<Handle> for ScreenBuffer {
    fn from(handle: Handle) -> Self {
        ScreenBuffer { handle }
    }
}

impl From<HANDLE> for ScreenBuffer {
    fn from(handle: HANDLE) -> Self {
        ScreenBuffer {
            handle: Handle::from(handle),
        }
    }
}

#[cfg(test)]
mod test {
    use super::ScreenBuffer;

    #[test]
    fn screen_buffer_info() {
        let buffer = ScreenBuffer::current().unwrap();
        let info = buffer.info().unwrap();
        info.terminal_size();
        info.terminal_window();
        info.attributes();
        info.cursor_pos();
    }
}
