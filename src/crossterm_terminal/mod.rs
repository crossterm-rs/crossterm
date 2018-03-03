mod raw_terminal;
mod base_terminal;
mod terminal;

pub mod screen;

mod ansi_terminal;
mod winapi_terminal;

use self::ansi_terminal::AnsiTerminal;
use self::winapi_terminal::WinApiTerminal;

pub use self::base_terminal::ClearType;
pub use self::terminal::{ Terminal, terminal};
pub use self::raw_terminal::{RawTerminal, IntoRawMode};