//! With this module you can perform actions that are color related.
//! Like styling the font, foreground color and background color.

use std::ops::Drop;
use std::fmt;
use std::io;

use {Construct, Context };
use crossterm_style::{ObjectStyle, StyledObject};
use super::base_color::ITerminalColor;
use shared::functions::get_module;
use super::super::Color;

use super::AnsiColor;

#[cfg(target_os = "windows")]
use super::WinApiColor;

/// Struct that stores an specific platform implementation for color related actions. 
pub struct TerminalColor {
    terminal_color: Option<Box<ITerminalColor>>,
    context: Context
}

impl TerminalColor {
    /// Create new instance whereon color related actions can be performed.
    pub fn new() -> TerminalColor {
        let mut context = Context::new();

        #[cfg(target_os = "windows")]
        let color = get_module::<Box<ITerminalColor>>(WinApiColor::new(), AnsiColor::new(), &mut context);

        #[cfg(not(target_os = "windows"))]
        let color = Some(AnsiColor::new());

        TerminalColor { terminal_color: color, context: context}
    }

    /// Set the foreground color to the given color.
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate crossterm;
    ///
    /// use self::crossterm::crossterm_style::{ color, Color};
    ///
    /// // Get colored terminal instance
    /// let mut colored_terminal = color();
    /// 
    /// // Set foreground color of the font
    /// colored_terminal.set_fg(Color::Red);
    /// // crossterm provides to set the background from &str or String
    /// colored_terminal.set_fg(Color::from("Red"));
    ///
    /// ```
    pub fn set_fg(&mut self, color: Color) {
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
    /// use self::crossterm::crossterm_style::{ color, Color};
    ///
    /// // Get colored terminal instance
    /// let mut colored_terminal = color();
    /// 
    /// // Set background color of the font
    /// colored_terminal.set_bg(Color::Red);
    /// // crossterm provides to set the background from &str or String
    /// colored_terminal.set_bg(Color::from("Red"));
    ///
    /// ```
    pub fn set_bg(&mut self, color: Color) {
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
    /// use self::crossterm::crossterm_style::color;
    ///
    /// // Get colored terminal instance
    /// let mut colored_terminal = color();
    /// 
    /// colored_terminal.reset();
    ///
    /// ```
    pub fn reset(&mut self) {
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

/// Get an TerminalColor implementation whereon color related actions can be performed.
///
/// # Example
///
/// ```rust
/// extern crate crossterm;
///
/// use self::crossterm::crossterm_style::{color, Color};
/// 
/// // Get colored terminal instance
/// let mut colored_terminal = color();
///
/// // preform some actions on the colored terminal
/// colored_terminal.set_fg(Color::Red);
/// colored_terminal.set_bg(Color::Blue);
/// colored_terminal.reset();
/// ```
pub fn color() -> Box<TerminalColor> {
    Box::from(TerminalColor::new())
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


