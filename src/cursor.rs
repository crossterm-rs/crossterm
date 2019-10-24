//! # Cursor
//!
//! The `cursor` module provides functionality to work with the terminal cursor.
//!
//! This documentation does not contain a lot of examples. The reason is that it's fairly
//! obvious how to use this crate. Although, we do provide
//! [examples](https://github.com/crossterm-rs/examples) repository
//! to demonstrate the capabilities.
//!
//! ## Examples
//! Cursor actions can be performed with commands.
//! Please have a look at [command documention](../index.html) for a more detailed documentation.
//!
//! ```no_run
//! use std::io::{stdout, Write};
//!
//! use crossterm::{ExecutableCommand, execute, Result, cursor::{DisableBlinking, EnableBlinking, MoveTo, RestorePosition, SavePosition}};
//!
//! fn main() -> Result<()> {
//!     // with macro
//!     execute!(
//!         stdout(),
//!         SavePosition,
//!         MoveTo(10, 10),
//!         EnableBlinking,
//!         DisableBlinking,
//!         RestorePosition
//!     );
//!
//!   // with function
//!   stdout()
//!     .execute(MoveTo(11,11))?
//!     .execute(RestorePosition);
//!
//!  Ok(())
//! }
//! ```
//!
//! For manual execution control check out [crossterm::queue](../macro.queue.html).

pub use sys::position;

use crate::impl_display;
#[cfg(windows)]
use crate::utils::Result;

use crate::utils::Command;

mod ansi;
pub(crate) mod sys;

/// Moves the terminal cursor to the given position (column, row).
///
/// # Notes
/// - The counting of the given dimensions starts from 0 were column 0 and row 0 is the top left.
/// - Commands must be executed/queued for execution otherwise they do nothing.
pub struct MoveTo(pub u16, pub u16);

impl Command for MoveTo {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::move_to_csi_sequence(self.0, self.1)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::move_to(self.0, self.1)
    }
}

/// Moves the terminal cursor a given number of rows up.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
pub struct MoveUp(pub u16);

impl Command for MoveUp {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::move_up_csi_sequence(self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::move_up(self.0)
    }
}

/// Moves the terminal cursor a given number of rows down.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
pub struct MoveDown(pub u16);

impl Command for MoveDown {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::move_down_csi_sequence(self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::move_down(self.0)
    }
}

/// Moves the terminal cursor a given number of columns to the left.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
pub struct MoveLeft(pub u16);

impl Command for MoveLeft {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::move_left_csi_sequence(self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::move_left(self.0)
    }
}

/// Moves the terminal cursor a given number of columns to the right.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
pub struct MoveRight(pub u16);

/// Saves the current terminal cursor position.
///
/// See the [RestorePosition](./struct.RestorePosition.html) command.
/// # Notes
///
/// - The cursor position is stored globally.
/// - Commands must be executed/queued for execution otherwise they do nothing.
pub struct SavePosition;

impl Command for MoveRight {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::move_right_csi_sequence(self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::move_right(self.0)
    }
}

impl Command for SavePosition {
    type AnsiType = &'static str;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::SAVE_POSITION_CSI_SEQUENCE
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::save_position()
    }
}

/// Restores the saved terminal cursor position.
///
/// See the [SavePosition](./struct.SavePosition.html) command.
/// # Notes
/// - The cursor position is stored globally.
/// - Commands must be executed/queued for execution otherwise they do nothing.
pub struct RestorePosition;

impl Command for RestorePosition {
    type AnsiType = &'static str;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::RESTORE_POSITION_CSI_SEQUENCE
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::restore_position()
    }
}

/// Hides the terminal cursor indicator.
///
/// # Notes
///
/// - Commands must be executed/queued for execution otherwise they do nothing.
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

/// Shows the terminal cursor indicator.
///
/// # Notes
///
/// - Commands must be executed/queued for execution otherwise they do nothing.
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

/// Enables blinking of the terminal cursor.
///
/// # Notes
///
/// - Windows versions lower than Windows 10 do not support this functionality.
/// - Commands must be executed/queued for execution otherwise they do nothing.
pub struct EnableBlinking;

impl Command for EnableBlinking {
    type AnsiType = &'static str;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::ENABLE_BLINKING_CSI_SEQUENCE
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        Ok(())
    }
}

/// Disables blinking of the terminal cursor.
///
/// # Notes
///
/// - Windows versions lower than Windows 10 do not support this functionality.
/// - Commands must be executed/queued for execution otherwise they do nothing.
pub struct DisableBlinking;

impl Command for DisableBlinking {
    type AnsiType = &'static str;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::DISABLE_BLINKING_CSI_SEQUENCE
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        Ok(())
    }
}

impl_display!(for MoveTo);
impl_display!(for MoveUp);
impl_display!(for MoveDown);
impl_display!(for MoveLeft);
impl_display!(for MoveRight);
impl_display!(for SavePosition);
impl_display!(for RestorePosition);
impl_display!(for Hide);
impl_display!(for Show);
impl_display!(for EnableBlinking);
impl_display!(for DisableBlinking);
