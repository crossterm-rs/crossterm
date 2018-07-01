//! This module handles some logic for cursor interaction in the windows console.

use super::kernel;
use super::super::super::manager::{ScreenManager, WinApiScreenManager};

use std::rc::Rc;
use std::sync::Mutex;

/// This stores the cursor pos, at program level. So it can be recalled later.
static mut SAVED_CURSOR_POS:(u16,u16) = (0,0);

/// Reset to saved cursor position
pub fn reset_to_saved_position(screen_manager: &Rc<Mutex<ScreenManager>>)
{
    unsafe {
        kernel::set_console_cursor_position(SAVED_CURSOR_POS.0  as i16, SAVED_CURSOR_POS.1  as i16, screen_manager);
    }
}

/// Save current cursor position to recall later.
pub fn save_cursor_pos(screen_manager: &Rc<Mutex<ScreenManager>>)
{
    let position = pos(screen_manager);

    unsafe {
        SAVED_CURSOR_POS = (position.0, position.1);
    }
}

/// get the current cursor position.
pub fn pos(screen_manager: &Rc<Mutex<ScreenManager>>) -> (u16,u16)
{
    let csbi = kernel::get_console_screen_buffer_info(screen_manager);
    ( csbi.dwCursorPosition.X as u16, csbi.dwCursorPosition.Y as u16 )
}