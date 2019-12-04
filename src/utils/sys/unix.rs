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

pub(crate) fn is_raw_mode_enabled() -> bool {
    TERMINAL_MODE_PRIOR_RAW_MODE.lock().unwrap().is_some()
}

pub(crate) fn wrap_with_result(result: i32) -> Result<bool> {
    if result == -1 {
        Err(ErrorKind::IoError(io::Error::last_os_error()))
    } else {
        Ok(true)
    }
}

/// Transform the given mode into an raw mode (non-canonical) mode.
pub(crate) fn raw_terminal_attr(termios: &mut Termios) {
    unsafe { cfmakeraw(termios) }
}

pub(crate) fn get_terminal_attr() -> Result<Termios> {
    unsafe {
        let mut termios = mem::zeroed();
        wrap_with_result(tcgetattr(STDIN_FILENO, &mut termios))?;
        Ok(termios)
    }
}

pub(crate) fn set_terminal_attr(termios: &Termios) -> Result<bool> {
    wrap_with_result(unsafe { tcsetattr(STDIN_FILENO, TCSANOW, termios) })
}

pub(crate) fn enable_raw_mode() -> Result<()> {
    let mut original_mode = TERMINAL_MODE_PRIOR_RAW_MODE.lock().unwrap();

    if original_mode.is_some() {
        return Ok(());
    }

    let mut ios = get_terminal_attr()?;
    let original_mode_ios = ios;

    raw_terminal_attr(&mut ios);
    set_terminal_attr(&ios)?;

    // Keep it last - set the original mode only if we were able to switch to the raw mode
    *original_mode = Some(original_mode_ios);

    Ok(())
}

pub(crate) fn disable_raw_mode() -> Result<()> {
    let mut original_mode = TERMINAL_MODE_PRIOR_RAW_MODE.lock().unwrap();

    if let Some(original_mode_ios) = original_mode.as_ref() {
        set_terminal_attr(original_mode_ios)?;
        // Keep it last - remove the original mode only if we were able to switch back
        *original_mode = None;
    }

    Ok(())
}
