mod base_terminal;
mod terminal;

mod ansi_terminal;
mod winapi_terminal;

use self::ansi_terminal::AnsiTerminal;
use self::winapi_terminal::WinApiTerminal;

pub use self::base_terminal::ClearType;
pub use self::terminal::{ Terminal, terminal };