use std::convert::AsRef;
use std::str::FromStr;

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
