//! This module provides platform related functions.

#[cfg(unix)]
pub use self::unix::{query_color_scheme, query_terminal_colors};
#[cfg(windows)]
pub use self::windows::{query_color_scheme, query_terminal_colors};

#[cfg(unix)]
pub(crate) mod unix;
#[cfg(windows)]
pub(crate) mod windows;
