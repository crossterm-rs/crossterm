//! This module contains all `unix` specific terminal related logic.

pub use self::libc::termios;
use self::libc::{c_int, c_ushort, ioctl, STDOUT_FILENO, TIOCGWINSZ};
use state::commands::{IStateCommand, NoncanonicalModeCommand, EnableRawModeCommand};
use {libc, CommandManager, Context, StateManager};

use std::io::Error;
use std::os::unix::io::AsRawFd;
use std::rc::Rc;
use std::{fs, io, mem};
use termios::{cfmakeraw, tcsetattr, Termios, TCSADRAIN};

/// A representation of the size of the current terminal.
#[repr(C)]
#[derive(Debug)]
pub struct UnixSize {
    /// number of rows
    pub rows: c_ushort,
    /// number of columns
    pub cols: c_ushort,
    pub ws_xpixel: c_ushort,
    pub ws_ypixel: c_ushort,
}

/// Get the current terminal size.
pub fn terminal_size() -> (u16, u16) {
    // http://rosettacode.org/wiki/Terminal_control/Dimensions#Library:_BSD_libc
    let us = UnixSize {
        rows: 0,
        cols: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    let r = unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &us) };
    if r == 0 {
        // because crossterm works starts counting at 0 and unix terminal starts at cell 1 you have subtract one to get 0-based results.
        (us.cols - 1, us.rows - 1)

    } else {
        (0, 0)
    }
}


use std::time::{SystemTime, Duration};
use std::io::ErrorKind;
use Terminal;
use std::io::Read;
/// Get the current cursor position.
pub fn pos() -> (u16, u16) {
    let crossterm = Terminal::new();
    let input = crossterm.input();

    let delimiter = b'R';
    let mut stdin = input.read_until_async(delimiter);

    // Where is the cursor?
    // Use `ESC [ 6 n`.

//    crossterm.write("\x1B[6n");

    let mut buf: [u8; 1] = [0];
    let mut read_chars = Vec::new();

    let timeout = Duration::from_millis(2000);
    let now = SystemTime::now();

    // Either consume all data up to R or wait for a timeout.
    while buf[0] != delimiter && now.elapsed().unwrap() < timeout {
        if let Ok(c) = stdin.read(&mut buf){
            if c >= 0
            {
                read_chars.push(buf[0]);
            }
        }
    }

    if read_chars.len() == 0 {
        return (0, 0);
    }

    // The answer will look like `ESC [ Cy ; Cx R`.

    read_chars.pop(); // remove trailing R.
    let read_str = String::from_utf8(read_chars).unwrap();
    let beg = read_str.rfind('[').unwrap();
    let coords: String = read_str.chars().skip(beg + 1).collect();
    let mut nums = coords.split(';');

    let cy = nums.next()
        .unwrap()
        .parse::<u16>()
        .unwrap();
    let cx = nums.next()
        .unwrap()
        .parse::<u16>()
        .unwrap();

    (cx, cy)
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
    fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/tty")
}

pub fn read_char() -> io::Result<char> {
    let mut buf = [0u8; 20];

    // get tty raw handle.
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

    make_raw(&mut termios);
    tcsetattr(fd, TCSADRAIN, &termios)?;

    // read input and convert it to char
    let rv = unsafe {
        let read = libc::read(fd, buf.as_mut_ptr() as *mut libc::c_void, 20);

        if read < 0 {
            Err(io::Error::last_os_error())
        } else if buf[0] == b'\x03' {
            Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "read interrupted",
            ))
        } else {
            let mut pressed_char = Ok(' ');

            if let Ok(s) = ::std::str::from_utf8(&buf[..read as usize]) {
                if let Some(c) = s.chars().next() {
                    pressed_char = Ok(c);
                }
            } else {
                pressed_char = Err(io::Error::new(
                    io::ErrorKind::Interrupted,
                    "Could not parse char to utf8 char",
                ));
            }

            pressed_char
        }
    };

    tcsetattr(fd, TCSADRAIN, &original)?;

    // if the user hit ^C we want to signal SIGINT to outselves.
    if let Err(ref err) = rv {
        if err.kind() == io::ErrorKind::Interrupted {
            unsafe {
                libc::raise(libc::SIGINT);
            }
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
