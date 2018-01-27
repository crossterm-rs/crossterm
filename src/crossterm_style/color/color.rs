//! With this module you can perform actions that are color related.
//! Like styling the font, foreground color and background color.

use std::fmt;
use std::io;

use Construct;
use crossterm_style::{ObjectStyle, StyledObject};
use super::base_color::ITerminalColor;
use super::super::Color;

#[cfg(unix)]
use super::ANSIColor;
#[cfg(windows)]
use super::WinApiColor;

/// Struct that stores an specific platform implementation for color related actions. 
pub struct TerminalColor {
    terminal_color: Option<Box<ITerminalColor>>,
}

impl TerminalColor {
    /// Instantiate an color implementation whereon color related actions can be performed.
    pub fn init(&mut self) {
        if let None = self.terminal_color {
            self.terminal_color = get_color_options();
        }
    }

    /// Set the forground color to the given color.
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate crossterm;
    ///
    /// use self::crossterm::crossterm_style::{ get, Color};
    ///
    /// // Get colored terminal instance
    /// let mut colored_terminal = get();
    /// 
    /// // Set foreground color of the font
    /// colored_terminal.set_fg(Color::Red);
    /// // crossterm provides to set the background from &str or String
    /// colored_terminal.set_fg(Color::from("Red"));
    ///
    /// ```
    pub fn set_fg(&mut self, color: Color) {
        &self.init();
        if let Some(ref terminal_color) = self.terminal_color {
            terminal_color.set_fg(color);
        }
    }

    /// Set the background color to the given color.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    ///
    /// use self::crossterm::crossterm_style::{ get, Color};
    ///
    /// // Get colored terminal instance
    /// let mut colored_terminal = get();
    /// 
    /// // Set background color of the font
    /// colored_terminal.set_bg(Color::Red);
    /// // crossterm provides to set the background from &str or String
    /// colored_terminal.set_bg(Color::from("Red"));
    ///
    /// ```
    pub fn set_bg(&mut self, color: Color) {
        &self.init();
        if let Some(ref terminal_color) = self.terminal_color {
            terminal_color.set_bg(color);
        }
    }

    /// Reset the terminal colors and attributes to default.
    /// # Example
    ///
    /// ```rust
    /// extern crate crossterm;
    ///
    /// use self::crossterm::crossterm_style::get;
    ///
    /// // Get colored terminal instance
    /// let mut colored_terminal = get();
    /// 
    /// colored_terminal.reset();
    ///
    /// ```
    pub fn reset(&mut self) {
        &self.init();
        if let Some(ref terminal_color) = self.terminal_color {
            terminal_color.reset();
        }
    }

    /// Get available color count.
    pub fn get_available_color_count(&self) -> io::Result<u16>
    {
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

/// Get an concrete ITerminalColor implementation based on the current operating system.
fn get_color_options() -> Option<Box<ITerminalColor>> {
    #[cfg(unix)]
    return Some(ANSIColor::new());
    #[cfg(windows)]
    return Some(WinApiColor::new());
}

/// Get an TerminalColor implementation whereon color related actions can be performed.
///
/// # Example
///
/// ```rust
/// extern crate crossterm;
///
/// use self::crossterm::crossterm_style::{get, Color};
/// 
/// // Get colored terminal instance
/// let mut colored_terminal = get();
///
/// // preform some actions on the colored terminal
/// colored_terminal.set_fg(Color::Red);
/// colored_terminal.set_bg(Color::Blue);
/// colored_terminal.reset();
/// ```
pub fn get() -> Box<TerminalColor> {
    Box::from(TerminalColor {
        terminal_color: get_color_options(),
    })
}

/// Wraps an displayable object so it can be formatted with colors and attributes.
///
/// Check `/examples/color` in the libary for more spesific examples.
/// 
/// #Example
///
/// ```rust
/// extern crate crossterm;
///
/// use self::crossterm::crossterm_style::{paint,Color};
///
/// fn main()
/// {
///     // Create an styledobject object from the text 'Unstyled font' 
///     // Currently it has the default foregroundcolor and backgroundcolor.
///     println!("{}",paint("Unstyled font"));
///
///     // Create an displayable object from the text 'Colored font', 
///     // Paint this with the `Red` foreground color and `Blue` backgroundcolor.
///     // Print the result.
///     let styledobject = paint("Colored font").with(Color::Red).on(Color::Blue);
///     println!("{}", styledobject);
///    
///     // Or all in one line
///     println!("{}", paint("Colored font").with(Color::Red).on(Color::Blue));
/// }
/// ```
pub fn paint<D>(val: D) -> StyledObject<D>
where
    D: fmt::Display,
{
    ObjectStyle::new().apply_to(val)
}


