use super::IStdout;
use screen::RawScreen;
use common::commands::win_commands::RawModeCommand;
use kernel::windows_kernel::{handle, writing};
use winapi::um::winnt::HANDLE;

use std::sync::Mutex;
use std::any::Any;
use std::io;

/// This struct is a wrapper for WINAPI `HANDLE`
pub struct WinApiOutput {
    pub handle: Mutex<HANDLE>,
    raw_mode: bool,
}

impl IStdout for WinApiOutput {

    fn write_str(&self, string: &str) -> io::Result<usize> {
        self.write(string.as_bytes())
    }

    fn write(&self, buf: &[u8]) -> io::Result<usize> {
        writing::write_char_buffer(&self.handle.lock().unwrap(), buf)
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

impl WinApiOutput {
    pub fn new() -> Self {
        let handle = handle::get_output_handle().unwrap();
        WinApiOutput { raw_mode: false, handle: Mutex::new(handle) }
    }

    pub fn set(&mut self, handle: HANDLE)
    {
        self.handle = Mutex::new(handle);
    }

    pub fn get_handle(&self) -> HANDLE
    {
        return self.handle.lock().unwrap().clone();
    }
}

unsafe impl Send for WinApiOutput {}

unsafe impl Sync for WinApiOutput {}
