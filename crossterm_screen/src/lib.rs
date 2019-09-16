#![deny(unused_imports)]

//! A module which provides some functionalities to work with the terminal screen.
//! Like allowing you to switch between the main and alternate screen or putting the terminal into raw mode.
pub use self::screen::{AlternateScreen, IntoRawMode, RawScreen};

mod screen;
mod sys;
