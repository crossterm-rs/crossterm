//! # Screen
//!
//! The `screen` module provides the functionality to work with the terminal screen.
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
//!
//! Those modes will be set when enabling raw modes:
//!
//! - Input will not be forwarded to screen
//! - Input will not be processed on enter press
//! - Input will not be line buffered (input sent byte-by-byte to input buffer)
//! - Special keys like backspace and CTL+C will not be processed by terminal driver
//! - New line character will not be processed therefore `println!` can't be used, use `write!` instead

pub use self::raw::{IntoRawMode, RawScreen};
use crate::utils::Command;

#[cfg(windows)]
use crate::utils::Result;
#[cfg(windows)]
use crossterm_winapi::{Handle, ScreenBuffer};

mod ansi;
mod raw;
mod sys;

/// A command that switches to alternate screen.
///
/// # Notes
///
/// * Commands must be executed/queued for execution otherwise they do nothing.
/// * Use [LeaveAlternateScreen](./struct.LeaveAlternateScreen.html) command to leave the entered alternate screen.
///
/// # Examples
///
/// ```no_run
/// use std::io::{stdout, Write};
/// use crossterm::{execute, Result,screen::{EnterAlternateScreen, LeaveAlternateScreen}};
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
        ansi::ENTER_ALTERNATE_SCREEN_CSI_SEQUENCE
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        let alternate_screen = ScreenBuffer::create();
        alternate_screen.show()?;
        Ok(())
    }
}

/// A command that switches back to the main alternate screen.
///
/// # Notes
///
/// * Commands must be executed/queued for execution otherwise they do nothing.
/// * Use [EnterAlternateScreen](./struct.EnterAlternateScreen.html) to enter the alternate screen.
///
/// # Examples
///
/// ```no_run
/// use std::io::{stdout, Write};
/// use crossterm::{execute, Result, screen::{EnterAlternateScreen, LeaveAlternateScreen}};
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
        ansi::LEAVE_ALTERNATE_SCREEN_CSI_SEQUENCE
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        let screen_buffer = ScreenBuffer::from(Handle::current_out_handle()?);
        screen_buffer.show()?;
        Ok(())
    }
}
