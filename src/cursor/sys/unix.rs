use std::io::{self, Write};

use crate::utils::{
    sys::unix::{disable_raw_mode, enable_raw_mode, is_raw_mode_enabled},
    Result,
};
use crate::{csi, write_cout, Event, EventPool};

/// Returns the cursor position (column, row).
///
/// The top left cell is represented `0,0`.
pub fn position() -> Result<(u16, u16)> {
    if is_raw_mode_enabled() {
        read_position_raw()
    } else {
        read_position()
    }
}

fn read_position() -> Result<(u16, u16)> {
    enable_raw_mode()?;
    let pos = read_position_raw();
    disable_raw_mode()?;
    pos
}

fn read_position_raw() -> Result<(u16, u16)> {
    // Use `ESC [ 6 n` to and retrieve the cursor position.
    let mut stdout = io::stdout();

    // Write command
    stdout.write_all(b"\x1B[6n")?;
    stdout.flush()?;

    // acquire mutable lock until we read the position, so that the user can't steal it from us.
    let mut lock = EventPool::get_mut();
    let mut pool = lock.pool();

    loop {
        match pool.poll(None).and_then(|_| pool.read()) {
            Ok(event) => {
                if let Event::CursorPosition(x, y) = event {
                    return Ok((x, y));
                }
            }
            Err(e) => Err(e),
        }
    }
}
