//! This module provides platform related functions.

#[cfg(unix)]
pub use self::unix::{exit, size};
#[cfg(windows)]
pub(crate) use self::windows::{clear, scroll_down, scroll_up, set_size};
#[cfg(windows)]
pub use self::windows::{exit, size};

#[cfg(windows)]
pub(crate) mod windows;

#[cfg(unix)]
pub(crate) mod unix;
