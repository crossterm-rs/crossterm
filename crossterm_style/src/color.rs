//! A module that contains all the actions related to the styling of the terminal.
//! Like applying attributes to font and changing the foreground and background.

use std::io;

use super::*;
use crate::{Color, ITerminalColor};
use crossterm_utils::{Result, TerminalOutput};

#[cfg(windows)]
use crossterm_utils::supports_ansi;

use std::sync::Arc;

/// Allows you to style the terminal.
///
/// # Features:
///
/// - Foreground color (16 base colors)
/// - Background color (16 base colors)
/// - 256 color support (Windows 10 and UNIX only)
/// - RGB support (Windows 10 and UNIX only)
/// - Text Attributes like: bold, italic, underscore and crossed word ect (Windows 10 and UNIX only)
///
/// Check `/examples/` in the library for more specific examples.
///
/// # Remarks
///
/// When you want to 'style' on 'alternate screen' use the 'crossterm_screen' crate.
pub struct TerminalColor<'stdout> {
    color: Box<ITerminalColor + Sync + Send>,
    stdout: Option<&'stdout Arc<TerminalOutput>>,
}

impl<'stdout> TerminalColor<'stdout> {
    /// Create new instance whereon color related actions can be performed.
    pub fn new() -> TerminalColor<'stdout> {
        #[cfg(windows)]
            let instance = if supports_ansi() {
            AnsiColor::new() as Box<(dyn ITerminalColor + Sync + Send)>
        } else {
            WinApiColor::new() as Box<(dyn ITerminalColor + Sync + Send)>
        };

        #[cfg(unix)]
        let instance = AnsiColor::new() as Box<(dyn ITerminalColor + Sync + Send)>;

        TerminalColor {
            color: instance,
            stdout: None,
        }
    }

    /// Create a new instance of `TerminalColor` whereon coloring could be preformed on the given output.
    ///
    /// # Remarks
    ///
    /// Use this function when you want your terminal to operate with a specific output.
    /// This could be useful when you have a screen which is in 'alternate mode',
    /// and you want your actions from the `TerminalColor`, created by this function, to operate on the 'alternate screen'.
    ///
    /// You should checkout the 'crossterm_screen' crate for more information about this.
    ///
    /// # Example
    /// ```
    /// let screen = Screen::default();
    //
    /// if let Ok(alternate) = screen.enable_alternate_modes(false) {
    ///    let terminal = TerminalColor::from_output(&alternate.screen.stdout);
    /// }
    /// ```
    pub fn from_output(stdout: &'stdout Arc<TerminalOutput>) -> TerminalColor<'stdout> {
        #[cfg(windows)]
            let instance = if supports_ansi() {
            AnsiColor::new() as Box<(dyn ITerminalColor + Sync + Send)>
        } else {
            WinApiColor::new() as Box<(dyn ITerminalColor + Sync + Send)>
        };

        #[cfg(unix)]
            let instance = AnsiColor::new() as Box<(dyn ITerminalColor + Sync + Send)>;

        TerminalColor {
            color: instance,
            stdout: Some(stdout),
        }
    }

    /// Set the foreground color to the given color.
    pub fn set_fg(&self, color: Color) -> Result<()> {
        self.color.set_fg(color, &self.stdout)
    }

    /// Set the background color to the given color.
    pub fn set_bg(&self, color: Color) -> Result<()> {
        self.color.set_bg(color, &self.stdout)
    }

    /// Reset the terminal colors and attributes to default.
    pub fn reset(&self) -> Result<()> {
        self.color.reset(&self.stdout)
    }

    /// Get available color count.
    /// (This does not always provide a good result.)
    pub fn get_available_color_count(&self) -> io::Result<u16> {
        use std::env;
        Ok(match env::var_os("TERM") {
            Some(val) => {
                if val.to_str().unwrap_or("").contains("256color") {
                    256
                } else {
                    8
                }
            }
            None => 8,
        })
    }
}

/// Get a `TerminalColor` implementation whereon color related actions can be performed.
pub fn color<'stdout>() -> TerminalColor<'stdout> {
    TerminalColor::new()
}
