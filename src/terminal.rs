//! # Terminal
//!
//! The `terminal` module provides a functionality to work with the terminal.
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
//! use crossterm::{execute, Result, ScrollUp, SetSize, size};
//!
//! fn main() -> Result<()> {
//!     let (cols, rows) = size()?;
//!     // Do something with the terminal
//!     execute!(
//!         stdout(),
//!         SetSize(10, 10),
//!         ScrollUp(5)
//!     )?;
//!
//!     // Be a good citizen, cleanup
//!     execute!(stdout(), SetSize(cols, rows))?;
//!     Ok(())
//! }
//! ```

pub use sys::exit;
pub use sys::get_terminal_size as size;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::impl_display;
#[doc(no_inline)]
use crate::utils::{Command, Result};

mod ansi;
mod sys;

/// Represents different options how to clear the terminal.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum ClearType {
    /// All cells.
    All,
    /// All cells from the cursor position downwards.
    FromCursorDown,
    /// All cells from the cursor position upwards.
    FromCursorUp,
    /// All cells at the cursor row.
    CurrentLine,
    /// All cells from the cursor position until the new line.
    UntilNewLine,
}

/// A command to scroll the terminal given rows up.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
pub struct ScrollUp(pub u16);

impl Command for ScrollUp {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::scroll_up_csi_sequence(self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::scroll_up(self.0)
    }
}

/// A command to scroll the terminal given rows down.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
pub struct ScrollDown(pub u16);

impl Command for ScrollDown {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::scroll_down_csi_sequence(self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::scroll_down(self.0)
    }
}

/// A command to clear the terminal.
///
/// See the [`ClearType`](enum.ClearType.html) enum.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
pub struct Clear(pub ClearType);

impl Command for Clear {
    type AnsiType = &'static str;

    fn ansi_code(&self) -> Self::AnsiType {
        match self.0 {
            ClearType::All => ansi::CLEAR_ALL_CSI_SEQUENCE,
            ClearType::FromCursorDown => ansi::CLEAR_FROM_CURSOR_DOWN_CSI_SEQUENCE,
            ClearType::FromCursorUp => ansi::CLEAR_FROM_CURSOR_UP_CSI_SEQUENCE,
            ClearType::CurrentLine => ansi::CLEAR_FROM_CURRENT_LINE_CSI_SEQUENCE,
            ClearType::UntilNewLine => ansi::CLEAR_UNTIL_NEW_LINE_CSI_SEQUENCE,
        }
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::clear(self.0.clone())
    }
}

/// A command to set the terminal size (rows, columns).
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
pub struct SetSize(pub u16, pub u16);

impl Command for SetSize {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::set_size_csi_sequence(self.0, self.1)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::set_size(self.0, self.1)
    }
}

impl_display!(for ScrollUp);
impl_display!(for ScrollDown);
impl_display!(for SetSize);
impl_display!(for Clear);
