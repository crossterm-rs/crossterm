//! This is an ANSI specific implementation for the screen write
//! This module is used for Windows 10 terminals and UNIX terminals by default.
//! This module uses the stdout to write to the console.

use super::IStdout;

use std::io::{self, stdout, Stdout, BufWriter, Write};

/// This struct is a wrapper for `Stdout`
pub struct AnsiOutput {
    pub handle: BufWriter<Stdout>
}

impl IStdout for AnsiOutput {
    fn write_str(&self, string: &str) -> io::Result<usize> {
        let amt = self.handle.get_mut().write(string.as_bytes())?;
        Ok(amt)
    }

    fn write(&self, buf: &[u8]) -> io::Result<usize> {
        let out = &self.handle;
        let mut handle = out.lock();
        handle.write(buf)
    }

    fn flush(&self) -> io::Result<()> {
        let out = &self.handle;
        let mut handle = out.lock();
        handle.flush()
    }
}

impl AnsiOutput {
    pub fn new() -> Self {
        AnsiOutput { handle: stdout() }
    }
}
