use std::io::{self, BufRead, Write};

use crossterm_utils::{
    sys::unix::{self, RAW_MODE_ENABLED},
    Result,
};

#[cfg(unix)]
pub fn get_cursor_position() -> (u16, u16) {
    if unsafe { RAW_MODE_ENABLED } {
        if let Ok(pos) = pos_raw() {
            pos
        } else {
            (0, 0)
        }
    } else {
        if let Ok(pos) = pos() {
            pos
        } else {
            (0, 0)
        }
    }
}

#[cfg(unix)]
pub fn show_cursor(show_cursor: bool) -> Result<()> {
    if show_cursor {
        write_cout!(csi!("?25h"))?;
    } else {
        write_cout!(csi!("?25l"))?;
    }
    Ok(())
}

pub fn pos() -> io::Result<(u16, u16)> {
    unix::into_raw_mode()?;
    let pos = pos_raw();
    unix::disable_raw_mode()?;
    pos
}

pub fn pos_raw() -> io::Result<(u16, u16)> {
    // Where is the cursor?
    // Use `ESC [ 6 n`.
    let mut stdout = io::stdout();
    let stdin = io::stdin();

    // Write command
    stdout.write_all(b"\x1B[6n")?;
    stdout.flush()?;

    stdin.lock().read_until(b'[', &mut vec![])?;

    let mut rows = vec![];
    stdin.lock().read_until(b';', &mut rows).unwrap();

    let mut cols = vec![];
    stdin.lock().read_until(b'R', &mut cols).unwrap();

    // remove delimiter
    rows.pop();
    cols.pop();

    let rows = rows
        .into_iter()
        .map(|b| (b as char))
        .fold(String::new(), |mut acc, n| {
            acc.push(n);
            acc
        })
        .parse::<usize>()
        .unwrap();
    let cols = cols
        .into_iter()
        .map(|b| (b as char))
        .fold(String::new(), |mut acc, n| {
            acc.push(n);
            acc
        })
        .parse::<usize>()
        .unwrap();

    Ok(((cols - 1) as u16, (rows - 1) as u16))
}
