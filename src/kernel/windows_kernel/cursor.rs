use winapi;
use kernel32;
use super::{handle, kernel};
use shared::functions;

/// These are the movement directions of an cursor
#[derive(Debug)]
pub enum CursorDirection {
    Top,
    Right,
    Down,
    Left,
}

/// Set the cursor position to an coordinate (x,y).
pub fn set(x: i16, y: i16) {
    set_cursor_pos(x as i16, y as i16);
}

/// Get the current cursor x position.
pub fn xpos() -> i16 {
    let csbi = kernel::get_console_screen_buffer_info();
    csbi.dwCursorPosition.X 
}

/// Get the current cursor y position.
pub fn ypos() -> i16 {
    let csbi = kernel::get_console_screen_buffer_info();
    csbi.dwCursorPosition.Y
}

pub fn move_down(count: u16) {
    let csbi = kernel::get_console_screen_buffer_info();
    unsafe {
        let output_handle = handle::get_output_handle();
        kernel32::SetConsoleCursorPosition(
            output_handle,
            winapi::COORD {
                X: csbi.dwCursorPosition.X,
                Y: csbi.dwCursorPosition.Y + count as i16,
            },
        );
    }
}

/// Set the cursor position to an coordinate (x,y).
fn set_cursor_pos(x: i16, y: i16) {
    functions::is_cursor_out_of_range(x, y);

    let output_handle = handle::get_output_handle();
    let position = winapi::COORD { X: x, Y: y };

    unsafe {
        let success = kernel32::SetConsoleCursorPosition(output_handle, position);

        if success == 0 {
            panic!("Cannot set console cursor position");
        }
    }
}
