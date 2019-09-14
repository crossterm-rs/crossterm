//! This module contains all `unix` specific terminal related logic.

use std::{io, mem};

pub use libc::{c_int, termios as Termios};

static mut ORIGINAL_TERMINAL_MODE: Option<Termios> = None;
pub static mut RAW_MODE_ENABLED: bool = false;

fn unwrap(t: i32) -> io::Result<()> {
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
        unwrap(tcgetattr(0, &mut termios))?;
        Ok(termios)
    }
}

pub fn set_terminal_attr(termios: &Termios) -> io::Result<()> {
    extern "C" {
        pub fn tcsetattr(fd: c_int, opt: c_int, termptr: *const Termios) -> c_int;
    }
    unwrap(unsafe { tcsetattr(0, 0, termios) }).and(Ok(()))
}

pub fn into_raw_mode() -> io::Result<()> {
    let mut ios = get_terminal_attr()?;
    let prev_ios = ios;

    unsafe {
        if ORIGINAL_TERMINAL_MODE.is_none() {
            ORIGINAL_TERMINAL_MODE = Some(prev_ios.clone());
        }

        RAW_MODE_ENABLED = true;
    }
    raw_terminal_attr(&mut ios);
    set_terminal_attr(&ios)?;
    Ok(())
}

pub fn disable_raw_mode() -> io::Result<()> {
    unsafe {
        if ORIGINAL_TERMINAL_MODE.is_some() {
            set_terminal_attr(&ORIGINAL_TERMINAL_MODE.unwrap())?;

            RAW_MODE_ENABLED = false;
        }
    }
    Ok(())
}
