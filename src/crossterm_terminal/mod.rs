mod raw_terminal;
mod base_terminal;
mod terminal;

pub mod screen;

mod ansi_terminal;

#[cfg(target_os = "windows")]
mod winapi_terminal;

use self::ansi_terminal::AnsiTerminal;

#[cfg(target_os = "windows")]
use self::winapi_terminal::WinApiTerminal;

pub use self::base_terminal::ClearType;
pub use self::terminal::{ Terminal, terminal};
pub use self::raw_terminal::{RawTerminal, IntoRawMode};