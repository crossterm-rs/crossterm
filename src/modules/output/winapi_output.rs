use super::IStdout;
use common::commands::win_commands::RawModeCommand;
use kernel::windows_kernel::{handle, writing};
use common::screen::RawScreen;
use winapi::um::winnt::HANDLE;

use std::any::Any;
use std::io;
use std::sync::Mutex;

/// This struct is a wrapper for WINAPI `HANDLE`
pub struct WinApiOutput;

impl WinApiOutput {
    pub fn new() -> WinApiOutput {
        WinApiOutput
    }
}

impl IStdout for WinApiOutput {
    fn write_str(&self, string: &str) -> io::Result<usize> {
        self.write(string.as_bytes())
    }

    fn write(&self, buf: &[u8]) -> io::Result<usize> {
        let handle = handle::get_current_handle().unwrap();
        writing::write_char_buffer(&handle, buf)
    }

    fn flush(&self) -> io::Result<()> {
        Ok(())
    }
}

unsafe impl Send for WinApiOutput {}

unsafe impl Sync for WinApiOutput {}
