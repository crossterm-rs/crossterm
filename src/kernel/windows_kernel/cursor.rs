//! This module handles some logic for cursor interaction in the windows console.

use winapi::shared::minwindef::{FALSE, TRUE};
use winapi::um::wincon::{
    SetConsoleCursorInfo, SetConsoleCursorPosition, CONSOLE_CURSOR_INFO, COORD,
};

use super::super::super::manager::{ScreenManager, WinApiScreenManager};
use super::{csbi, handle, kernel};

use std::io::{self, ErrorKind, Result};

/// This stores the cursor pos, at program level. So it can be recalled later.
static mut SAVED_CURSOR_POS: (u16, u16) = (0, 0);

/// Reset to saved cursor position
pub fn reset_to_saved_position(screen_manager: &ScreenManager) {
    unsafe {
        set_console_cursor_position(
            SAVED_CURSOR_POS.0 as i16,
            SAVED_CURSOR_POS.1 as i16,
            screen_manager,
        );
    }
}

/// Save current cursor position to recall later.
pub fn save_cursor_pos(screen_manager: &ScreenManager) {
    let position = pos(screen_manager);

    unsafe {
        SAVED_CURSOR_POS = (position.0, position.1);
    }
}

/// get the current cursor position.
pub fn pos(screen_manager: &ScreenManager) -> (u16, u16) {

    let handle = handle::get_current_handle(screen_manager).unwrap();

    if let Ok(csbi) = csbi::get_csbi_by_handle(&handle) {
        ( csbi.dwCursorPosition.X as u16, csbi.dwCursorPosition.Y as u16 )
    } else {
        (0, 0)
    }
}


/// Set the cursor position to the given x and y. Note that this is 0 based.
pub fn set_console_cursor_position(x: i16, y: i16, screen_manager: &ScreenManager) {
    if x < 0 || x >= <i16>::max_value() {
        panic!(
            "Argument Out of Range Exception when setting cursor position to X: {}",
            x
        );
    }

    if y < 0 || y >= <i16>::max_value() {
        panic!(
            "Argument Out of Range Exception when setting cursor position to Y: {}",
            y
        );
    }

    let handle = handle::get_current_handle(screen_manager).unwrap();

    let position = COORD { X: x, Y: y };

    unsafe {
        let success = SetConsoleCursorPosition(handle, position);

        if success == 0 {
            panic!("Argument out of range when trying to set cursor position.");
        }
    }
}

/// change the cursor visibility.
pub fn cursor_visibility(visable: bool, screen_manager: &ScreenManager) -> Result<()> {
    let handle = handle::get_current_handle(screen_manager).unwrap();

    let cursor_info = CONSOLE_CURSOR_INFO {
        dwSize: 100,
        bVisible: if visable { TRUE } else { FALSE },
    };

    unsafe {
        if !kernel::is_true(SetConsoleCursorInfo(handle, &cursor_info)) {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Could not get console screen buffer info",
            ));
        }
    }
    Ok(())
}
