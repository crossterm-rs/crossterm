//! This module contains all the logic for switching between alternate screen and main screen.
//!
//!
//! *Nix style applications often utilize an alternate screen buffer, so that they can modify the entire contents of the buffer, without affecting the application that started them.
//! The alternate buffer is exactly the dimensions of the window, without any scrollback region.
//! For an example of this behavior, consider when vim is launched from bash.
//! Vim uses the entirety of the screen to edit the file, then returning to bash leaves the original buffer unchanged.

use super::commands::{self, IAlternateScreenCommand};
use super::{functions, RawScreen};

use TerminalOutput;

use std::convert::From;
use std::io::{self, Write};
use std::sync::Mutex;

pub type AlternateScreen = Box<IAlternateScreenCommand + Send>;

pub fn alternate_screen() -> AlternateScreen {
    #[cfg(target_os = "windows")]
        let command = functions::get_module::<Box<commands::IAlternateScreenCommand + Send>>(
            Box::from(commands::win_commands::ToAlternateScreenCommand::new()),
            Box::from(commands::shared_commands::ToAlternateScreenCommand::new()),
        ).unwrap();

        #[cfg(not(target_os = "windows"))]
        let command = Box::from(commands::shared_commands::ToAlternateScreenCommand::new());
        command
}

pub fn to_alternate_screen(command: AlternateScreen, target: Arc<TerminalOutput>) -> io::Result<()> {
    command.enable(target)?;
    Ok(())
}

pub fn to_main_screen(command: AlternateScreen, previous: Arc<TerminalOutput>) -> io::Result<()> {
    command.disable(previous)?;
    Ok(())
}
