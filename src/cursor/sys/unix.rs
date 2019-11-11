use std::{
    collections::VecDeque,
    io::{self, Error, ErrorKind, Write},
    time::Duration,
};

use crate::{
    event::{enqueue_internal, poll_internal, read_internal, InternalEvent},
    mask::CursorEventMask,
    utils::{
        sys::unix::{disable_raw_mode, enable_raw_mode, is_raw_mode_enabled},
        Result,
    },
};

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
    stdout.write_all(b"\x1B[6n")?;
    stdout.flush()?;

    loop {
        let mut temp_buffer = VecDeque::new();

        match poll_internal(Some(Duration::from_millis(2000))) {
            Ok(true) => {
                match read_internal(CursorEventMask) {
                    Ok(InternalEvent::CursorPosition(x, y)) => {
                        if !temp_buffer.is_empty() {
                            while let Some(event) = temp_buffer.pop_front() {
                                enqueue_internal(event);
                            }
                        }
                        return Ok((x, y));
                    }
                    Ok(event) => {
                        // We can not write events directly back to the reader.
                        // If we did we would, we would put put our self's into an recursive call,
                        // by enqueueing and popping the same event again and again.
                        // Therefore, store them into the temporary buffer,
                        // and enqueue the events back when we read the cursor position.
                        temp_buffer.push_back(event);
                    }
                    Err(_) => {}
                };
            }
            Ok(false) => {
                return Err(Error::new(
                    ErrorKind::Other,
                    "The cursor position could not be read within a normal duration",
                ))?;
            }
            Err(_) => {}
        }
    }
}
