#[cfg(unix)]
pub use self::unix::get_cursor_position;
#[cfg(unix)]
pub(crate) use self::unix::show_cursor;
#[cfg(windows)]
pub use self::windows::get_cursor_position;
#[cfg(windows)]
pub(crate) use self::windows::{
    goto, move_down, move_left, move_right, move_up, restore_position, save_position, show_cursor,
};

#[cfg(windows)]
pub(crate) mod windows;

#[cfg(unix)]
pub(crate) mod unix;
