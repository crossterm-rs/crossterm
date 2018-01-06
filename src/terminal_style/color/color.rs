use std::fmt;
use std::convert::From;
use std::str::FromStr;

use Construct;
use super::{ANSIColor, NoTerminalColor, WinApiColor};
use super::base_color::ITerminalColor;
use terminal_style::{ObjectStyle, StyledObject};

/// Colors that are available for coloring the termainal font.
#[derive(Debug, Copy, Clone)]
pub enum Color {
    Black,

    Red,
    DarkRed,

    Green,
    DarkGreen,

    Yellow,
    DarkYellow,

    Blue,
    DarkBlue,

    Magenta,
    DarkMagenta,

    Cyan,
    DarkCyan,

    Grey,
    White,
}

/// Color types
#[derive(Debug, Copy, Clone)]
pub enum ColorType {
    Background,
    Foreground,
}

/// Enables an user to pass in an color as str.
/// *Default color if cannot be parsed will be white.*
///
/// # Example
///
/// ``` rust
/// let fg_color = Color::from("red");
/// let bg_color = Color::from("blue");
///
/// println!("{}",paint("■").with(fg_color).on(bg_color));
/// ```
impl<'a> From<&'a str> for Color {
    fn from(src: &str) -> Self {
        src.parse().unwrap_or(Color::White)
    }
}

/// Enables an user to pass in an color as String.
/// *Default color if cannot be parsed will be white.*
///
/// # Example
///
/// ``` rust
/// let fg_color = Color::from(String::from("red"));
/// let bg_color = Color::from(String::from("blue"));
///
/// println!("{}",paint("■").with(fg_color).on(bg_color));
/// ```
impl From<String> for Color {
    fn from(src: String) -> Self {
        src.parse().unwrap_or(Color::White)
    }
}

impl FromStr for Color {
    type Err = ();

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let src = src.to_lowercase();

        match src.as_ref() {
            "black" => Ok(Color::Black),
            "red" => Ok(Color::Red),
            "dark_red" => Ok(Color::DarkRed),
            "green" => Ok(Color::Green),
            "dark_green" => Ok(Color::DarkGreen),
            "yellow" => Ok(Color::Yellow),
            "dark_yellow" => Ok(Color::DarkYellow),
            "blue" => Ok(Color::Blue),
            "dark_blue" => Ok(Color::DarkBlue),
            "magenta" => Ok(Color::Magenta),
            "dark_magenta" => Ok(Color::DarkMagenta),
            "cyan" => Ok(Color::Cyan),
            "dark_cyan" => Ok(Color::DarkCyan),
            "grey" => Ok(Color::Grey),
            "white" => Ok(Color::White),
            _ => Ok(Color::White),
        }
    }
}

/// Struct on wits the color realated actions can be performed.
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
    ///
    /// let mut colored_terminal = colored_terminal();
    /// colored_terminal.set_fg(Color::Red);
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
    /// let mut colored_terminal = colored_terminal();
    /// colored_terminal.set_bg(Color::Red);
    ///
    /// ```
    pub fn set_bg(&mut self, color: Color) {
        &self.init();
        if let Some(ref terminal_color) = self.terminal_color {
            terminal_color.set_bg(color);
        }
    }

    /// Reset the terminal colors to default.
    /// # Example
    ///
    /// ```rust
    ///
    /// let mut colored_terminal = colored_terminal();
    /// colored_terminal.reset();
    ///
    /// ```
    pub fn reset(&mut self) {
        &self.init();
        if let Some(ref terminal_color) = self.terminal_color {
            terminal_color.reset();
        }
    }
}

/// Get the concrete ITerminalColor implementation based on the current operating system.
fn get_color_options() -> Option<Box<ITerminalColor>> {
    if cfg!(target_os = "linux") {
        Some(ANSIColor::new())
    } else if cfg!(target_os = "windows") {
        Some(WinApiColor::new())
    } else {
        Some(NoTerminalColor::new())
    }
}

/// Get the terminal options for colors, whereon color related actions can be performed.
pub fn colored_terminal() -> Box<TerminalColor> {
    Box::from(TerminalColor {
        terminal_color: get_color_options(),
    })
}

/// Wraps an displayable object so it can be formatted with colors and attributes.
///
/// #Example
///
/// ```rust
/// extern crate crossterm;

/// use self::crossterm::terminal_style::{paint,Color};
///
/// fn main()
/// {
///    // default foregroundcolor and backgroundcolor.
///    println!("{}",paint("■"));
///
///    // red foregroundcolor and Blue backgroundcolor
///    let styledobject = paint("■").with(Color::Red).on(Color::Blue);
///    println!("{}", styledobject);
/// }
/// ```
pub fn paint<D>(val: D) -> StyledObject<D>
where
    D: fmt::Display,
{
    ObjectStyle::new().apply_to(val)
}
