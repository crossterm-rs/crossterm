//! This module contains all the logic for switching between alternate screen and main screen.
//!
//!
//! *Nix style applications often utilize an alternate screen buffer, so that they can modify the entire contents of the buffer, without affecting the application that started them.
//! The alternate buffer is exactly the dimensions of the window, without any scrollback region.
//! For an example of this behavior, consider when vim is launched from bash.
//! Vim uses the entirety of the screen to edit the file, then returning to bash leaves the original buffer unchanged.
//!
//!
//! When using alternate screen there is one thing to keep in mind.
//! To get the functionalities of `cursor, color, terminal` also working on alternate screen.
//! You need to pass it the same `Context` as you have passed to the previous three functions,
//! If you don't use the same `Context` the `cursor(), color(), terminal()` these modules will be using main screen to write to.
//! So you will see nothing on alternate screen.
//!
//!
//! When you want to switch to alternate screen there are a couple of things to keep in mind for it to work correctly.
//! First off some code of how to switch to Alternate screen, for more info check the example folder at github
//! Create alternate screen from `Crossterm`:
//!
//!
//! Todo: example
//!

use super::commands;
use super::{functions, ScreenManager};

use std::convert::From;
use std::io::{self, Write};

pub struct AlternateScreen;

impl AlternateScreen {
    /// Create an new alternate screen type.
    pub fn new() -> Box<commands::IAlternateScreenCommand> {
        #[cfg(target_os = "windows")]
        let command = functions::get_module::<Box<commands::IAlternateScreenCommand>>(
            Box::from(commands::win_commands::ToAlternateScreenCommand::new()),
            Box::from(commands::shared_commands::ToAlternateScreenCommand::new()),
        ).unwrap();

        #[cfg(not(target_os = "windows"))]
        let command = Box::from(commands::shared_commands::ToAlternateScreenCommand::new());

        command
    }
}
