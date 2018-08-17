//! This module provides some modules to work with the terminal screen. Like raw and alternate screen.

pub mod alternate;
pub mod raw;

use super::{commands, functions};

pub use self::alternate::AlternateScreen;
pub use self::raw::RawScreen;
