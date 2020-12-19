use crossterm_winapi::{ConsoleMode, Handle};
use winapi::um::wincon::ENABLE_VIRTUAL_TERMINAL_PROCESSING;

use lazy_static::lazy_static;

use crate::Result;

/// Enable virtual terminal processing.
///
/// This method attempts to enable virtual terminal processing for this
/// console. If there was a problem enabling it, then an error returned.
/// On success, the caller may assume that enabling it was successful.
///
/// When virtual terminal processing is enabled, characters emitted to the
/// console are parsed for VT100 and similar control character sequences
/// that control color and other similar operations.
fn enable_vt_processing() -> Result<()> {
    let mask = ENABLE_VIRTUAL_TERMINAL_PROCESSING;

    let console_mode = ConsoleMode::from(Handle::current_out_handle()?);
    let old_mode = console_mode.mode()?;

    if old_mode & mask == 0 {
        console_mode.set_mode(old_mode | mask)?;
    }

    Ok(())
}

lazy_static! {
    static ref SUPPORTS_ANSI_ESCAPE_CODES: bool = {
        std::env::var("TERM").map_or(false, |term| term != "dumb") || enable_vt_processing().is_ok()
    };
}

/// Checks if the current terminal supports ansi escape sequences
pub fn supports_ansi() -> bool {
    *SUPPORTS_ANSI_ESCAPE_CODES
}
