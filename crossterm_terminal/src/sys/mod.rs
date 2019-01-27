#[cfg(windows)]
pub mod winapi;

#[cfg(unix)]
pub mod unix;

#[cfg(unix)]
pub use self::unix::{exit, get_terminal_size};
#[cfg(windows)]
pub use self::winapi::{exit, get_terminal_size};
