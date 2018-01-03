mod no_color;
mod ansi_color;
mod winapi_color;

pub mod color;
pub mod base_color;

use self::no_color::NoTerminalColor;
use self::ansi_color::ANSIColor;
use self::winapi_color::WinApiColor;


