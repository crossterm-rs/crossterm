//! This module contains all the logic for switching between alternate screen and main screen.
//!
//! *Nix style applications often utilize an alternate screen buffer, so that they can modify the entire contents of the buffer, without affecting the application that started them.
//! The alternate buffer is exactly the dimensions of the window, without any scrollback region.
//! For an example of this behavior, consider when vim is launched from bash.
//! Vim uses the entirety of the screen to edit the file, then returning to bash leaves the original buffer unchanged.

#[cfg(windows)]
use crate::sys::winapi::ToAlternateScreenCommand;
#[cfg(windows)]
use crossterm_utils::supports_ansi;

use crate::sys::{self, IAlternateScreenCommand};

use super::{RawScreen, Screen};
use std::convert::From;
use std::io;

/// With this type you will be able to switch to alternate screen and back to main screen.
/// Check also the Screen type for swishing to alternate mode.
///
/// Although this type is available for you to use I would recommend using `Screen` instead.
pub struct AlternateScreen {
    #[cfg(windows)]
    command: Box<(dyn IAlternateScreenCommand + Sync + Send)>,
    #[cfg(unix)]
    command: sys::ToAlternateScreenCommand,

    pub screen: Screen,
}

impl AlternateScreen {
    /// Switch to alternate screen. This function will return an `AlternateScreen` instance if everything went well this type will give you control over the `AlternateScreen`.
    ///
    /// The bool specifies whether the screen should be in raw mode or not.
    ///
    /// # What is Alternate screen?
    /// *Nix style applications often utilize an alternate screen buffer, so that they can modify the entire contents of the buffer, without affecting the application that started them.
    /// The alternate buffer is exactly the dimensions of the window, without any scrollback region.
    /// For an example of this behavior, consider when vim is launched from bash.
    /// Vim uses the entirety of the screen to edit the file, then returning to bash leaves the original buffer unchanged.
    pub fn to_alternate_screen(raw_mode: bool) -> io::Result<AlternateScreen> {
        #[cfg(windows)]
        let command = if supports_ansi() {
            Box::from(ToAlternateScreenCommand::new())
                as Box<(dyn IAlternateScreenCommand + Sync + Send)>
        } else {
            Box::from(sys::ToAlternateScreenCommand::new())
                as Box<(dyn IAlternateScreenCommand + Sync + Send)>
        };

        #[cfg(unix)]
        let command = sys::ToAlternateScreenCommand::new();

        command.enable()?;

        let screen = Screen::new(raw_mode);

        if raw_mode {
            RawScreen::into_raw_mode()?;
        }

        Ok(AlternateScreen { command, screen })
    }

    /// Switch the alternate screen back to main screen.
    pub fn to_main_screen(&self) -> io::Result<()> {
        self.command.disable()?;
        Ok(())
    }
}

impl Drop for AlternateScreen {
    /// This will switch back to main screen on drop.
    fn drop(&mut self) {
        self.to_main_screen().unwrap();
    }
}
