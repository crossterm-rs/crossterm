use std::convert::AsRef;
use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Enum with the different colors to color your test and terminal.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
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

impl FromStr for Color {
    type Err = ();

    /// Creates a `Color` from the string representation.
    ///
    /// # Remarks
    ///
    /// * `Color::White` is returned in case of an unknown color.
    /// * This function does not return `Err` and you can safely unwrap.
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

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn test_known_color_conversion() {
        assert_eq!("black".parse(), Ok(Color::Black));
        assert_eq!("dark_grey".parse(), Ok(Color::DarkGrey));
        assert_eq!("red".parse(), Ok(Color::Red));
        assert_eq!("dark_red".parse(), Ok(Color::DarkRed));
        assert_eq!("green".parse(), Ok(Color::Green));
        assert_eq!("dark_green".parse(), Ok(Color::DarkGreen));
        assert_eq!("yellow".parse(), Ok(Color::Yellow));
        assert_eq!("dark_yellow".parse(), Ok(Color::DarkYellow));
        assert_eq!("blue".parse(), Ok(Color::Blue));
        assert_eq!("dark_blue".parse(), Ok(Color::DarkBlue));
        assert_eq!("magenta".parse(), Ok(Color::Magenta));
        assert_eq!("dark_magenta".parse(), Ok(Color::DarkMagenta));
        assert_eq!("cyan".parse(), Ok(Color::Cyan));
        assert_eq!("dark_cyan".parse(), Ok(Color::DarkCyan));
        assert_eq!("white".parse(), Ok(Color::White));
        assert_eq!("grey".parse(), Ok(Color::Grey));
    }

    #[test]
    fn test_unknown_color_conversion_yields_white() {
        assert_eq!("foo".parse(), Ok(Color::White));
    }
}
