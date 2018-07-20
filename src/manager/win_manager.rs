use super::IScreenManager;

use kernel::windows_kernel::kernel;
use kernel::windows_kernel::writing;
use winapi::um::wincon::ENABLE_PROCESSED_OUTPUT;
use winapi::um::winnt::HANDLE;

use std::any::Any;
use std::io::{self, Write};
use std::rc::Rc;

pub struct WinApiScreenManager {
    pub is_alternate_screen: bool,
    output: HANDLE,
    alternate_handle: HANDLE,
}

impl IScreenManager for WinApiScreenManager {
    fn toggle_is_alternate_screen(&mut self, is_alternate_screen: bool) {
        self.is_alternate_screen = is_alternate_screen;
    }

    fn write_string(&mut self, string: String) -> io::Result<usize>
    {
        self.write(string.as_bytes())
    }

    fn write_str(&mut self, string: &str) -> io::Result<usize>
    {
        self.write(string.as_bytes())
    }

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if self.is_alternate_screen {
            writing::write_char_buffer(&self.alternate_handle, buf)
        } else {
            writing::write_char_buffer(&self.output, buf)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn as_any(&mut self) -> &mut Any {
        self
    }
}

// for winapi we have some custom implementation that will be used by windows only. You can get a reference to this implementation by using the any and that cast it to this struct.
impl WinApiScreenManager {
    pub fn new() -> Self {
        WinApiScreenManager {
            output: kernel::get_output_handle(),
            is_alternate_screen: false,
            alternate_handle: kernel::get_output_handle(),
        }
    }

    /// Set the alternate handle to the given handle.
    pub fn set_alternate_handle(&mut self, alternate_handle: HANDLE) {
        self.alternate_handle = alternate_handle;

        // needs to be turned on so that escape characters like \n and \t will be processed.
        kernel::set_console_mode(&self.alternate_handle, ENABLE_PROCESSED_OUTPUT as u32);
    }

    /// get the current screen handle.
    pub fn get_handle(&mut self) -> &HANDLE {
        if self.is_alternate_screen {
            return &self.alternate_handle;

        } else {
            return &self.output;
        }
    }
}
