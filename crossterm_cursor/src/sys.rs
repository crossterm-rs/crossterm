#[cfg(unix)]
pub use self::unix::get_cursor_position;
#[cfg(unix)]
pub use self::unix::show_cursor;
#[cfg(windows)]
pub use self::winapi::get_cursor_position;
#[cfg(windows)]
pub use self::winapi::show_cursor;

#[cfg(unix)]
pub mod unix;

#[cfg(windows)]
pub mod winapi;
