#[cfg(unix)]
pub mod unix;

#[cfg(windows)]
pub mod winapi;

#[cfg(unix)]
pub use self::unix::get_cursor_position;
#[cfg(windows)]
pub use self::winapi::get_cursor_position;
