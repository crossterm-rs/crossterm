//! This module provides platform related functions.

#[cfg(unix)]
#[cfg(feature = "events")]
#[cfg(not(feature = "no-tty"))]
pub use self::unix::supports_keyboard_enhancement;
#[cfg(unix)]
#[cfg(not(feature = "no-tty"))]
pub(crate) use self::unix::{
    disable_raw_mode, enable_raw_mode, is_raw_mode_enabled, size, window_size,
};
#[cfg(all(windows, not(feature = "no-tty")))]
#[cfg(feature = "events")]
pub use self::windows::supports_keyboard_enhancement;
#[cfg(all(windows, test, not(feature = "no-tty")))]
pub(crate) use self::windows::temp_screen_buffer;
#[cfg(all(windows, not(feature = "no-tty")))]
pub(crate) use self::windows::{
    clear, disable_raw_mode, enable_raw_mode, is_raw_mode_enabled, scroll_down, scroll_up,
    set_size, set_window_title, size, window_size,
};

#[cfg(all(windows, not(feature = "no-tty")))]
mod windows;

#[cfg(unix)]
#[cfg(not(feature = "no-tty"))]
pub mod file_descriptor;
#[cfg(unix)]
#[cfg(not(feature = "no-tty"))]
mod unix;

#[cfg(unix)]
#[cfg(feature = "no-tty")]
pub(crate) use self::no_tty::{
    disable_raw_mode, enable_raw_mode, is_raw_mode_enabled, size, window_size,
};

#[cfg(unix)]
#[cfg(feature = "no-tty")]
mod no_tty;

#[cfg(unix)]
#[cfg(feature = "events")]
#[cfg(feature = "no-tty")]
pub use self::no_tty::supports_keyboard_enhancement;
