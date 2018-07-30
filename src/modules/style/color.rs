//! With this module you can perform actions that are color related.
//! Like styling the font, foreground color and background.

use super::*;
use std::io;

/// Struct that stores an specific platform implementation for color related actions.
///
/// Check `/examples/version/color` in the library for more specific examples.
///
/// #Example
///
/// ```rust
///
/// let crossterm = Crossterm::new();
/// let colored_terminal = crossterm.color();
///
/// // set foreground color
/// colored_terminal.set_fg(Color::Red);
/// // set background color
/// colored_terminal.set_bg(Color::Red);
/// // reset color to default
/// colored_terminal.reset();
///
/// ```
pub struct TerminalColor<'terminal> {
    color: Box<ITerminalColor>,
    screen_manager: &'terminal ScreenManager,
}

impl<'terminal> TerminalColor<'terminal> {
    /// Create new instance whereon color related actions can be performed.
    pub fn new(screen_manager: &'terminal ScreenManager) -> TerminalColor<'terminal> {
        #[cfg(target_os = "windows")]
        let color = functions::get_module::<Box<ITerminalColor>>(
            Box::from(WinApiColor::new()),
            Box::from(AnsiColor::new()),
        ).unwrap();

        #[cfg(not(target_os = "windows"))]
        let color = Box::from(AnsiColor::new()) as Box<ITerminalColor>;

        TerminalColor {
            color,
            screen_manager,
        }
    }

    /// Set the foreground color to the given color.
    ///
    /// #Example
    ///
    /// ```rust
    /// let crossterm = Crossterm::new();
    /// let colored_terminal = crossterm.color();
    ///
    /// // Set foreground color of the font
    /// colored_terminal.set_fg(Color::Red);
    /// // crossterm provides to set the background from &str or String
    /// colored_terminal.set_fg(Color::from("Red"));
    ///
    /// ```
    pub fn set_fg(&self, color: Color) {
        self.color.set_fg(color, &self.screen_manager);
    }

    /// Set the background color to the given color.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// let crossterm = Crossterm::new();
    /// let colored_terminal = crossterm.color();
    ///
    /// // Set background color of the font
    /// colored_terminal.set_bg(Color::Red);
    /// // crossterm provides to set the background from &str or String
    /// colored_terminal.set_bg(Color::from("Red"));
    ///
    /// ```
    pub fn set_bg(&self, color: Color) {
        self.color.set_bg(color, &self.screen_manager);
    }

    /// Reset the terminal colors and attributes to default.
    /// # Example
    ///
    /// ```rust
    ///
    /// let crossterm = Crossterm::new();
    /// let colored_terminal = crossterm.color();
    ///
    /// colored_terminal.reset();
    ///
    /// ```
    pub fn reset(&self) {
        self.color.reset(&self.screen_manager);
    }

    /// Get available color count.
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

/// Get an Terminal Color implementation whereon color related actions can be performed.
pub fn color(screen_manager: &ScreenManager) -> TerminalColor {
    TerminalColor::new(screen_manager)
}
