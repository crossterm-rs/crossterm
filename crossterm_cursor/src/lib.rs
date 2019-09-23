#![deny(unused_imports)]

//! # Cursor
//!
//! The `crossterm_cursor` crate provides a functionality to work with the terminal cursor.
//!
//! This documentation does not contain lot of examples. The reason is that it's fairly
//! obvious how to use this crate. Although, we do provide
//! [separate examples](https://github.com/TimonPost/crossterm/tree/master/examples) and
//! some [complex programs](https://github.com/TimonPost/crossterm/tree/master/examples/program_examples)
//! to demonstrate the capabilities.
//!
//! ## Examples
//!
//! Basic usage:
//!
//! ```no_run
//! // You can replace the following line with `use crossterm::TerminalCursor;`
//! // if you're using the `crossterm` crate with the `cursor` feature enabled.
//! use crossterm_cursor::{Result, TerminalCursor};
//!
//! fn main() -> Result<()> {
//!     // Get a cursor, save position
//!     let cursor = TerminalCursor::new();
//!     cursor.save_position()?;
//!
//!     // Do something with the cursor
//!     cursor.goto(10, 10)?;
//!     cursor.blink(true)?;
//!
//!     // Be a good citizen, cleanup
//!     cursor.blink(false)?;
//!     cursor.restore_position()
//! }
//! ```
//!
//! Commands:
//!
//! ```no_run
//! use std::io::{stdout, Write};
//!
//! use crossterm_cursor::{execute, Result};
//! use crossterm_cursor::{BlinkOff, BlinkOn, Goto, ResetPos, SavePos};
//!
//! fn main() -> Result<()> {
//!     execute!(
//!         stdout(),
//!         SavePos,
//!         Goto(10, 10),
//!         BlinkOn,
//!         BlinkOff,
//!         ResetPos
//!     )
//! }   
//! ```
use crossterm_utils::impl_display;
#[cfg(windows)]
use crossterm_utils::supports_ansi;
#[doc(no_inline)]
pub use crossterm_utils::{
    execute, queue, Command, ErrorKind, ExecutableCommand, Output, QueueableCommand, Result,
};
use cursor::ansi::{self, AnsiCursor};
#[cfg(windows)]
use cursor::windows::WindowsCursor;
use cursor::Cursor;

mod cursor;
mod sys;

/// A terminal cursor.
///
/// The `TerminalCursor` instance is stateless and does not hold any data.
/// You can create as many instances as you want and they will always refer to the
/// same terminal cursor.
///
/// The cursor position is 0 based. For example `0` means first column / row, `1`
/// second column / row, etc.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use crossterm_cursor::{Result, TerminalCursor};
///
/// fn main() -> Result<()> {
///     let cursor = TerminalCursor::new();
///     cursor.save_position()?;
///
///     cursor.goto(10, 10)?;
///     cursor.blink(true)?;
///
///     cursor.blink(false)?;
///     cursor.restore_position()
/// }
/// ```
pub struct TerminalCursor {
    #[cfg(windows)]
    cursor: Box<(dyn Cursor + Sync + Send)>,
    #[cfg(unix)]
    cursor: AnsiCursor,
}

impl TerminalCursor {
    /// Creates a new terminal cursor.
    pub fn new() -> TerminalCursor {
        #[cfg(windows)]
        let cursor = if supports_ansi() {
            Box::new(AnsiCursor::new()) as Box<(dyn Cursor + Sync + Send)>
        } else {
            Box::new(WindowsCursor::new()) as Box<(dyn Cursor + Sync + Send)>
        };

        #[cfg(unix)]
        let cursor = AnsiCursor::new();

        TerminalCursor { cursor }
    }

    /// Moves the cursor to the given position.
    pub fn goto(&self, x: u16, y: u16) -> Result<()> {
        self.cursor.goto(x, y)
    }

    /// Gets the cursor position.
    pub fn pos(&self) -> Result<(u16, u16)> {
        self.cursor.pos()
    }

    /// Moves the cursor `row_count` times up.
    pub fn move_up(&mut self, row_count: u16) -> Result<&mut TerminalCursor> {
        self.cursor.move_up(row_count)?;
        Ok(self)
    }

    /// Moves the cursor `col_count` times right.
    pub fn move_right(&mut self, col_count: u16) -> Result<&mut TerminalCursor> {
        self.cursor.move_right(col_count)?;
        Ok(self)
    }

    /// Moves the cursor `row_count` times down.
    pub fn move_down(&mut self, row_count: u16) -> Result<&mut TerminalCursor> {
        self.cursor.move_down(row_count)?;
        Ok(self)
    }

    /// Moves the cursor `col_count` times left.
    pub fn move_left(&mut self, col_count: u16) -> Result<&mut TerminalCursor> {
        self.cursor.move_left(col_count)?;
        Ok(self)
    }

    /// Saves the cursor position.
    ///
    /// See the [restore_position](struct.TerminalCursor.html#method.restore_position) method.
    ///
    /// # Notes
    ///
    /// The cursor position is stored globally and is not related to the current / any
    /// `TerminalCursor` instance.
    pub fn save_position(&self) -> Result<()> {
        self.cursor.save_position()
    }

    /// Restores the saved cursor position.
    ///
    /// See the [save_position](struct.TerminalCursor.html#method.save_position) method.
    pub fn restore_position(&self) -> Result<()> {
        self.cursor.restore_position()
    }

    /// Hides the cursor.
    ///
    /// See the [show](struct.TerminalCursor.html#method.show) method.
    pub fn hide(&self) -> Result<()> {
        self.cursor.hide()
    }

    /// Shows the cursor.
    ///
    /// See the [hide](struct.TerminalCursor.html#method.hide) method.
    pub fn show(&self) -> Result<()> {
        self.cursor.show()
    }

    /// Enables or disables the cursor blinking.
    ///
    /// # Notes
    ///
    /// Not all terminals do support this functionality. Windows versions lower than
    /// Windows 10 do not support this functionality.
    pub fn blink(&self, blink: bool) -> Result<()> {
        self.cursor.blink(blink)
    }
}

/// Creates a new terminal cursor.
pub fn cursor() -> TerminalCursor {
    TerminalCursor::new()
}

/// A command to move the cursor to the given position.
///
/// # Notes
///
/// Commands must be executed / queued for execution otherwise they do nothing.
pub struct Goto(pub u16, pub u16);

impl Command for Goto {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::get_goto_code(self.0, self.1)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        WindowsCursor::new().goto(self.0, self.1)
    }
}

/// A command to move the cursor given rows up.
///
/// # Notes
///
/// Commands must be executed / queued for execution otherwise they do nothing.
pub struct Up(pub u16);

impl Command for Up {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::get_move_up_code(self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        WindowsCursor::new().move_up(self.0)
    }
}

/// A command to move the cursor given rows down.
///
/// # Notes
///
/// Commands must be executed / queued for execution otherwise they do nothing.
pub struct Down(pub u16);

impl Command for Down {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::get_move_down_code(self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        WindowsCursor::new().move_down(self.0)
    }
}

/// A command to move the cursor given columns left.
///
/// # Notes
///
/// Commands must be executed / queued for execution otherwise they do nothing.
pub struct Left(pub u16);

impl Command for Left {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::get_move_left_code(self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        WindowsCursor::new().move_left(self.0)
    }
}

/// A command to move the cursor given columns right.
///
/// # Notes
///
/// Commands must be executed / queued for execution otherwise they do nothing.
pub struct Right(pub u16);

impl Command for Right {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::get_move_right_code(self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        WindowsCursor::new().move_right(self.0)
    }
}

/// A command to save the cursor position.
///
/// # Notes
///
/// The cursor position is stored globally and is not related to the current / any
/// `TerminalCursor` instance.
///
/// Commands must be executed / queued for execution otherwise they do nothing.
pub struct SavePos;

impl Command for SavePos {
    type AnsiType = &'static str;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::SAVE_POS_CODE
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        WindowsCursor::new().save_position()
    }
}

/// A command to restore the saved cursor position.
///
/// # Notes
///
/// Commands must be executed / queued for execution otherwise they do nothing.
pub struct ResetPos;

impl Command for ResetPos {
    type AnsiType = &'static str;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::RESTORE_POS_CODE
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        WindowsCursor::new().restore_position()
    }
}

/// A command to hide the cursor.
///
/// # Notes
///
/// The cursor position is stored globally and is not related to the current / any
/// `TerminalCursor` instance.
///
/// Commands must be executed / queued for execution otherwise they do nothing.
pub struct Hide;

impl Command for Hide {
    type AnsiType = &'static str;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::HIDE_CODE
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        WindowsCursor::new().hide()
    }
}

/// A command to show the cursor.
///
/// # Notes
///
/// The cursor position is stored globally and is not related to the current / any
/// `TerminalCursor` instance.
///
/// Commands must be executed / queued for execution otherwise they do nothing.
pub struct Show;

impl Command for Show {
    type AnsiType = &'static str;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::SHOW_CODE
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        WindowsCursor::new().show()
    }
}

/// A command to enable the cursor blinking.
///
/// # Notes
///
/// Not all terminals do support this functionality. Windows versions lower than
/// Windows 10 do not support this functionality.
///
/// Commands must be executed / queued for execution otherwise they do nothing.
pub struct BlinkOn;

impl Command for BlinkOn {
    type AnsiType = &'static str;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::BLINK_ON_CODE
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
/// Not all terminals do support this functionality. Windows versions lower than
/// Windows 10 do not support this functionality.
///
/// Commands must be executed / queued for execution otherwise they do nothing.
pub struct BlinkOff;

impl Command for BlinkOff {
    type AnsiType = &'static str;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::BLINK_OFF_CODE
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
