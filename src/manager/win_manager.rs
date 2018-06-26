use super::IScreenManager;
use kernel::windows_kernel::kernel;
use winapi::um::winnt::HANDLE;

use std::io::{self,Write};
use std::any::Any;
use std::rc::Rc;

pub struct WinApiScreenManager
{
    pub is_alternate_screen: bool,
    output: HANDLE,
    alternate_handle: HANDLE
}

impl IScreenManager for WinApiScreenManager
{
    fn toggle_is_alternate_screen(&mut self, is_alternate_screen: bool)
    {
        self.is_alternate_screen = is_alternate_screen;
    }

    fn write_ansi(&mut self, string: String)
    { }

    fn write_ansi_str(&mut self, string: &str)
    { }

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if self.is_alternate_screen
            {
                kernel::write_char_buffer(self.alternate_handle, buf);
            }
            else {
                kernel::write_char_buffer(self.output, buf);
            }
        Ok(0)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn as_any(&mut self) -> &mut Any
    {
        self
    }
}

impl WinApiScreenManager {
    pub fn new() -> Self {
        WinApiScreenManager {
            output: kernel::get_output_handle(),
            is_alternate_screen: false,
            alternate_handle: kernel::get_output_handle(),
        }
    }

    pub fn set_alternate_handle(&mut self, alternate_handle: HANDLE)
    {
        self.alternate_handle = alternate_handle;
    }

    pub fn get_handle(&mut self) -> Rc<HANDLE>
    {
        if self.is_alternate_screen
            {
                return Rc::from(self.alternate_handle);
            }
            else {
                return Rc::from(self.output);
            }
    }
}