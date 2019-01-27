use crossterm_utils::sys::unix;
use std::io::{self, Error, ErrorKind, Read, Write};

/// Get the cursor position based on the current platform.
#[cfg(unix)]
pub fn get_cursor_position() -> (u16, u16) {
    if let Ok(pos) = pos() {
        pos
    } else {
        (0, 0)
    }
}

pub fn pos() -> io::Result<(u16, u16)> {
    // if we enable raw modes with screen, this could cause problems if raw mode is already enabled in applicaition.
    // I am not completely happy with this approach so feel free to find an other way.

    unsafe {
        if !unix::RAW_MODE_ENABLED_BY_USER || !unix::RAW_MODE_ENABLED_BY_SYSTEM {
            // set this boolean so that we know that the systems has enabled raw mode.
            unix::RAW_MODE_ENABLED_BY_SYSTEM = true;
            unix::into_raw_mode()?;
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
        Ok(((cols - 1) as u16, (rows - 1) as u16))
    } else {
        return Err(Error::new(ErrorKind::Other, "test"));
    };

    // If raw mode is enabled from else where in the application (by the user) we do not want to disable raw modes.
    // I am not completely happy with this approach so feel free to find an other way.
    unsafe {
        if unix::RAW_MODE_ENABLED_BY_SYSTEM && !unix::RAW_MODE_ENABLED_BY_USER {
            unix::RAW_MODE_ENABLED_BY_SYSTEM = false;
            unix::disable_raw_mode()?;
        }
    }

    res
}
