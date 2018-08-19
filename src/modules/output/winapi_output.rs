use super::IStdout;
use kernel::windows_kernel::{handle, kernel, writing};
use winapi::um::wincon::ENABLE_PROCESSED_OUTPUT;
use winapi::um::winnt::HANDLE;

use std::sync::Mutex;
use std::ptr::NonNull;
use std::any::Any;
use std::io;


/// This struct is a wrapper for WINAPI `HANDLE`
pub struct WinApiOutput {
    pub handle: Mutex<HANDLE>,
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
        WinApiOutput { handle: Mutex::new(handle::get_output_handle().unwrap()) }
    }

    pub fn set(&mut self, handle: HANDLE)
    {
        self.handle = Mutex::new(handle);
    }

    pub fn get_handle(&self) -> HANDLE
    {
        let gx = self.handle.lock();
        gx.unwrap().clone()
    }
}

unsafe impl Send for WinApiOutput {}

unsafe impl Sync for WinApiOutput {}
