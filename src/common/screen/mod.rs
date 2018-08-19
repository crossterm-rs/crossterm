//! This module provides some modules to work with the terminal screen. Like raw and alternate screen.

mod alternate;
mod raw;
mod screen;

use super::{commands, functions, TerminalOutput};

pub use self::alternate::AlternateScreen;
pub use self::screen::Screen;
pub use self::raw::RawScreen;
