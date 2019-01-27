use crossterm_utils::sys::unix;
use std::fs;
use std::io;
use std::os::unix::io::AsRawFd;

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

    let fd = unix::into_raw_mode()?;

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

    unix::disable_raw_mode();

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
