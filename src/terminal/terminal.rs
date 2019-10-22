//! A module that contains all the actions related to the terminal. like clearing, resizing, pausing
//! and scrolling the terminal.
use crate::utils::Result;

use super::ClearType;

pub(crate) mod ansi;
#[cfg(windows)]
pub(crate) mod winapi;

/// This trait defines the actions that can be performed with the terminal color.
/// This trait can be implemented so that an concrete implementation of the ITerminalColor can fulfill.
/// the wishes to work on an specific platform.
///
/// ## For example:
///
/// This trait is implemented for `WinApi` (Windows specific) and `ANSI` (Unix specific),
/// so that terminal related actions can be performed on both Unix and Windows systems.
pub(crate) trait Terminal {
    /// Clear the current cursor by specifying the clear type
    fn clear(&self, clear_type: ClearType) -> Result<()>;
    /// Get the terminal size (x,y)
    fn size(&self) -> Result<(u16, u16)>;
    /// Scroll `n` lines up in the current terminal.
    fn scroll_up(&self, count: u16) -> Result<()>;
    /// Scroll `n` lines down in the current terminal.
    fn scroll_down(&self, count: u16) -> Result<()>;
    /// Resize terminal to the given width and height.
    fn set_size(&self, width: u16, height: u16) -> Result<()>;
}
