#[cfg(unix)]
pub(crate) use self::unix::{exit, get_terminal_size};
#[cfg(windows)]
pub(crate) use self::winapi::{exit, get_terminal_size};

#[cfg(windows)]
pub(crate) mod winapi;

#[cfg(unix)]
pub(crate) mod unix;
