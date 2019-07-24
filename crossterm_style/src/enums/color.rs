use std::convert::AsRef;
use std::str::FromStr;

/// Enum with the different colors to color your test and terminal.
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum Color {
    // This resets the color.
    Reset,

    Black,
    DarkGrey,

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

    White,
    Grey,
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

impl<'a> From<&'a str> for Color {
    /// Get a `Color` from a `&str` like `Color::from("blue")`.
    fn from(src: &str) -> Self {
        src.parse().unwrap_or(Color::White)
    }
}

impl From<String> for Color {
    /// Get a `Color` from a `&str` like `Color::from(String::from(blue))`.
    fn from(src: String) -> Self {
        src.parse().unwrap_or(Color::White)
    }
}

impl FromStr for Color {
    type Err = ();

    /// Convert a `&str` to a `Color` value
    fn from_str(src: &str) -> ::std::result::Result<Self, Self::Err> {
        let src = src.to_lowercase();

        match src.as_ref() {
            "black" => Ok(Color::Black),
            "dark_grey" => Ok(Color::DarkGrey),
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
            "white" => Ok(Color::White),
            "grey" => Ok(Color::Grey),
            _ => Ok(Color::White),
        }
    }
}
