use std::io::{self, BufRead, Write};

use crossterm_utils::{
    csi,
    sys::unix::{self, RAW_MODE_ENABLED},
    write_cout, Result,
};

#[cfg(unix)]
pub fn get_cursor_position() -> Result<(u16, u16)> {
    if unsafe { RAW_MODE_ENABLED } {
        pos_raw()
    } else {
        pos()
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

pub fn pos() -> Result<(u16, u16)> {
    unix::into_raw_mode()?;
    let pos = pos_raw();
    unix::disable_raw_mode()?;
    pos
}

pub fn pos_raw() -> Result<(u16, u16)> {
    // Where is the cursor?
    // Use `ESC [ 6 n`.
    let mut stdout = io::stdout();
    let stdin = io::stdin();

    // Write command
    stdout.write_all(b"\x1B[6n")?;
    stdout.flush()?;

    stdin.lock().read_until(b'[', &mut vec![])?;

    let mut rows = vec![];
    stdin.lock().read_until(b';', &mut rows)?;

    let mut cols = vec![];
    stdin.lock().read_until(b'R', &mut cols)?;

    // remove delimiter
    rows.pop();
    cols.pop();

    let rows = String::from_utf8(rows)?.parse::<u16>()?;
    let cols = String::from_utf8(cols)?.parse::<u16>()?;

    Ok((cols - 1, rows - 1))
}
