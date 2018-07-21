//! This module contains all `unix` specific terminal related logic.

pub use self::libc::termios;
use self::libc::{c_int, c_ushort, ioctl, STDOUT_FILENO, TIOCGWINSZ};
use state::commands::{IStateCommand, NoncanonicalModeCommand};
use {libc, CommandManager, Context, StateManager};

use termios::{ Termios,cfmakeraw,tcsetattr,TCSADRAIN };
use std::io::Error;
use std::rc::Rc;
use std::{io, mem, fs};
use std::os::unix::io::AsRawFd;

/// A representation of the size of the current terminal.
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

/// Get the TTY device.
///
/// This allows for getting stdio representing _only_ the TTY, and not other streams.
pub fn get_tty() -> io::Result<fs::File> {
    fs::OpenOptions::new().read(true).write(true).open("/dev/tty")
}


pub fn read_char() -> io::Result<char>
{
    let mut buf = [0u8; 20];

    // get tty raw handle.
    let tty_f;

    let fd = unsafe
        {
            if libc::isatty(libc::STDIN_FILENO) == 1
                {
                    libc::STDIN_FILENO
                } else {
                tty_f = fs::File::open("/dev/tty")?;
                tty_f.as_raw_fd()
            }
        };


    let mut termios = Termios::from_fd(fd)?;
    let original = termios.clone();

    make_raw(&mut termios);
    tcsetattr(fd, TCSADRAIN, &termios)?;

    // read input and convert it to char
    let rv = unsafe {
        let read = libc::read(fd, buf.as_mut_ptr() as *mut libc::c_void, 20);

        if read < 0 {
            Err(io::Error::last_os_error())
        } else if buf[0] == b'\x03' {
            Err(io::Error::new(io::ErrorKind::Interrupted, "read interrupted"))
        } else {
            let mut pressed_char = Ok(' ');

            if let Ok(s) = ::std::str::from_utf8(&buf[..read as usize])
                {
                    if let Some(c) = s.chars().next()
                        {
                            pressed_char = Ok(c);
                        }
                }else {
                pressed_char = Err(io::Error::new(io::ErrorKind::Interrupted, "Could not parse char to utf8 char"));
            }

            pressed_char
        }
    };


    tcsetattr(fd, TCSADRAIN, &original)?;

    // if the user hit ^C we want to signal SIGINT to outselves.
    if let Err(ref err) = rv {
        if err.kind() == io::ErrorKind::Interrupted {
            unsafe { libc::raise(libc::SIGINT); }
        }
    }

    rv
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
