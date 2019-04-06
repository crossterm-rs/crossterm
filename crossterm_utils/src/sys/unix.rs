//! This module contains all `unix` specific terminal related logic.

use libc::c_int;
pub use libc::termios as Termios;
use libc::{self, TCSADRAIN};

use std::{fs, io, mem};

static mut ORIGINAL_TERMINAL_MODE: Option<Termios> = None;
pub static mut RAW_MODE_ENABLED_BY_SYSTEM: bool = false;
pub static mut RAW_MODE_ENABLED_BY_USER: bool = false;

fn cvt(t: i32) -> io::Result<()> {
    if t == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

/// Transform the given mode into an raw mode (non-canonical) mode.
pub fn raw_terminal_attr(termios: &mut Termios) {
    extern "C" {
        pub fn cfmakeraw(termptr: *mut Termios);
    }
    unsafe { cfmakeraw(termios) }
}

pub fn get_terminal_attr() -> io::Result<Termios> {
    extern "C" {
        pub fn tcgetattr(fd: c_int, termptr: *mut Termios) -> c_int;
    }
    unsafe {
        let mut termios = mem::zeroed();
        cvt(tcgetattr(0, &mut termios))?;
        Ok(termios)
    }
}

pub fn set_terminal_attr(termios: &Termios) -> io::Result<()> {
    extern "C" {
        pub fn tcsetattr(fd: c_int, opt: c_int, termptr: *const Termios) -> c_int;
    }
    cvt(unsafe { tcsetattr(0, 0, termios) }).and(Ok(()))
}

pub fn into_raw_mode() -> io::Result<()> {
    let mut ios = get_terminal_attr()?;
    let prev_ios = ios;

    unsafe {
        if ORIGINAL_TERMINAL_MODE.is_none() {
            ORIGINAL_TERMINAL_MODE = Some(prev_ios.clone())
        }
    }

    raw_terminal_attr(&mut ios);
    set_terminal_attr(&ios)?;
    Ok(())
}

pub fn disable_raw_mode() -> io::Result<()> {
    unsafe {
        if ORIGINAL_TERMINAL_MODE.is_some() {
            set_terminal_attr(unsafe { &ORIGINAL_TERMINAL_MODE.unwrap() })?;
        }
    }
    Ok(())
}
