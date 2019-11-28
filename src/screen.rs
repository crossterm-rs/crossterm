//! # Screen
//!
//! The `screen` module provides the functionality to work with terminal modes.
//!
//! ## Screen Buffer
//!
//! A screen buffer is a two-dimensional array of character
//! and color data which is displayed in a terminal screen.
//!
//! The terminal has several of those buffers and is able to switch between them.
//! The default screen in which you work is called the 'main screen'.
//! The other screens are called the 'alternative screen'.
//!
//! It is important to understand that crossterm does not yet support creating screens,
//! or switch between more than two buffers, and only offers the ability to change
//! between the alternate and main screens.
//!
//! ### Alternate Screen
//!
//! By default, you will be working on the main screen, however,
//! the alternative screen is slightly different from the main screen.
//! An example of this is that it has the exact dimensions of the terminal window, without any scroll-back area.
//!
//! Vim is a good example of this. When it is launched from bash, a whole new screen is used to modify a file.
//! Then, when the modification is finished, it closes again and continues on the main screen.
//!
//! Crossterm offers the possibility to switch to the alternative screen,
//! make some modifications, and move back to the main screen again.
//! The main screen will stay intact and will have the original data as we performed all
//! operations on the alternative screen.
//!
//! ### Raw Mode
//!
//! By default, the terminal functions in a certain way.
//! For example, it will move the cursor to the beginning of the next line when the input hits the end of a line.
//! Or that the backspace is interpreted for character removal.
//!
//! Sometimes these default modes are irrelevant,
//! and in this case, we can turn them off.
//! This is what happens when you enable raw modes.
//!
//! Those modes will be set when enabling raw modes:
//!
//! - Input will not be forwarded to screen
//! - Input will not be processed on enter press
//! - Input will not be line buffered (input sent byte-by-byte to input buffer)
//! - Special keys like backspace and CTL+C will not be processed by terminal driver
//! - New line character will not be processed therefore `println!` can't be used, use `write!` instead

#[cfg(windows)]
use crossterm_winapi::{Handle, ScreenBuffer};

use crate::utils::Command;
#[cfg(windows)]
use crate::utils::Result;

pub use self::raw::{IntoRawMode, RawScreen};

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

/// A command that switches back to the main screen.
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
