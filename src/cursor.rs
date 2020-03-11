//! # Cursor
//!
//! The `cursor` module provides functionality to work with the terminal cursor.
//!
//! This documentation does not contain a lot of examples. The reason is that it's fairly
//! obvious how to use this crate. Although, we do provide
//! [examples](https://github.com/crossterm-rs/crossterm/tree/master/examples) repository
//! to demonstrate the capabilities.
//!
//! ## Examples
//!
//! Cursor actions can be performed with commands.
//! Please have a look at [command documention](../index.html#command-api) for a more detailed documentation.
//!
//! ```no_run
//! use std::io::{stdout, Write};
//!
//! use crossterm::{
//!     ExecutableCommand, execute, Result,
//!     cursor::{DisableBlinking, EnableBlinking, MoveTo, RestorePosition, SavePosition}
//! };
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

#[cfg(windows)]
use crate::Result;
use crate::{impl_display, Ansi, Command};
use std::fmt;

mod ansi;
pub(crate) mod sys;

/// A command that moves the terminal cursor to the given position (column, row).
///
/// # Notes
///
/// * Top left cell is represented as `0,0`.
/// * Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveTo(pub u16, pub u16);

impl fmt::Display for Ansi<MoveTo> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ansi::move_to_csi_sequence(f, (self.0).0, (self.0).1)
    }
}

impl Command for MoveTo {
    type AnsiType = Ansi<Self>;

    #[inline]
    fn ansi_code(&self) -> Self::AnsiType {
        Ansi(*self)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::move_to(self.0, self.1)
    }
}

/// A command that moves the terminal cursor up the given number of lines,
/// and moves it to the first column.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveToNextLine(pub u16);

impl fmt::Display for Ansi<MoveToNextLine> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ansi::move_to_next_line_csi_sequence(f, (self.0).0)
    }
}

impl Command for MoveToNextLine {
    type AnsiType = Ansi<Self>;

    #[inline]
    fn ansi_code(&self) -> Self::AnsiType {
        Ansi(*self)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::move_to_next_line(self.0)
    }
}

/// A command that moves the terminal cursor down the given number of lines,
/// and moves it to the first column.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveToPreviousLine(pub u16);

impl fmt::Display for Ansi<MoveToPreviousLine> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ansi::move_to_previous_line_csi_sequence(f, (self.0).0)
    }
}

impl Command for MoveToPreviousLine {
    type AnsiType = Ansi<Self>;

    #[inline]
    fn ansi_code(&self) -> Self::AnsiType {
        Ansi(*self)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::move_to_previous_line(self.0)
    }
}

/// A command that moves the terminal cursor to the given column on the current row.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveToColumn(pub u16);

impl fmt::Display for Ansi<MoveToColumn> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ansi::move_to_column_csi_sequence(f, (self.0).0)
    }
}

impl Command for MoveToColumn {
    type AnsiType = Ansi<Self>;

    #[inline]
    fn ansi_code(&self) -> Self::AnsiType {
        Ansi(*self)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::move_to_column(self.0)
    }
}

/// A command that moves the terminal cursor a given number of rows up.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveUp(pub u16);

impl fmt::Display for Ansi<MoveUp> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ansi::move_up_csi_sequence(f, (self.0).0)
    }
}

impl Command for MoveUp {
    type AnsiType = Ansi<Self>;

    #[inline]
    fn ansi_code(&self) -> Self::AnsiType {
        Ansi(*self)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::move_up(self.0)
    }
}

/// A command that moves the terminal cursor a given number of columns to the right.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveRight(pub u16);

impl fmt::Display for Ansi<MoveRight> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ansi::move_right_csi_sequence(f, (self.0).0)
    }
}

impl Command for MoveRight {
    type AnsiType = Ansi<Self>;

    #[inline]
    fn ansi_code(&self) -> Self::AnsiType {
        Ansi(*self)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::move_right(self.0)
    }
}

/// A command that moves the terminal cursor a given number of rows down.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveDown(pub u16);

impl fmt::Display for Ansi<MoveDown> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ansi::move_down_csi_sequence(f, (self.0).0)
    }
}

impl Command for MoveDown {
    type AnsiType = Ansi<Self>;

    #[inline]
    fn ansi_code(&self) -> Self::AnsiType {
        Ansi(*self)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::move_down(self.0)
    }
}

/// A command that moves the terminal cursor a given number of columns to the left.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveLeft(pub u16);

impl fmt::Display for Ansi<MoveLeft> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ansi::move_left_csi_sequence(f, (self.0).0)
    }
}

impl Command for MoveLeft {
    type AnsiType = Ansi<Self>;

    #[inline]
    fn ansi_code(&self) -> Self::AnsiType {
        Ansi(*self)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::move_left(self.0)
    }
}

/// A command that saves the current terminal cursor position.
///
/// See the [RestorePosition](./struct.RestorePosition.html) command.
///
/// # Notes
///
/// - The cursor position is stored globally.
/// - Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SavePosition;

impl Command for SavePosition {
    type AnsiType = &'static str;

    #[inline]
    fn ansi_code(&self) -> Self::AnsiType {
        ansi::SAVE_POSITION_CSI_SEQUENCE
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::save_position()
    }
}

/// A command that restores the saved terminal cursor position.
///
/// See the [SavePosition](./struct.SavePosition.html) command.
///
/// # Notes
///
/// - The cursor position is stored globally.
/// - Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RestorePosition;

impl Command for RestorePosition {
    type AnsiType = &'static str;

    #[inline]
    fn ansi_code(&self) -> Self::AnsiType {
        ansi::RESTORE_POSITION_CSI_SEQUENCE
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::restore_position()
    }
}

/// A command that hides the terminal cursor.
///
/// # Notes
///
/// - Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hide;

impl Command for Hide {
    type AnsiType = &'static str;

    #[inline]
    fn ansi_code(&self) -> Self::AnsiType {
        ansi::HIDE_CSI_SEQUENCE
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::show_cursor(false)
    }
}

/// A command that shows the terminal cursor.
///
/// # Notes
///
/// - Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Show;

impl Command for Show {
    type AnsiType = &'static str;

    #[inline]
    fn ansi_code(&self) -> Self::AnsiType {
        ansi::SHOW_CSI_SEQUENCE
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::show_cursor(true)
    }
}

/// A command that enables blinking of the terminal cursor.
///
/// # Notes
///
/// - Windows versions lower than Windows 10 do not support this functionality.
/// - Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnableBlinking;

impl Command for EnableBlinking {
    type AnsiType = &'static str;

    #[inline]
    fn ansi_code(&self) -> Self::AnsiType {
        ansi::ENABLE_BLINKING_CSI_SEQUENCE
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        Ok(())
    }
}

/// A command that disables blinking of the terminal cursor.
///
/// # Notes
///
/// - Windows versions lower than Windows 10 do not support this functionality.
/// - Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisableBlinking;

impl Command for DisableBlinking {
    type AnsiType = &'static str;

    #[inline]
    fn ansi_code(&self) -> Self::AnsiType {
        ansi::DISABLE_BLINKING_CSI_SEQUENCE
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        Ok(())
    }
}

impl_display!(for MoveTo);
impl_display!(for MoveToColumn);
impl_display!(for MoveToNextLine);
impl_display!(for MoveToPreviousLine);
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

#[cfg(test)]
mod tests {
    use std::io::{self, stdout, Write};

    use crate::execute;

    use super::{
        position, MoveDown, MoveLeft, MoveRight, MoveTo, MoveUp, RestorePosition, SavePosition,
    };

    // Test is disabled, because it's failing on Travis
    #[test]
    #[ignore]
    fn test_move_to() {
        let (saved_x, saved_y) = position().unwrap();

        execute!(stdout(), MoveTo(saved_x + 1, saved_y + 1)).unwrap();
        assert_eq!(position().unwrap(), (saved_x + 1, saved_y + 1));

        execute!(stdout(), MoveTo(saved_x, saved_y)).unwrap();
        assert_eq!(position().unwrap(), (saved_x, saved_y));
    }

    // Test is disabled, because it's failing on Travis
    #[test]
    #[ignore]
    fn test_move_right() {
        let (saved_x, saved_y) = position().unwrap();
        execute!(io::stdout(), MoveRight(1)).unwrap();
        assert_eq!(position().unwrap(), (saved_x + 1, saved_y));
    }

    // Test is disabled, because it's failing on Travis
    #[test]
    #[ignore]
    fn test_move_left() {
        execute!(stdout(), MoveTo(2, 0), MoveLeft(2)).unwrap();
        assert_eq!(position().unwrap(), (0, 0));
    }

    // Test is disabled, because it's failing on Travis
    #[test]
    #[ignore]
    fn test_move_up() {
        execute!(stdout(), MoveTo(0, 2), MoveUp(2)).unwrap();
        assert_eq!(position().unwrap(), (0, 0));
    }

    // Test is disabled, because it's failing on Travis
    #[test]
    #[ignore]
    fn test_move_down() {
        execute!(stdout(), MoveTo(0, 0), MoveDown(2)).unwrap();

        assert_eq!(position().unwrap(), (0, 2));
    }

    // Test is disabled, because it's failing on Travis
    #[test]
    #[ignore]
    fn test_save_restore_position() {
        let (saved_x, saved_y) = position().unwrap();

        execute!(
            stdout(),
            SavePosition,
            MoveTo(saved_x + 1, saved_y + 1),
            RestorePosition
        )
        .unwrap();

        let (x, y) = position().unwrap();

        assert_eq!(x, saved_x);
        assert_eq!(y, saved_y);
    }
}
