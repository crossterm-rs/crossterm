mod base_terminal;
mod terminal;

#[cfg(unix)]
mod ansi_terminal;
#[cfg(windows)]
mod winapi_terminal;

#[cfg(unix)]
use self::ansi_terminal::UnixTerminal;
#[cfg(windows)]
use self::winapi_terminal::WinApiTerminal;

pub use self::base_terminal::ClearType;
pub use self::terminal::{ Terminal, terminal };