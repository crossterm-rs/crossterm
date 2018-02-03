use super::kernel;

/// This stores the cursor pos, at program level. So it can be recalled later.
static  mut SAVED_CURSOR_POS:(i16,i16) = (0,0);

/// Set the current cursor position to X and Y
pub fn set(x: i16, y: i16)
{
    kernel::set_console_cursor_position(x, y );

}

/// Reset to saved cursor position
pub fn reset_to_saved_position()
{
    unsafe {
        kernel::set_console_cursor_position(SAVED_CURSOR_POS.0, SAVED_CURSOR_POS.1);
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

/// Get current cursor position (X,Y)
pub fn pos() -> (i16,i16)
{
    let csbi = kernel::get_console_screen_buffer_info();
    ( csbi.dwCursorPosition.X ,  csbi.dwCursorPosition.Y )
}