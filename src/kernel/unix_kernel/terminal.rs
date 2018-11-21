//! This module contains all `unix` specific terminal related logic.

use self::libc::{c_ushort, ioctl, STDOUT_FILENO, TIOCGWINSZ, TCSADRAIN};

use {libc, Screen};
pub use libc::termios;

use std::io::{self, Error, ErrorKind, Read, Write};
use std::os::unix::io::AsRawFd;
use std::fs;
use termios::{tcsetattr, Termios};

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
        (us.cols, us.rows)
    } else {
        (0,0)
    }
}

// maybe this coudl be used over ANSI escape code
//pub fn set_terminal_size() -> io::Result<(u16,u16)>
//{
//    let new_size = UnixSize {
//        rows: 40,
//        cols: 40,
//        ws_xpixel: 0,
//        ws_ypixel: 0,
//    };
//
//    let r = unsafe { ioctl(STDOUT_FILENO, TIOCSWINSZ, &new_size) };
//
//    if r == 0 {
//        // because crossterm works starts counting at 0 and unix terminal starts at cell 1 you have subtract one to get 0-based results.
//        (us.cols, us.rows)
//    } else {
//        Err(ErrorKind::Other, "Could not resize try ansi escape codes")
//        (0, 0)
//    }
//}

pub fn pos() -> io::Result<(u16, u16)>
{
    let _screen = Screen::new(false);

    // if we enable raw modes with screen, this could cause problems if raw mode is already enabled in applicaition.
    // I am not completely happy with this approach so feel free to find an other way.

    unsafe {
        if !RAW_MODE_ENABLED_BY_USER || !RAW_MODE_ENABLED_BY_SYSTEM {
            // set this boolean so that we know that the systems has enabled raw mode.
            RAW_MODE_ENABLED_BY_SYSTEM = true;
            into_raw_mode()?;
        }
    }

    // Where is the cursor?
    // Use `ESC [ 6 n`.
    let mut stdout = io::stdout();

    // Write command
    stdout.write_all(b"\x1B[6n")?;
    stdout.flush()?;

    let mut buf = [0u8; 2];

    // Expect `ESC[`
    io::stdin().read_exact(&mut buf)?;
    if buf[0] != 0x1B || buf[1] as char != '[' {
        return Err(Error::new(ErrorKind::Other, "test"));
    }

    // Read rows and cols through a ad-hoc integer parsing function
    let read_num: fn() -> Result<(i32, char), Error> = || -> Result<(i32, char), Error> {
        let mut num = 0;
        let mut c: char;

        loop {
            let mut buf = [0u8; 1];
            io::stdin().read_exact(&mut buf)?;
            c = buf[0] as char;
            if let Some(d) = c.to_digit(10) {
                num = if num == 0 { 0 } else { num * 10 };
                num += d as i32;
            } else {
                break;
            }
        }

        Ok((num, c))
    };

    // Read rows and expect `;`
    let (rows, c) = read_num()?;
    if c != ';' {
        return Err(Error::new(ErrorKind::Other, "test"));
    }

    // Read cols
    let (cols, c) = read_num()?;

    // Expect `R`
    let res = if c == 'R' {
        Ok(((cols -1) as u16, (rows -1) as u16))
    } else {
        return Err(Error::new(ErrorKind::Other, "test"));
    };

    // If raw mode is enabled from else where in the application (by the user) we do not want to disable raw modes.
    // I am not completely happy with this approach so feel free to find an other way.
    unsafe {
        if RAW_MODE_ENABLED_BY_SYSTEM && !RAW_MODE_ENABLED_BY_USER {
            RAW_MODE_ENABLED_BY_SYSTEM = false;
            disable_raw_mode()?;
        }
    }

    res
}

static mut ORIGINAL_TERMINAL_MODE: Option<Termios> = None;
static mut RAW_MODE_ENABLED_BY_SYSTEM: bool = false;
pub static mut RAW_MODE_ENABLED_BY_USER: bool = false;

/// Transform the given mode into an raw mode (non-canonical) mode.
pub fn make_raw(termios: &mut Termios) {
    extern "C" {
        pub fn cfmakeraw(termptr: *mut Termios);
    }
    unsafe { cfmakeraw(termios) }
}

pub fn into_raw_mode() -> io::Result<()>
{
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
        if ORIGINAL_TERMINAL_MODE.is_none()
        {
            ORIGINAL_TERMINAL_MODE = Some(original.clone())
        }
    }

    make_raw(&mut termios);
    tcsetattr(fd, TCSADRAIN, &termios)?;


    Ok(())
}

pub fn disable_raw_mode() -> io::Result<()>
{
    let tty_f;

    let fd = unsafe {
        if libc::isatty(libc::STDIN_FILENO) == 1 {
            libc::STDIN_FILENO
        } else {
            tty_f = fs::File::open("/dev/tty")?;
            tty_f.as_raw_fd()
        }
    };

    if let Some(original) = unsafe { ORIGINAL_TERMINAL_MODE }
    {
        tcsetattr(fd, TCSADRAIN, &original)?;
    }
    Ok(())
}

/// Get the TTY device.
///
/// This allows for getting stdio representing _only_ the TTY, and not other streams.
pub fn get_tty() -> io::Result<fs::File> {
    let mut tty_f: fs::File = unsafe { ::std::mem::zeroed() };

    let _fd = unsafe {
        if libc::isatty(libc::STDIN_FILENO) == 1 {
            libc::STDIN_FILENO
        } else {
            tty_f = fs::File::open("/dev/tty")?;
            tty_f.as_raw_fd()
        }
    };

    Ok(tty_f)
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
    let original = termios;

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
