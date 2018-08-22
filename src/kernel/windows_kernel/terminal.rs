//! This module contains terminal specific logic.

use super::{csbi, handle, TerminalOutput};
use std::sync::Arc;

/// Get the terminal size
pub fn terminal_size() -> (u16, u16) {
    let handle = handle::get_output_handle().unwrap();

    if let Ok(csbi) = csbi::get_csbi_by_handle(&handle) {
        (
            (csbi.srWindow.Right - csbi.srWindow.Left) as u16,
            (csbi.srWindow.Bottom - csbi.srWindow.Top) as u16,
        )
    } else {
        return (0, 0);
    }
}

pub fn buffer_size() -> (u16, u16) {
    let handle = handle::get_output_handle().unwrap();

    if let Ok(csbi) = csbi::get_csbi_by_handle(&handle) {
        ((csbi.dwSize.X) as u16, (csbi.dwSize.Y) as u16)
    } else {
        return (0, 0);
    }
}

/// Exit the current process.
pub fn exit() {
    ::std::process::exit(256);
}
