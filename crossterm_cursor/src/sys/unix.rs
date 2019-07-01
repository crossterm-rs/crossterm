use crossterm_utils::sys::unix::{self, RAW_MODE_ENABLED};
use std::io::{self, BufRead, Write};

/// Get the cursor position based on the current platform.
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

    let mut buf = vec![];
    stdin.lock().read_until(b'R', &mut buf)?;

    // message looks like `.*[{row};{col}R`

    // remove `R`
    buf.pop();

    // split to `.*[{row}` and `{col}`
    let mut v: Vec<Vec<u8>> = buf
        .split(|c| *c == ';' as u8)
        .map(ToOwned::to_owned)
        .collect();

    // get the `{col}` part
    let cols: Vec<u8> = v.pop().unwrap();

    // get the `{row}` part
    let rows: Vec<u8> = v
        .pop()
        .unwrap()
        .split(|c| *c == b'[')
        .last()
        .unwrap()
        .to_vec();;

    // parse {rows}
    let rows = rows
        .into_iter()
        .fold(String::new(), |mut acc, n| {
            acc.push(n as char);
            acc
        })
        .parse::<usize>()
        .unwrap();

    // parse {cols}
    let cols = cols
        .into_iter()
        .fold(String::new(), |mut acc, n| {
            acc.push(n as char);
            acc
        })
        .parse::<usize>()
        .unwrap();

    Ok(((cols - 1) as u16, (rows - 1) as u16))
}