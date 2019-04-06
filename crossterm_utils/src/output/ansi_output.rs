//! This is an ANSI specific implementation for the screen write
//! This module is used for Windows 10 terminals and UNIX terminals by default.
//! This module uses the stdout to write to the console.

use super::IStdout;

use std::io::{self, stdout, Stdout, BufWriter, Write};

/// This struct is a wrapper for `Stdout`
pub struct AnsiOutput { }

impl IStdout for AnsiOutput {
    fn write_str(&self, string: &str) -> io::Result<usize> {
//        let out = &self.handle;
//        let mut handle = out.lock();
        let amt = stdout().write(string.as_bytes())?;
//        panic!("asdf");
        Ok(amt)
    }

    fn write(&self, buf: &[u8]) -> io::Result<usize> {
//        let out = &self.handle;
//        let mut handle = out.lock();
//        panic!("asdf");
        stdout().write(buf)
    }

    fn flush(&self) -> io::Result<()> {
//        let out = &self.handle;
//        let mut handle = out.lock();
//        panic!("asdf");
        stdout().flush()
    }
}

impl AnsiOutput {
    pub fn new() -> Self {
        AnsiOutput { }
    }
}
