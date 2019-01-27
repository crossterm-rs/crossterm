//! A module that contains all the actions related to the styling of the terminal.
//! Like applying attributes to font and changing the foreground and background.

#[macro_use]
extern crate crossterm_utils;
#[cfg(target_os = "windows")]
extern crate crossterm_winapi;

mod color;
pub mod objectstyle;
pub mod styledobject;

mod ansi_color;
#[cfg(target_os = "windows")]
mod winapi_color;

use self::ansi_color::AnsiColor;
#[cfg(target_os = "windows")]
use self::winapi_color::WinApiColor;

use std::convert::From;
use std::fmt::Display;
use std::str::FromStr;
use std::sync::Arc;

pub use self::color::{color, TerminalColor};
pub use self::objectstyle::ObjectStyle;
pub use self::styledobject::DisplayableObject;
pub use self::styledobject::StyledObject;
use crossterm_utils::{Result, TerminalOutput};

/// This trait defines the actions that can be preformed with terminal color.
/// This trait can be implemented so that a concrete implementation of the ITerminalColor can fulfill
/// the wishes to work on an specific platform.
///
/// ## For example:
///
/// This trait is implemented for `WinApi` (Windows specific) and `ANSI` (Unix specific),
/// so that color-related actions can be performed on both UNIX and Windows systems.
trait ITerminalColor {
    /// Set the foreground color to the given color.
    fn set_fg(&self, fg_color: Color, stdout: &Option<&Arc<TerminalOutput>>) -> Result<()>;
    /// Set the background color to the given color.
    fn set_bg(&self, fg_color: Color, stdout: &Option<&Arc<TerminalOutput>>) -> Result<()>;
    /// Reset the terminal color to default.
    fn reset(&self, stdout: &Option<&Arc<TerminalOutput>>) -> Result<()>;
    /// Gets an value that represents an color from the given `Color` and `ColorType`.
    fn color_value(&self, color: Color, color_type: ColorType) -> String;
}

/// This could be used to style a type who is implementing `Display` with colors and attributes.
///
/// # Example
/// ```rust
/// // get an styled object which could be painted to the terminal.
/// let styled_object = style("Some Blue colored text on black background")
///     .with(Color::Blue)
///     .on(Color::Black);
///
/// // print the styled font * times to the current screen.
/// for i in 1..10
/// {
///     println!("{}", styled_object);
/// }
/// ```
pub fn style<'a, D: 'a>(val: D) -> StyledObject<D>
where
    D: Display,
{
    ObjectStyle::new().apply_to(val)
}

/// Attributes that could be applied on some text. (*nix values)
#[cfg(unix)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum Attribute {
    Bold = 1,
    Dim = 2,
    Italic = 3,
    Underlined = 4,
    SlowBlink = 5,
    RapidBlink = 6,
    Reverse = 7,
    Hidden = 8,
    CrossedOut = 9,
}

/// Attributes that could be applied on some text. (Windows specific)
#[cfg(windows)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum Attribute {
    Reset = 0,
    Bold = 1,
    Underlined = 4,
    NoUnderline = 24,
    Negative = 7,
    Positive = 27,
}

/// Colors that are available for coloring the terminal font.
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
    /// Color representing RGB-colors;
    /// r = red
    /// g = green
    /// b = blue
    Rgb {
        r: u8,
        g: u8,
        b: u8,
    },
    AnsiValue(u8),
}

/// Color types that can be used to determine if the Color enum is a Fore- or Background Color.
#[derive(Debug, Copy, Clone)]
pub enum ColorType {
    Background,
    Foreground,
}

impl<'a> From<&'a str> for Color {
    /// Get an color from an &str like `Color::from("blue")`.
    fn from(src: &str) -> Self {
        src.parse().unwrap_or(Color::White)
    }
}

impl From<String> for Color {
    /// Get an color from an &str like `Color::from(String::from(blue))`.
    fn from(src: String) -> Self {
        src.parse().unwrap_or(Color::White)
    }
}

impl FromStr for Color {
    type Err = ();

    /// Convert a string to an Color value
    fn from_str(src: &str) -> ::std::result::Result<Self, Self::Err> {
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
