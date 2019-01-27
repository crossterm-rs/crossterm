//! This module is used for enabling and disabling raw mode for the terminal.
//!
//! What exactly is raw state:
//! - No line buffering.
//!    Normally the terminals uses line buffering. This means that the input will be send to the terminal line by line.
//!    With raw mode the input will be send one byte at a time.
//! - Input
//!   All input has to be written manually by the programmer.
//! - Characters
//!   The characters are not processed by the terminal driver, but are sent straight through.
//!   Special character have no meaning, like backspace will not be interpret as backspace but instead will be directly send to the terminal.
//! - Escape characters
//!   Note that in raw modes `\n` `\r` will move to the new line but the cursor will be at the same position as before on the new line therefor use `\n\r` to start at the new line at the first cell.
//!
//! With these modes you can easier design the terminal screen.

use crate::sys;

use std::io;

/// A wrapper for the raw terminal state. Which can be used to write to.
///
/// Although this type is available for you to use I would recommend using `Screen` instead.
/// Note that when you want to use input and raw mode you should use `Screen`.
pub struct RawScreen;

impl RawScreen {
    /// Put terminal in raw mode.
    pub fn into_raw_mode() -> io::Result<()> {
        #[cfg(not(target_os = "windows"))]
        let mut command = sys::unix::RawModeCommand::new();
        #[cfg(target_os = "windows")]
        let mut command = sys::winapi::RawModeCommand::new();

        let _result = command.enable();

        Ok(())
    }

    /// Put terminal back in original modes.
    pub fn disable_raw_modes() -> io::Result<()> {
        #[cfg(not(target_os = "windows"))]
        let mut command = sys::unix::RawModeCommand::new();
        #[cfg(target_os = "windows")]
        let command = sys::winapi::RawModeCommand::new();

        command.disable()?;
        Ok(())
    }
}
