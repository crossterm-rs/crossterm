use std::io::{self, Write};

use crate::utils::{
    sys::unix::{disable_raw_mode, enable_raw_mode, is_raw_mode_enabled},
    Result,
};
use crate::{csi, write_cout};

pub(crate) fn get_cursor_position() -> Result<(u16, u16)> {
    if is_raw_mode_enabled() {
        pos_raw()
    } else {
        pos()
    }
}

pub(crate) fn show_cursor(show_cursor: bool) -> Result<()> {
    if show_cursor {
        write_cout!(csi!("?25h"))?;
    } else {
        write_cout!(csi!("?25l"))?;
    }
    Ok(())
}

fn pos() -> Result<(u16, u16)> {
    enable_raw_mode()?;
    let pos = pos_raw();
    disable_raw_mode()?;
    pos
}

fn pos_raw() -> Result<(u16, u16)> {
    // Where is the cursor?
    // Use `ESC [ 6 n`.
    let mut stdout = io::stdout();

    // Write command
    stdout.write_all(b"\x1B[6n")?;
    stdout.flush()?;

    loop {
        // TODO: READ MOUSE POSITION
        return Ok((0, 0));
    }
}
