#[cfg(unix)]
pub use self::unix::{exit, get_terminal_size};
#[cfg(windows)]
pub(crate) use self::winapi::{clear, scroll_down, scroll_up, set_size};
#[cfg(windows)]
pub use self::winapi::{exit, get_terminal_size};

#[cfg(windows)]
pub(crate) mod winapi;

#[cfg(unix)]
pub(crate) mod unix;
