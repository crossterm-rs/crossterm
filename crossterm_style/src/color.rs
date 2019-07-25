//! A module that contains all the actions related to the styling of the terminal.
//! Like applying attributes to text and changing the foreground and background.

use std::io;

use super::*;
use crate::{Color, ITerminalColor};
use crossterm_utils::{impl_display, Command, Result};
use std::clone::Clone;

#[cfg(windows)]
use crossterm_utils::supports_ansi;

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
pub struct TerminalColor {
    #[cfg(windows)]
    color: Box<(dyn ITerminalColor + Sync + Send)>,
    #[cfg(unix)]
    color: AnsiColor,
}

impl TerminalColor {
    /// Create new instance whereon color related actions can be performed.
    pub fn new() -> TerminalColor {
        #[cfg(windows)]
        let color = if supports_ansi() {
            Box::from(AnsiColor::new()) as Box<(dyn ITerminalColor + Sync + Send)>
        } else {
            WinApiColor::new() as Box<(dyn ITerminalColor + Sync + Send)>
        };

        #[cfg(unix)]
        let color = AnsiColor::new();

        TerminalColor { color }
    }

    /// Set the foreground color to the given color.
    pub fn set_fg(&self, color: Color) -> Result<()> {
        self.color.set_fg(color)
    }

    /// Set the background color to the given color.
    pub fn set_bg(&self, color: Color) -> Result<()> {
        self.color.set_bg(color)
    }

    /// Reset the terminal colors and attributes to default.
    pub fn reset(&self) -> Result<()> {
        self.color.reset()
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
pub fn color() -> TerminalColor {
    TerminalColor::new()
}

/// When executed, this command will set the foreground color of the terminal to the given color.
///
/// See `crossterm/examples/command.rs` for more information on how to execute commands.
pub struct SetFg(pub Color);

impl Command for SetFg {
    type AnsiType = String;

    fn get_ansi_code(&self) -> Self::AnsiType {
        ansi_color::get_set_fg_ansi(self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        WinApiColor::new().set_fg(self.0)
    }
}

/// When executed, this command will set the background color of the terminal to the given color.
///
/// See `crossterm/examples/command.rs` for more information on how to execute commands.
pub struct SetBg(pub Color);

impl Command for SetBg {
    type AnsiType = String;

    fn get_ansi_code(&self) -> Self::AnsiType {
        ansi_color::get_set_bg_ansi(self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        WinApiColor::new().set_fg(self.0)
    }
}

/// When executed, this command will set the given attribute to the terminal.
///
/// See `crossterm/examples/command.rs` for more information on how to execute commands.
pub struct SetAttr(pub Attribute);

impl Command for SetAttr {
    type AnsiType = String;

    fn get_ansi_code(&self) -> Self::AnsiType {
        ansi_color::get_set_attr_ansi(self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        // attributes are not supported by WinAPI.
        Ok(())
    }
}

/// When executed, this command will print the styled font to the terminal.
///
/// See `crossterm/examples/command.rs` for more information on how to execute commands.
pub struct PrintStyledFont<D: Display + Clone>(pub StyledObject<D>);

impl<D> Command for PrintStyledFont<D>
where
    D: Display + Clone,
{
    type AnsiType = StyledObject<D>;

    fn get_ansi_code(&self) -> Self::AnsiType {
        self.0.clone()
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        // attributes are not supported by WinAPI.
        Ok(())
    }
}

impl_display!(for SetFg);
impl_display!(for SetBg);
impl_display!(for SetAttr);
impl_display!(for PrintStyledFont<String>);
impl_display!(for PrintStyledFont<&'static str>);
