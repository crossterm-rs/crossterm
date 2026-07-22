use std::{
    io::{self, Error, ErrorKind},
    time::Duration,
};

use crate::event::{
    filter::CursorPositionFilter, internal::InternalEvent, internal_no_tty::NoTtyEvent,
};

use bytes::Bytes;

/// Returns the cursor position (column, row).
///
/// The top left cell is represented as `(0, 0)`.
///
/// This sends a cursor-position query out the event handle's query channel and awaits
/// the response. It returns an error with kind [`io::ErrorKind::BrokenPipe`] if the
/// input channel disconnects while waiting.
pub async fn position(event: &NoTtyEvent) -> io::Result<(u16, u16)> {
    // Use `ESC [ 6 n` to request the cursor position.
    event
        .send
        .send_timeout(Bytes::from_static(b"\x1B[6n"), Duration::from_secs(1))
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    loop {
        match event
            .poll(Some(Duration::from_millis(2000)), &CursorPositionFilter)
            .await
        {
            Ok(true) => {
                if let Ok(InternalEvent::CursorPosition(x, y)) =
                    event.read(&CursorPositionFilter).await
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
            // The input channel disconnected; propagate so the caller can stop.
            Err(e) if e.kind() == io::ErrorKind::BrokenPipe => {
                return Err(e);
            }
            Err(_) => {}
        }
    }
}
