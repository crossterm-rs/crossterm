//! This module contains all `unix` specific terminal related logic.

pub use self::libc::termios;
use self::libc::{c_int, c_ushort, ioctl, STDOUT_FILENO, TIOCGWINSZ};
use state::commands::{IStateCommand, NoncanonicalModeCommand};
use termios::Termios;
use {libc, CommandManager, Context, StateManager};

use std::io::Error;
use std::rc::Rc;
use std::{io, mem};

//! A representation of the size of the current terminal.

#[repr(C)]
#[derive(Debug)]
pub struct UnixSize {
    /// number of rows
    pub rows: c_ushort,
    /// number of columns
    pub cols: c_ushort,
    x: c_ushort,
    y: c_ushort,
}

/// Get the current terminal size.
pub fn terminal_size() -> (u16, u16) {
    // http://rosettacode.org/wiki/Terminal_control/Dimensions#Library:_BSD_libc
    let us = UnixSize {
        rows: 0,
        cols: 0,
        x: 0,
        y: 0,
    };
    let r = unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &us) };
    if r == 0 {
        // because crossterm works starts counting at 0 and unix terminal starts at cell 1 you have subtract one to get 0-based results.
        (us.cols - 1, us.rows - 1)
    } else {
        (0, 0)
    }
}

/// Get the current cursor position.
pub fn pos(context: Rc<Context>) -> (u16, u16) {
    use std::io::{Read, Write};

    let mut command_id = NoncanonicalModeCommand::new(&context.state_manager);

    CommandManager::execute(context.clone(), command_id);

    // This code is original written by term_cursor credits to them.
    use std::io;
    let mut std = io::stdout();
    // Write command
    std.write(b"\x1B[6n");
    std.flush();

    // Read back result
    let mut buf = [0u8; 2];
    // Expect `ESC[`
    io::stdin().read_exact(&mut buf);
    if buf[0] != 0x1B || buf[1] as char != '[' {
        return (0, 0);
    }

    // Read rows and cols through a ad-hoc integer parsing function
    let read_num = || -> (i32, char) {
        let mut num = 0;
        let mut c;

        loop {
            let mut buf = [0u8; 1];
            io::stdin().read_exact(&mut buf);
            c = buf[0] as char;
            if let Some(d) = c.to_digit(10) {
                num = if num == 0 { 0 } else { num * 10 };
                num += d as i32;
            } else {
                break;
            }
        }

        (num, c)
    };

    // Read rows and expect `;`
    let (rows, c) = read_num();

    if c != ';' {
        return (0, 0);
    }

    // Read cols
    let (cols, c) = read_num();

    // Expect `R`
    let res = if c == 'R' {
        (cols as u16, rows as u16)
    } else {
        return (0, 0);
    };

    CommandManager::undo(context.clone(), command_id);

    res
}

/// Set the terminal mode to the given mode.
pub fn set_terminal_mode(termios: &Termios) -> io::Result<()> {
    extern "C" {
        pub fn tcsetattr(fd: c_int, opt: c_int, termptr: *const Termios) -> c_int;
    }
    is_true(unsafe { tcsetattr(0, 0, termios) }).and(Ok(()))
}

/// Transform the given mode into an raw mode (non-canonical) mode.
pub fn make_raw(termios: &mut Termios) {
    extern "C" {
        pub fn cfmakeraw(termptr: *mut Termios);
    }
    unsafe { cfmakeraw(termios) }
}

/// Get the current terminal mode.
pub fn get_terminal_mode() -> io::Result<Termios> {
    extern "C" {
        pub fn tcgetattr(fd: c_int, termptr: *mut Termios) -> c_int;
    }
    unsafe {
        let mut termios = mem::zeroed();
        is_true(tcgetattr(0, &mut termios))?;
        Ok(termios)
    }
}

pub fn exit() {
    ::std::process::exit(0);
}

/// Is the return value true?
fn is_true(value: i32) -> Result<(), Error> {
    match value {
        -1 => Err(io::Error::last_os_error()),
        0 => Ok(()),
        _ => Err(io::Error::last_os_error()),
    }
}
