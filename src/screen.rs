#![deny(unused_imports, unused_must_use)]

//! # Screen
//!
//! **The `crossterm_screen` crate is deprecated and no longer maintained. The GitHub repository will
//! be archived soon. All the code is being moved to the `crossterm`
//! [crate](https://github.com/crossterm-rs/crossterm). You can learn more in
//! the [Merge sub-crates to the crossterm crate](https://github.com/crossterm-rs/crossterm/issues/265)
//! issue.**
//!
//! The `crossterm_screen` crate provides the functionality to work with the terminal screen.
//!
//! This documentation does not contain a lot of examples. The reason is that it's fairly
//! obvious how to use this crate. Although, we do provide
//! [examples](https://github.com/crossterm-rs/examples) repository
//! to demonstrate the capabilities.
//!
//! ## Screen Buffer
//!
//! A screen buffer is a two-dimensional array of characters and color data to be output in a console window.
//! A terminal can have multiple of those screen buffers, and the active screen buffer is the one that is
//! displayed on the screen.
//!
//! Crossterm allows you to switch between those buffers; the screen you are working in is called the
//! 'main screen'. We call the other screen the 'alternate screen'. One note to take is that crossterm
//! does not support the creation and switching between several buffers.
//!
//! ### Alternate Screen
//!
//! Normally you are working on the main screen but an alternate screen is somewhat different from a
//! normal screen. For example, it has the exact dimensions of the terminal window, without any
//! scroll back region. An example of this is vim when it is launched from bash.
//!
//! Vim uses the entirety of the screen to edit the file, then exits to bash leaving the original buffer unchanged.
//!
//! Crossterm provides the ability to switch to the alternate screen, make some changes, and then go back
//! to the main screen. The main screen will still have its original data since we made all the edits on
//! the alternate screen.
//!
//! ### Raw Mode
//!
//! By default, the terminal behaves in a certain way.
//! You can think of going to a new line if the input is at the end of the current line, or interpreting backspace
//! to remove letters. Sometimes it can be useful to disable these modes because this is undesirable.
//! This may be undesirable if your application wants to read the input without it being shown on the screen.
//! Raw modes are the modes to create this possibility.
//
//! Those modes will be set when enabling raw modes:
//!
//! - Input will not be forwarded to screen
//! - Input will not be processed on enter press
//! - Input will not be line buffered (input sent byte-by-byte to input buffer)
//! - Special keys like backspace and CTL+C will not be processed by terminal driver
//! - New line character will not be processed therefore `println!` can't be used, use `write!` instead

// This brings the trait into scope, so we're able to call enter()/leave(),
// but it it's false positive for unused_imports check
#[allow(unused_imports)]
use alternate::AlternateScreen as _;

use crate::{Command, Result};

pub use self::raw::{IntoRawMode, RawScreen};

mod alternate;
mod raw;
mod sys;

/// An alternate screen.
///
/// With this type, you will be able to switch to the alternate screen and then back to
/// the main screen.
///
/// Be aware that you'll be switched back to the main screen when you drop the
/// `AlternateScreen` value.
///
/// It's recommended to use the command API. See the
/// [`EnterAlternateScreen`](struct.EnterAlternateScreen.html)
/// and [`LeaveAlternateScreen`](struct.LeaveAlternateScreen.html)
/// commands documentation for more info.
///
/// # Examples
///
/// Alternate screen with raw mode enabled:
///
/// ```no_run
/// use crossterm::{AlternateScreen, Result};
///
/// fn main() -> Result<()> {
///     let _alternate = AlternateScreen::to_alternate(true)?;
///
///     // Do something on the alternate screen in the raw mode
///
///     Ok(())
/// } // `_alternate` dropped here <- raw mode disabled & back to main screen
/// ```
pub struct AlternateScreen {
    #[cfg(windows)]
    alternate: Box<(dyn alternate::AlternateScreen + Sync + Send)>,
    #[cfg(unix)]
    alternate: alternate::AnsiAlternateScreen,
    raw_screen: Option<RawScreen>,
}

impl AlternateScreen {
    /// Switches to the alternate screen.
    ///
    /// # Arguments
    ///
    /// * `raw_mode` - `true` enables the raw mode as well
    ///
    /// # Notes
    ///
    /// You'll be automatically switched to the main screen if this function
    /// fails.
    pub fn to_alternate(raw_mode: bool) -> Result<AlternateScreen> {
        let alternate = alternate::alternate_screen();
        alternate.enter()?;

        let mut alternate = AlternateScreen {
            alternate,
            raw_screen: None,
        };

        if raw_mode {
            // If into_raw_mode fails, `alternate` will be dropped and
            // we'll switch back to the main screen.
            alternate.raw_screen = Some(RawScreen::into_raw_mode()?);
        }

        Ok(alternate)
    }

    /// Switches to the main screen.
    pub fn to_main(&self) -> Result<()> {
        self.alternate.leave()
    }
}

impl Drop for AlternateScreen {
    fn drop(&mut self) {
        let _ = self.to_main();
    }
}

/// A command to switch to the alternate screen.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
///
/// # Examples
///
/// ```no_run
/// use std::io::{stdout, Write};
/// use crossterm::{execute, Result, EnterAlternateScreen, LeaveAlternateScreen};
///
/// fn main() -> Result<()> {
///     execute!(stdout(), EnterAlternateScreen)?;
///
///     // Do anything on the alternate screen
///
///     execute!(stdout(), LeaveAlternateScreen)
/// }
/// ```
pub struct EnterAlternateScreen;

impl Command for EnterAlternateScreen {
    type AnsiType = &'static str;

    fn ansi_code(&self) -> Self::AnsiType {
        alternate::ansi::ENTER_ALTERNATE_SCREEN_CSI_SEQUENCE
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        let alternate = alternate::alternate_screen();
        alternate.enter()
    }
}

/// A command to switch back to the main screen.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
///
/// # Examples
///
/// ```no_run
/// use std::io::{stdout, Write};
/// use crossterm::{execute, Result, EnterAlternateScreen, LeaveAlternateScreen};
///
/// fn main() -> Result<()> {
///     execute!(stdout(), EnterAlternateScreen)?;
///
///     // Do anything on the alternate screen
///
///     execute!(stdout(), LeaveAlternateScreen)
/// }
/// ```
pub struct LeaveAlternateScreen;

impl Command for LeaveAlternateScreen {
    type AnsiType = &'static str;

    fn ansi_code(&self) -> Self::AnsiType {
        alternate::ansi::LEAVE_ALTERNATE_SCREEN_CSI_SEQUENCE
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        let alternate = alternate::alternate_screen();
        alternate.leave()
    }
}
