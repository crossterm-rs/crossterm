//! This module is used for styling the terminal text.
//! Under styling we can think of coloring the font and applying attributes to it.

mod color;
mod styles;

pub use self::color::color::{color, TerminalColor };

pub use self::styles::objectstyle::ObjectStyle;
pub use self::styles::styledobject::StyledObject;

use std::convert::From;
use std::str::FromStr;

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
    CrossedOut = 9
}

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

    #[cfg(unix)]
    Rgb { r: u8, g: u8, b:u8 },
    #[cfg(unix)]
    AnsiValue(u8)
}

/// Color types that can be used to determine if the Color enum is an Fore- or Background Color
#[derive(Debug, Copy, Clone)]
pub enum ColorType {
    Background,
    Foreground,
}

impl<'a> From<&'a str> for Color {
    fn from(src: &str) -> Self {
        src.parse().unwrap_or(Color::White)
    }
}

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
