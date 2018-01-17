pub mod base_color;
pub mod color;

#[cfg(unix)]
mod ansi_color;
#[cfg(windows)]
mod winapi_color;

#[cfg(unix)]
use self::ansi_color::ANSIColor;
#[cfg(windows)]
use self::winapi_color::WinApiColor;
