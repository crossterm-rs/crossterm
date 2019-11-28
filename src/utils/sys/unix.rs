//! This module contains all `unix` specific terminal related logic.

use std::{io, mem, sync::Mutex};

pub use libc::termios as Termios;
use libc::{cfmakeraw, tcgetattr, tcsetattr, STDIN_FILENO, TCSANOW};

use lazy_static::lazy_static;

use super::super::error::{ErrorKind, Result};

lazy_static! {
    // Some(Termios) -> we're in the raw mode and this is the previous mode
    // None -> we're not in the raw mode
    static ref TERMINAL_MODE_PRIOR_RAW_MODE: Mutex<Option<Termios>> = Mutex::new(None);
}

pub fn is_raw_mode_enabled() -> bool {
    TERMINAL_MODE_PRIOR_RAW_MODE.lock().unwrap().is_some()
}

pub fn wrap_with_result(result: i32) -> Result<bool> {
    if result == -1 {
        Err(ErrorKind::IoError(io::Error::last_os_error()))
    } else {
        Ok(true)
    }
}
