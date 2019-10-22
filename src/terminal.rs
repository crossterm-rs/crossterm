#![deny(unused_imports, unused_must_use)]

//! # Terminal
//!
//! **The `crossterm_terminal` crate is deprecated and no longer maintained. The GitHub repository will
//! be archived soon. All the code is being moved to the `crossterm`
//! [crate](https://github.com/crossterm-rs/crossterm). You can learn more in
//! the [Merge sub-crates to the crossterm crate](https://github.com/crossterm-rs/crossterm/issues/265)
//! issue.**
//!
//! The `crossterm_terminal` crate provides a functionality to work with the terminal.
//!
//! This documentation does not contain a lot of examples. The reason is that it's fairly
//! obvious how to use this crate. Although, we do provide
//! [examples](https://github.com/crossterm-rs/examples) repository
//! to demonstrate the capabilities.
//!
//! ## Examples
//!
//! ```no_run
//! use crossterm::{Result, Terminal};
//!
//! fn main() -> Result<()> {
//!     // Get a terminal, save size
//!     let terminal = Terminal::new();
//!     let (cols, rows) = terminal.size()?;
//!
//!     // Do something with the terminal
//!     terminal.set_size(10, 10)?;
//!     terminal.scroll_up(5)?;
//!
//!     // Be a good citizen, cleanup
//!     terminal.set_size(cols, rows)
//! }
//! ```
//!
//! Commands:
//!
//! ```no_run
//! use std::io::{stdout, Write};
//! use crossterm::{execute, Result, ScrollUp, SetSize, Terminal};
//!
//! fn main() -> Result<()> {
//!     // Get a terminal, save size
//!     let terminal = Terminal::new();
//!     let (cols, rows) = terminal.size()?;
//!
//!     // Do something with the terminal
//!     execute!(
//!         stdout(),
//!         SetSize(10, 10),
//!         ScrollUp(5)
//!     )?;
//!
//!     // Be a good citizen, cleanup
//!     terminal.set_size(cols, rows)
//! }
//! ```
use std::fmt;

#[cfg(windows)]
use crate::utils::supports_ansi;
#[doc(no_inline)]
use crate::utils::{Command, Result};
use crate::{impl_display, write_cout};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use self::terminal::ansi::AnsiTerminal;
#[cfg(windows)]
use self::terminal::winapi::WinApiTerminal;
use self::terminal::Terminal as TerminalTrait;

mod sys;
mod terminal;

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

/// A terminal.
///
/// The `Terminal` instance is stateless and does not hold any data.
/// You can create as many instances as you want and they will always refer to the
/// same terminal.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use crossterm::{Result, Terminal};
///
/// fn main() -> Result<()> {
///     let terminal = Terminal::new();
///     let (cols, rows) = terminal.size()?;
///
///     terminal.set_size(10, 10)?;
///     terminal.scroll_up(5)?;
///
///     terminal.set_size(cols, rows)
/// }
/// ```
pub struct Terminal {
    #[cfg(windows)]
    terminal: Box<(dyn TerminalTrait + Sync + Send)>,
    #[cfg(unix)]
    terminal: AnsiTerminal,
}

impl Terminal {
    /// Creates a new `Terminal`.
    pub fn new() -> Terminal {
        #[cfg(windows)]
        let terminal = if supports_ansi() {
            Box::from(AnsiTerminal::new()) as Box<(dyn TerminalTrait + Sync + Send)>
        } else {
            WinApiTerminal::new() as Box<(dyn TerminalTrait + Sync + Send)>
        };

        #[cfg(unix)]
        let terminal = AnsiTerminal::new();

        Terminal { terminal }
    }

    /// Clears the terminal.
    ///
    /// See the [`ClearType`](enum.ClearType.html) enum to learn about
    /// all ways how the terminal can be cleared.
    pub fn clear(&self, clear_type: ClearType) -> Result<()> {
        self.terminal.clear(clear_type)
    }

    /// Returns the terminal size (`(columns, rows)`).
    pub fn size(&self) -> Result<(u16, u16)> {
        self.terminal.size()
    }

    /// Scrolls the terminal `row_count` rows up.
    pub fn scroll_up(&self, row_count: u16) -> Result<()> {
        self.terminal.scroll_up(row_count)
    }

    /// Scrolls the terminal `row_count` rows down.
    pub fn scroll_down(&self, row_count: u16) -> Result<()> {
        self.terminal.scroll_down(row_count)
    }

    /// Sets the terminal size.
    pub fn set_size(&self, columns: u16, rows: u16) -> Result<()> {
        self.terminal.set_size(columns, rows)
    }

    /// Exits the current process.
    ///
    /// # Platform-specific Behavior
    ///
    /// [`std::process::exit`](https://doc.rust-lang.org/std/process/fn.exit.html) is
    /// called internally with platform specific exit codes.
    ///
    /// **Unix**: exit code 0.
    ///
    /// **Windows**: exit code 256.
    pub fn exit(&self) {
        crate::terminal::sys::exit();
    }

    /// Writes any displayable content to the current terminal and flushes
    /// the standard output.
    pub fn write<D: fmt::Display>(&self, value: D) -> Result<usize> {
        write_cout!(format!("{}", value))
    }
}

/// Creates a new `Terminal`.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use crossterm::{terminal, Result};
///
/// fn main() -> Result<()> {
///     // Get a terminal, save size
///     let terminal = terminal();
///     let (cols, rows) = terminal.size()?;
///
///     // Do something with the terminal
///     terminal.set_size(10, 10)?;
///     terminal.scroll_up(5)?;
///
///     // Be a good citizen, cleanup
///     terminal.set_size(cols, rows)
/// }
/// ```
pub fn terminal() -> Terminal {
    Terminal::new()
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
        terminal::ansi::scroll_up_csi_sequence(self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        WinApiTerminal::new().scroll_up(self.0)
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
        terminal::ansi::scroll_down_csi_sequence(self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        WinApiTerminal::new().scroll_down(self.0)
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
            ClearType::All => terminal::ansi::CLEAR_ALL_CSI_SEQUENCE,
            ClearType::FromCursorDown => terminal::ansi::CLEAR_FROM_CURSOR_DOWN_CSI_SEQUENCE,
            ClearType::FromCursorUp => terminal::ansi::CLEAR_FROM_CURSOR_UP_CSI_SEQUENCE,
            ClearType::CurrentLine => terminal::ansi::CLEAR_FROM_CURRENT_LINE_CSI_SEQUENCE,
            ClearType::UntilNewLine => terminal::ansi::CLEAR_UNTIL_NEW_LINE_CSI_SEQUENCE,
        }
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        WinApiTerminal::new().clear(self.0.clone())
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
        terminal::ansi::set_size_csi_sequence(self.0, self.1)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        WinApiTerminal::new().set_size(self.0, self.1)
    }
}

impl_display!(for ScrollUp);
impl_display!(for ScrollDown);
impl_display!(for SetSize);
impl_display!(for Clear);
