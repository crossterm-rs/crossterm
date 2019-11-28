//! This module provides platform related functions.

#[cfg(unix)]
pub(crate) use self::unix::{disable_raw_mode, enable_raw_mode, exit, size};
#[cfg(windows)]
pub(crate) use self::windows::{
    clear, disable_raw_mode, enable_raw_mode, exit, scroll_down, scroll_up, set_size, size,
};

#[cfg(windows)]
pub(crate) mod windows;

#[cfg(unix)]
pub(crate) mod unix;
