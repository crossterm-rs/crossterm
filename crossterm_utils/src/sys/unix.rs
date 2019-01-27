//! This module contains all `unix` specific terminal related logic.

use libc::{self, c_ushort, ioctl, termios, STDOUT_FILENO, TCSADRAIN, TIOCGWINSZ};

use crate::termios::{tcsetattr, Termios};
use std::fs;
use std::io::{self, Error, ErrorKind, Read, Write};
use std::os::unix::io::{AsRawFd, RawFd};

static mut ORIGINAL_TERMINAL_MODE: Option<Termios> = None;
pub static mut RAW_MODE_ENABLED_BY_SYSTEM: bool = false;
pub static mut RAW_MODE_ENABLED_BY_USER: bool = false;

/// Transform the given mode into an raw mode (non-canonical) mode.
pub fn make_raw(termios: &mut Termios) {
    extern "C" {
        pub fn cfmakeraw(termptr: *mut Termios);
    }
    unsafe { cfmakeraw(termios) }
}

pub fn into_raw_mode() -> io::Result<RawFd> {
    let tty_f;

    let fd = unsafe {
        if libc::isatty(libc::STDIN_FILENO) == 1 {
            libc::STDIN_FILENO
        } else {
            tty_f = fs::File::open("/dev/tty")?;
            tty_f.as_raw_fd()
        }
    };

    let mut termios = Termios::from_fd(fd)?;
    let original = termios.clone();

    unsafe {
        if ORIGINAL_TERMINAL_MODE.is_none() {
            ORIGINAL_TERMINAL_MODE = Some(original.clone())
        }
    }

    make_raw(&mut termios);
    tcsetattr(fd, TCSADRAIN, &termios)?;

    Ok(fd)
}

pub fn disable_raw_mode() -> io::Result<()> {
    let tty_f;

    let fd = unsafe {
        if libc::isatty(libc::STDIN_FILENO) == 1 {
            libc::STDIN_FILENO
        } else {
            tty_f = fs::File::open("/dev/tty")?;
            tty_f.as_raw_fd()
        }
    };

    if let Some(original) = unsafe { ORIGINAL_TERMINAL_MODE } {
        tcsetattr(fd, TCSADRAIN, &original)?;
    }
    Ok(())
}
