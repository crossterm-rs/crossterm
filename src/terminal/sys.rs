//! This module provides platform related functions.

#[cfg(unix)]
#[cfg(feature = "event")]
pub use self::unix::supports_keyboard_enhancement;
#[cfg(unix)]
pub(crate) use self::unix::{disable_raw_mode, enable_raw_mode, is_raw_mode_enabled, size};
#[cfg(windows)]
#[cfg(feature = "event")]
pub use self::windows::supports_keyboard_enhancement;
#[cfg(windows)]
pub(crate) use self::windows::{
    clear, disable_raw_mode, enable_raw_mode, is_raw_mode_enabled, scroll_down, scroll_up,
    set_size, set_window_title, size,
};

#[cfg(windows)]
mod windows;

#[cfg(unix)]
mod unix;
