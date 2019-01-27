//! This module provides one place to work with the screen.
//!
//! In Rust we can call `stdout()` to get a handle to the current default console handle.
//! However, we can't use `stdout()` to access the alternate screen handle therefore we also won't be able to use `print!(), println!(), or write!()`.
//! The same goes for coloring, cursor movement, input, and terminal actions.
//! All of those functions are writing to the standard output and not to our alternate screen we are currently on.
//!
//! To get the handle to the `alternate screen` we first need to store this handle so that we are able to call it later on.
//! Through this stored handle, crossterm can write to or execute commands at the current screen whether it be an alternate screen or main screen.
//!
//! For UNIX and Windows10 systems, we store the handle gotten from `stdout()`. For Windows systems who are not supporting ANSI escape codes, we can call `CONOUT$` to get the current screen `HANDLE`.

use super::*;

use std::default::Default;
use std::io::Write;

/// Struct that is a handle to the current terminal screen.
///
/// For UNIX and Windows 10 `stdout()` will be used as handle. And for Windows systems, not supporting ANSI escape codes, will use WinApi's `HANDLE` as handle.
pub struct TerminalOutput {
    stdout: Box<IStdout + Send + Sync>,
    /// checks if this output is in raw mode.
    pub is_in_raw_mode: bool,
}

impl TerminalOutput {
    /// Create a new screen write instance whereon screen related actions can be performed.
    pub fn new(raw_mode: bool) -> Self {
        #[cfg(target_os = "windows")]
        let stdout: Box<IStdout + Send + Sync> =
            functions::get_module::<Box<IStdout + Send + Sync>>(
                Box::from(WinApiOutput::new()),
                Box::from(AnsiOutput::new()),
            )
            .unwrap();

        #[cfg(not(target_os = "windows"))]
        let stdout = Box::from(AnsiOutput::new()) as Box<IStdout + Send + Sync>;

        TerminalOutput {
            stdout,
            is_in_raw_mode: raw_mode,
        }
    }

    /// Write String to the current screen.
    pub fn write_string(&self, string: String) -> io::Result<usize> {
        self.stdout.write_str(string.as_str())
    }

    /// Flush the current screen.
    pub fn flush(&self) -> io::Result<()> {
        self.stdout.flush()
    }

    /// Write &str to the current screen.
    pub fn write_str(&self, string: &str) -> io::Result<usize> {
        self.stdout.write_str(string)
    }

    /// Write buffer to the screen
    pub fn write_buf(&self, buf: &[u8]) -> io::Result<usize> {
        self.stdout.write(buf)
    }
}

impl Write for TerminalOutput {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.write_buf(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }
}

impl Default for TerminalOutput {
    /// Get the default handle to the current screen.
    fn default() -> Self {
        #[cfg(target_os = "windows")]
        let stdout = functions::get_module::<Box<IStdout + Send + Sync>>(
            Box::from(WinApiOutput::new()),
            Box::from(AnsiOutput::new()),
        )
        .unwrap();

        #[cfg(not(target_os = "windows"))]
        let stdout = Box::from(AnsiOutput::new()) as Box<IStdout + Send + Sync>;

        TerminalOutput {
            stdout,
            is_in_raw_mode: false,
        }
    }
}
