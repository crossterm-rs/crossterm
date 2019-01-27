//! A module which provides some functionalities to work with the terminal screen.
//! Like allowing you to switch between main and alternate screen or putting the terminal into raw mode.

mod alternate;
mod raw;
mod screen;

use crossterm_utils::TerminalOutput;

pub use self::alternate::AlternateScreen;
pub use self::raw::RawScreen;
pub use self::screen::Screen;
