//! A module that contains all the actions related to the styling of the terminal.
//! Like applying attributes to font and changing the foreground and background.

use super::*;
use std::io;
use Screen;

/// Struct that stores a platform-specific implementation for color related actions.
///
/// For styling text use the `::crossterm::style()` function. `TerminalColor` will set the colors of the screen permanently and the `style()` will only style the text given.
///
/// Check `/examples/color` in the library for more specific examples.
///
///
/// ```rust
/// use crossterm::style::color;
///
/// let colored_terminal = color();
///
/// // set foreground color
/// colored_terminal.set_fg(Color::Red);
/// // set background color
/// colored_terminal.set_bg(Color::Red);
/// // reset color to default
/// colored_terminal.reset();
/// ```
///
/// When you want to use 'color' on 'alternate screen' use the `Screen` type instead and pass it to the `color::from_screen()` function.
/// By doing that styling actions will be performed on the alternate screen.
pub struct TerminalColor<'stdout> {
    color: Box<ITerminalColor + Sync + Send>,
    stdout: Option<&'stdout Arc<TerminalOutput>>,
}

impl<'stdout> TerminalColor<'stdout> {
    /// Create new instance whereon color related actions can be performed.
    pub fn new() -> TerminalColor<'stdout> {
        #[cfg(target_os = "windows")]
        let color = functions::get_module::<Box<ITerminalColor + Sync + Send>>(
            Box::from(WinApiColor::new()),
            Box::from(AnsiColor::new()),
        )
        .unwrap();

        #[cfg(not(target_os = "windows"))]
        let color = Box::from(AnsiColor::new()) as Box<ITerminalColor + Sync + Send>;

        TerminalColor {
            color,
            stdout: None,
        }
    }

    /// Create a new instance of `TerminalColor` whereon coloring could be preformed on the given output.
    ///
    /// **Note**
    ///
    /// Use this function when you want your terminal to operate with a specific output.
    /// This could be useful when you have a screen which is in 'alternate mode'.
    /// And you want your actions from the `TerminalColor`, created by this function, to operate on the 'alternate screen'.
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
        #[cfg(target_os = "windows")]
        let color = functions::get_module::<Box<ITerminalColor + Sync + Send>>(
            Box::from(WinApiColor::new()),
            Box::from(AnsiColor::new()),
        )
        .unwrap();

        #[cfg(not(target_os = "windows"))]
        let color = Box::from(AnsiColor::new()) as Box<ITerminalColor + Sync + Send>;

        TerminalColor {
            color,
            stdout: Some(stdout),
        }
    }

    /// Set the foreground color to the given color.
    ///
    /// ```rust
    /// let colored_terminal = color();
    ///
    /// // Set foreground color of the font
    /// colored_terminal.set_fg(Color::Red);
    /// // crossterm provides to set the background from &str or String
    /// colored_terminal.set_fg(Color::from("Red"));
    /// ```
    pub fn set_fg(&self, color: Color) {
        self.color.set_fg(color, &self.stdout);
    }

    /// Set the background color to the given color.
    ///
    /// ```rust
    /// let colored_terminal = color();
    ///
    /// // Set background color of the font
    /// colored_terminal.set_bg(Color::Red);
    /// // crossterm provides to set the background from &str or String
    /// colored_terminal.set_bg(Color::from("Red"));
    /// ```
    pub fn set_bg(&self, color: Color) {
        self.color.set_bg(color, &self.stdout);
    }

    /// Reset the terminal colors and attributes to default.
    ///
    /// ```rust
    /// let colored_terminal = color();
    /// colored_terminal.reset();
    /// ```
    pub fn reset(&self) {
        self.color.reset(&self.stdout);
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

/// Get a `TerminalColor` implementation whereon color related actions can be performed.
pub fn color<'stdout>() -> TerminalColor<'stdout> {
    TerminalColor::new()
}

/// Get a `TerminalColor` instance whereon color related actions can be performed.
/// Pass the reference to any `Screen` you want this type to perform actions on.
pub fn from_screen(screen: &Screen) -> TerminalColor {
    TerminalColor::from_output(&screen.stdout)
}
