//! This module provides platform related functions.

#[cfg(unix)]
pub(crate) use self::unix::{disable_raw_mode, enable_raw_mode, exit, is_raw_mode_enabled, size};
#[cfg(windows)]
pub(crate) use self::windows::{
    clear, disable_raw_mode, enable_raw_mode, exit, scroll_down, scroll_up, set_size, size,
};

#[cfg(windows)]
mod windows;

#[cfg(unix)]
mod unix;
