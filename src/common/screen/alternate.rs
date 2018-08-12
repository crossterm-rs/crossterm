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

use super::commands::{self, IAlternateScreenCommand};
use super::{functions, Screen, Stdout, RawScreen};

use std::convert::From;
use std::io::{self, Write};
use std::sync::Mutex;

pub struct AlternateScreen
{
    command: Box<IAlternateScreenCommand + Send>,
    pub screen: Screen,
}

impl AlternateScreen {

    pub fn new(command: Box<IAlternateScreenCommand + Send>, screen: Screen) -> Self
    {
        return AlternateScreen { command, screen }
    }

    pub fn to_alternate_screen(screen_manager: Stdout) -> io::Result<AlternateScreen> {
        #[cfg(target_os = "windows")]
        let command = functions::get_module::<Box<commands::IAlternateScreenCommand + Send>>(
            Box::from(commands::win_commands::ToAlternateScreenCommand::new()),
            Box::from(commands::shared_commands::ToAlternateScreenCommand::new()),
        ).unwrap();

        #[cfg(not(target_os = "windows"))]
        let command = Box::from(commands::shared_commands::ToAlternateScreenCommand::new());

        let mut stdout = screen_manager;
        command.enable(&mut stdout)?;
        return Ok(AlternateScreen::new(command, Screen::from(stdout)));
    }

    pub fn to_main_screen(&self) -> io::Result<()> {
        self.command.disable(&self.screen.stdout)?;
        Ok(())
    }
}

impl Drop for AlternateScreen
{
    fn drop(&mut self) {
        self.to_main_screen();

    }
}
