//! This module provides some modules to work with the terminal screen.
//! Like allowing you to switch between raw and alternate screen.

mod alternate;
mod raw;
mod screen;

use super::{commands, TerminalOutput};

pub use self::raw::RawScreen;
pub use self::alternate::AlternateScreen;
pub use self::screen::Screen;
