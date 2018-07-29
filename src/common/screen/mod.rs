//! This module provides some modules to work with the terminal screen. Like raw and alternate screen.

mod raw;
mod alternate;

use super::{ScreenManager, functions, commands};

pub use self::raw::RawScreen;
pub use self::alternate::AlternateScreen;