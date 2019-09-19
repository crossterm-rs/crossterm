//! This module contains all the logic for switching between alternate screen and main screen.
//!
//! *Nix style applications often utilize an alternate screen buffer, so that they can modify the entire contents of the buffer, without affecting the application that started them.
//! The alternate buffer is exactly the dimensions of the window, without any scrollback region.
//! For an example of this behavior, consider when vim is launched from bash.
//! Vim uses the entirety of the screen to edit the file, then returning to bash leaves the original buffer unchanged.

#[cfg(windows)]
use crossterm_utils::supports_ansi;
use crossterm_utils::Result;

#[cfg(windows)]
use crate::sys::winapi::ToAlternateScreenCommand;
use crate::sys::{self, IAlternateScreenCommand};

use super::RawScreen;

/// With this type you will be able to switch to the alternate screen and then back to the main screen.
/// Check also the Screen type for switching to alternate mode.
///
/// Although this type is available for you to use I would recommend using `Screen` instead.
pub struct AlternateScreen {
    #[cfg(windows)]
    command: Box<(dyn IAlternateScreenCommand + Sync + Send)>,
    #[cfg(unix)]
    command: sys::ToAlternateScreenCommand,
    _raw_screen: Option<RawScreen>,
}

impl AlternateScreen {
    /// Switch to the alternate screen. This function will return an `AlternateScreen` instance if everything went well. This type will give you control over the `AlternateScreen`.
    ///
    /// The bool specifies whether the screen should be in raw mode or not.
    ///
    /// # What is Alternate screen?
    /// *Nix style applications often utilize an alternate screen buffer, so that they can modify the entire contents of the buffer without affecting the application that started them.
    /// The alternate buffer dimensions are exactly the same as the window, without any scrollback region.
    /// For an example of this behavior, consider when vim is launched from bash.
    /// Vim uses the entirety of the screen to edit the file, then returning to bash leaves the original buffer unchanged.
    pub fn to_alternate(raw_mode: bool) -> Result<AlternateScreen> {
        #[cfg(windows)]
        let command = if supports_ansi() {
            Box::from(sys::ToAlternateScreenCommand::new())
                as Box<(dyn IAlternateScreenCommand + Sync + Send)>
        } else {
            Box::from(ToAlternateScreenCommand::new())
                as Box<(dyn IAlternateScreenCommand + Sync + Send)>
        };

        #[cfg(unix)]
        let command = sys::ToAlternateScreenCommand::new();

        command.enable()?;

        if raw_mode {
            let raw_screen = RawScreen::into_raw_mode()?;
            return Ok(AlternateScreen {
                command,
                _raw_screen: Some(raw_screen),
            });
        }

        Ok(AlternateScreen {
            command,
            _raw_screen: None,
        })
    }

    /// Switch the alternate screen back to the main screen.
    pub fn to_main(&self) -> Result<()> {
        self.command.disable()
    }
}

impl Drop for AlternateScreen {
    /// This will switch back to the main screen on drop.
    fn drop(&mut self) {
        let _ = self.to_main();
    }
}
