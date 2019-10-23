//! # Cursor
//!
//! The `cursor` module provides a functionality to work with the terminal cursor.
//!
//! This documentation does not contain a lot of examples. The reason is that it's fairly
//! obvious how to use this crate. Although, we do provide
//! [examples](https://github.com/crossterm-rs/examples) repository
//! to demonstrate the capabilities.
//!
//! ## Examples
//!
//! Commands:
//!
//! ```no_run
//! use std::io::{stdout, Write};
//!
//! use crossterm::{BlinkOff, BlinkOn, execute, Goto, ResetPos, Result, SavePos, ExecutableCommand};
//!
//! fn main() -> Result<()> {
//!     // with macro
//!     execute!(
//!         stdout(),
//!         SavePos,
//!         Goto(10, 10),
//!         BlinkOn,
//!         BlinkOff,
//!         ResetPos
//!     );
//!
//!   // with function
//!   stdout()
//!     .execute(Goto(11,11))?
//!     .execute(ResetPos);
//!
//!  Ok(())
//! }
//! ```
//! Manual execution control `crossterm::queue`

pub use sys::get_cursor_position as pos;

use crate::impl_display;
#[cfg(windows)]
use crate::utils::{Command, Result};

mod ansi;
pub(crate) mod sys;

/// A command to move the cursor to the given position.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
pub struct Goto(pub u16, pub u16);

impl Command for Goto {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::goto_csi_sequence(self.0, self.1)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::goto(self.0, self.1)
    }
}

/// A command to move the cursor given rows up.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
pub struct Up(pub u16);

impl Command for Up {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::move_up_csi_sequence(self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::move_up(self.0)
    }
}

/// A command to move the cursor given rows down.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
pub struct Down(pub u16);

impl Command for Down {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::move_down_csi_sequence(self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::move_down(self.0)
    }
}

/// A command to move the cursor given columns left.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
pub struct Left(pub u16);

impl Command for Left {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::move_left_csi_sequence(self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::move_left(self.0)
    }
}

/// A command to move the cursor given columns right.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
pub struct Right(pub u16);

impl Command for Right {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::move_right_csi_sequence(self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::move_right(self.0)
    }
}

/// A command to save the cursor position.
///
/// # Notes
///
/// The cursor position is stored globally and is not related to the current/any
/// `TerminalCursor` instance.
///
/// Commands must be executed/queued for execution otherwise they do nothing.
pub struct SavePos;

impl Command for SavePos {
    type AnsiType = &'static str;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::SAVE_POSITION_CSI_SEQUENCE
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::save_position()
    }
}

/// A command to restore the saved cursor position.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
pub struct ResetPos;

impl Command for ResetPos {
    type AnsiType = &'static str;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::RESTORE_POSITION_CSI_SEQUENCE
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::restore_position()
    }
}

/// A command to hide the cursor.
///
/// # Notes
///
/// The cursor position is stored globally and is not related to the current/any
/// `TerminalCursor` instance.
///
/// Commands must be executed/queued for execution otherwise they do nothing.
pub struct Hide;

impl Command for Hide {
    type AnsiType = &'static str;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::HIDE_CSI_SEQUENCE
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::show_cursor(false)
    }
}

/// A command to show the cursor.
///
/// # Notes
///
/// The cursor position is stored globally and is not related to the current/any
/// `TerminalCursor` instance.
///
/// Commands must be executed/queued for execution otherwise they do nothing.
pub struct Show;

impl Command for Show {
    type AnsiType = &'static str;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::SHOW_CSI_SEQUENCE
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::show_cursor(true)
    }
}

/// A command to enable the cursor blinking.
///
/// # Notes
///
/// Windows versions lower than Windows 10 do not support this functionality.
///
/// Commands must be executed/queued for execution otherwise they do nothing.
pub struct BlinkOn;

impl Command for BlinkOn {
    type AnsiType = &'static str;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::BLINKING_ON_CSI_SEQUENCE
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        Ok(())
    }
}

/// A command to disable the cursor blinking.
///
/// # Notes
///
/// Windows versions lower than Windows 10 do not support this functionality.
///
/// Commands must be executed/queued for execution otherwise they do nothing.
pub struct BlinkOff;

impl Command for BlinkOff {
    type AnsiType = &'static str;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::BLINKING_OFF_CSI_SEQUENCE
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        Ok(())
    }
}

impl_display!(for Goto);
impl_display!(for Up);
impl_display!(for Down);
impl_display!(for Left);
impl_display!(for Right);
impl_display!(for SavePos);
impl_display!(for ResetPos);
impl_display!(for Hide);
impl_display!(for Show);
impl_display!(for BlinkOn);
impl_display!(for BlinkOff);
