mod base_terminal;
mod no_terminal;
mod unix_terminal;
mod winapi_terminal;
pub mod terminal;

pub use self::terminal::{get, Terminal};
pub use self::base_terminal::ClearType;

use self::unix_terminal::UnixTerminal;
use self::winapi_terminal::WinApiTerminal;
use self::no_terminal::NoTerminal;
