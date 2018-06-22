//! This module handles some logic for cursor interaction in the windows console.

use super::kernel;

/// This stores the cursor pos, at program level. So it can be recalled later.
static mut SAVED_CURSOR_POS:(u16,u16) = (0,0);

/// Reset to saved cursor position
pub fn reset_to_saved_position()
{
    unsafe {
        kernel::set_console_cursor_position(SAVED_CURSOR_POS.0  as i16, SAVED_CURSOR_POS.1  as i16);
    }
}

/// Save current cursor position to recall later.
pub fn save_cursor_pos()
{
    let position = pos();

    unsafe {
        SAVED_CURSOR_POS = (position.0, position.1);
    }
}

/// get the current cursor position.
pub fn pos() -> (u16,u16)
{
    let csbi = kernel::get_console_screen_buffer_info();
    ( csbi.dwCursorPosition.X as u16, csbi.dwCursorPosition.Y as u16 )
}