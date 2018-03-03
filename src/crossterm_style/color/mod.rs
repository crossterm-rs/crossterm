pub mod base_color;
pub mod color;

mod ansi_color;

#[cfg(target_os = "windows")]
mod winapi_color;

use self::ansi_color::AnsiColor;

#[cfg(target_os = "windows")]
use self::winapi_color::WinApiColor;
