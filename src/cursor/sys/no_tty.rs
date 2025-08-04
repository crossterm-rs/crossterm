use std::{
    io::{self, Error, ErrorKind},
    time::Duration,
};

use crate::event::{
    filter::CursorPositionFilter, internal::InternalEvent, internal_no_tty::NoTtyEvent,
};

/// Returns the cursor position (column, row).
///
/// The top left cell is represented as `(0, 0)`.
///
/// On unix systems, this function will block and possibly time out while
/// [`crossterm::event::read`](crate::event::read) or [`crossterm::event::poll`](crate::event::poll) are being called.
pub fn position(event: &NoTtyEvent) -> io::Result<(u16, u16)> {
    // Use `ESC [ 6 n` to and retrieve the cursor position.
    event
        .send
        .send_timeout(b"\x1B[6n".into(), Duration::from_secs(1))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    loop {
        match event.poll(Some(Duration::from_millis(2000)), &CursorPositionFilter) {
            Ok(true) => {
                if let Ok(InternalEvent::CursorPosition(x, y)) = event.read(&CursorPositionFilter) {
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
