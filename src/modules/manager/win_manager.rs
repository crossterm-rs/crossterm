use super::IScreenManager;
use kernel::windows_kernel::{handle, kernel, writing};
use winapi::um::wincon::ENABLE_PROCESSED_OUTPUT;
use winapi::um::winnt::HANDLE;

use std::ptr::NonNull;
use std::any::Any;
use std::io::{self, Write};
use std::sync::Arc;

/// This struct is an WINAPI implementation for screen related actions.
pub struct WinApiScreenManager {
    is_alternate_screen: bool,
    is_raw_screen: bool,
    output: HANDLE,
    alternate_handle: HANDLE,
}

impl IScreenManager for WinApiScreenManager {
    fn set_is_raw_screen(&mut self, value: bool) {
        self.is_raw_screen = value;
    }
    fn set_is_alternate_screen(&mut self, value: bool) {
        self.is_alternate_screen = value;
    }

    fn is_raw_screen(&self) -> bool {
        self.is_raw_screen
    }

    fn is_alternate_screen(&self) -> bool {
        self.is_alternate_screen
    }

    fn write_str(&self, string: &str) -> io::Result<usize> {
        self.write(string.as_bytes())
    }

    fn write(&self, buf: &[u8]) -> io::Result<usize> {
        if self.is_alternate_screen {
            writing::write_char_buffer(&self.alternate_handle, buf)
        } else {
            writing::write_char_buffer(&self.output, buf)
        }
    }

    fn flush(&self) -> io::Result<()> {
        Ok(())
    }

    fn as_any(&self) -> &Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut Any {
        self
    }
}

// for winapi we have some custom implementation that will be used by windows only. You can get a reference to this implementation by using the `as_any()` and that cast it to this struct.
impl WinApiScreenManager {
    /// Create a new instance.
    pub fn new() -> Self {

        WinApiScreenManager {
            output:  handle::get_output_handle().unwrap(),
            alternate_handle: handle::get_output_handle().unwrap(),
            is_alternate_screen: false,
            is_raw_screen: false,
        }
    }

    /// Set the alternate handle to the given handle.
    pub fn set_alternate_handle(&mut self, alternate_handle: HANDLE) {
        self.alternate_handle = alternate_handle;
        // needs to be turned on so that escape characters like \n and \t will be processed.
        kernel::set_console_mode(&self.alternate_handle, ENABLE_PROCESSED_OUTPUT as u32);
    }

    /// get the current screen handle.
    pub fn get_handle(&self) -> &HANDLE {
        if self.is_alternate_screen {
            return &self.alternate_handle;
        } else {
            return &self.output;
        }
    }
}

unsafe impl Send for WinApiScreenManager {}
