#[cfg(unix)]
pub(crate) use self::unix::get_cursor_position;
#[cfg(unix)]
pub(crate) use self::unix::show_cursor;
#[cfg(windows)]
pub(crate) use self::windows::get_cursor_position;
#[cfg(windows)]
pub(crate) use self::windows::show_cursor;

#[cfg(windows)]
pub(crate) mod windows;

#[cfg(unix)]
pub(crate) mod unix;
