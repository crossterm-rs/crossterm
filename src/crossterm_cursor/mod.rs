mod base_cursor;
mod cursor;

mod ansi_cursor;

#[cfg(target_os = "windows")]
mod winapi_cursor;

use self::ansi_cursor::AnsiCursor;

#[cfg(target_os = "windows")]
use self::winapi_cursor::WinApiCursor;

pub use self::cursor::{ cursor, TerminalCursor };

