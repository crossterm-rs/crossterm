mod base_cursor;
mod cursor;

#[cfg(unix)]
mod ansi_cursor;
#[cfg(windows)]
mod winapi_cursor;

#[cfg(unix)]
use self::ansi_cursor::AnsiCursor;
#[cfg(windows)]
use self::winapi_cursor::WinApiCursor;

pub use self::cursor::{ cursor, TerminalCursor };

