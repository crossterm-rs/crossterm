use std::{
    fs::OpenOptions,
    io::{self, Error, ErrorKind, Write},
    time::Duration,
};

use crate::{
    event::{filter::CursorPositionFilter, poll_internal, read_internal, InternalEvent},
    terminal::{disable_raw_mode, enable_raw_mode, sys::is_raw_mode_enabled},
};

#[cfg(feature = "libc")]
fn fd_is_tty(fd: libc::c_int) -> bool {
    unsafe { libc::isatty(fd) == 1 }
}

#[cfg(not(feature = "libc"))]
fn fd_is_tty<F: rustix::fd::AsFd>(fd: F) -> bool {
    rustix::termios::isatty(fd)
}

/// Returns the cursor position (column, row).
///
/// The top left cell is represented as `(0, 0)`.
///
/// On unix systems, this function will block and possibly time out while
/// [`crossterm::event::read`](crate::event::read) or [`crossterm::event::poll`](crate::event::poll) are being called.
pub fn position() -> io::Result<(u16, u16)> {
    if is_raw_mode_enabled() {
        read_position_raw()
    } else {
        read_position()
    }
}

fn read_position() -> io::Result<(u16, u16)> {
    enable_raw_mode()?;
    let pos = read_position_raw();
    disable_raw_mode()?;
    pos
}

// `ESC [ 6 n` device status report: ask terminal for cursor position.
// Must reach the controlling terminal even when stdout/stderr are redirected,
// otherwise the query is piped to the redirect target (see issue #652).
fn write_cursor_query(buf: &[u8]) -> io::Result<()> {
    #[cfg(feature = "libc")]
    let (stdout_is_tty, stderr_is_tty) = (
        fd_is_tty(libc::STDOUT_FILENO),
        fd_is_tty(libc::STDERR_FILENO),
    );
    #[cfg(not(feature = "libc"))]
    let (stdout_is_tty, stderr_is_tty) = (
        fd_is_tty(rustix::stdio::stdout()),
        fd_is_tty(rustix::stdio::stderr()),
    );

    if stdout_is_tty {
        let stdout = io::stdout();
        let mut out = stdout.lock();
        out.write_all(buf)?;
        out.flush()?;
        return Ok(());
    }

    if stderr_is_tty {
        let stderr = io::stderr();
        let mut out = stderr.lock();
        out.write_all(buf)?;
        out.flush()?;
        return Ok(());
    }

    let mut tty = OpenOptions::new().write(true).open("/dev/tty")?;
    tty.write_all(buf)?;
    tty.flush()?;
    Ok(())
}

fn read_position_raw() -> io::Result<(u16, u16)> {
    write_cursor_query(b"\x1B[6n")?;

    loop {
        match poll_internal(Some(Duration::from_millis(2000)), &CursorPositionFilter) {
            Ok(true) => {
                if let Ok(InternalEvent::CursorPosition(x, y)) =
                    read_internal(&CursorPositionFilter)
                {
                    return Ok((x, y));
                }
            }
            Ok(false) => {
                return Err(Error::new(
                    ErrorKind::Other,
                    "The cursor position could not be read within a normal duration",
                ));
            }
            Err(_) => {}
        }
    }
}
