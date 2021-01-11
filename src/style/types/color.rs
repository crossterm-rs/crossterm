use std::{convert::AsRef, convert::TryFrom, result::Result, str::FromStr};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::style::parse_next_u8;

/// Represents a color.
///
/// # Platform-specific Notes
///
/// The following list of 16 base colors are available for almost all terminals (Windows 7 and 8 included).
///
/// | Light | Dark |
/// | :--| :--   |
/// | `Grey` | `Black` |
/// | `Red` | `DarkRed` |
/// | `Green` | `DarkGreen` |
/// | `Yellow` | `DarkYellow` |
/// | `Blue` | `DarkBlue` |
/// | `Magenta` | `DarkMagenta` |
/// | `Cyan` | `DarkCyan` |
/// | `White` | `DarkWhite` |
///
/// Most UNIX terminals and Windows 10 consoles support additional colors.
/// See [`Color::Rgb`] or [`Color::AnsiValue`] for more info.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Color {
    /// Resets the terminal color.
    Reset,

    /// Black color.
    Black,

    /// Dark grey color.
    DarkGrey,

    /// Light red color.
    Red,

    /// Dark red color.
    DarkRed,

    /// Light green color.
    Green,

    /// Dark green color.
    DarkGreen,

    /// Light yellow color.
    Yellow,

    /// Dark yellow color.
    DarkYellow,

    /// Light blue color.
    Blue,

    /// Dark blue color.
    DarkBlue,

    /// Light magenta color.
    Magenta,

    /// Dark magenta color.
    DarkMagenta,

    /// Light cyan color.
    Cyan,

    /// Dark cyan color.
    DarkCyan,

    /// White color.
    White,

    /// Grey color.
    Grey,

    /// An RGB color. See [RGB color model](https://en.wikipedia.org/wiki/RGB_color_model) for more info.
    ///
    /// Most UNIX terminals and Windows 10 supported only.
    /// See [Platform-specific notes](enum.Color.html#platform-specific-notes) for more info.
    Rgb { r: u8, g: u8, b: u8 },

    /// An ANSI color. See [256 colors - cheat sheet](https://jonasjacek.github.io/colors/) for more info.
    ///
    /// Most UNIX terminals and Windows 10 supported only.
    /// See [Platform-specific notes](enum.Color.html#platform-specific-notes) for more info.
    AnsiValue(u8),
}

impl Color {
    /// Parses an ANSI color sequence.
    ///
    /// # Examples
    ///
    /// ```
    /// use crossterm::style::Color;
    ///
    /// assert_eq!(Color::parse_ansi("5;0"), Some(Color::Black));
    /// assert_eq!(Color::parse_ansi("5;26"), Some(Color::AnsiValue(26)));
    /// assert_eq!(Color::parse_ansi("2;50;60;70"), Some(Color::Rgb { r: 50, g: 60, b: 70 }));
    /// assert_eq!(Color::parse_ansi("invalid color"), None);
    /// ```
    ///
    /// Currently, 3/4 bit color values aren't supported so return `None`.
    ///
    /// See also: [`Colored::parse_ansi`](crate::style::Colored::parse_ansi).
    pub fn parse_ansi(ansi: &str) -> Option<Self> {
        Self::parse_ansi_iter(&mut ansi.split(';'))
    }

    /// The logic for parse_ansi, takes an iterator of the sequences terms (the numbers between the
    /// ';'). It's a separate function so it can be used by both Color::parse_ansi and
    /// colored::parse_ansi.
    /// Tested in Colored tests.
    pub(crate) fn parse_ansi_iter<'a>(values: &mut impl Iterator<Item = &'a str>) -> Option<Self> {
        let color = match parse_next_u8(values)? {
            // 8 bit colors: `5;<n>`
            5 => {
                let n = parse_next_u8(values)?;

                use Color::*;
                [
                    Black,       // 0
                    DarkRed,     // 1
                    DarkGreen,   // 2
                    DarkYellow,  // 3
                    DarkBlue,    // 4
                    DarkMagenta, // 5
                    DarkCyan,    // 6
                    Grey,        // 7
                    DarkGrey,    // 8
                    Red,         // 9
                    Green,       // 10
                    Yellow,      // 11
                    Blue,        // 12
                    Magenta,     // 13
                    Cyan,        // 14
                    White,       // 15
                ]
                .get(n as usize)
                .copied()
                .unwrap_or(Color::AnsiValue(n))
            }

            // 24 bit colors: `2;<r>;<g>;<b>`
            2 => Color::Rgb {
                r: parse_next_u8(values)?,
                g: parse_next_u8(values)?,
                b: parse_next_u8(values)?,
            },

            _ => return None,
        };
        // If there's another value, it's unexpected so return None.
        if values.next().is_some() {
            return None;
        }
        Some(color)
    }
}

impl TryFrom<&str> for Color {
    type Error = ();

    /// Try to create a `Color` from the string representation. This returns an error if the string does not match.
    fn try_from(src: &str) -> Result<Self, Self::Error> {
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
            _ => Err(()),
        }
    }
}

impl FromStr for Color {
    type Err = ();

    /// Creates a `Color` from the string representation.
    ///
    /// # Notes
    ///
    /// * Returns `Color::White` in case of an unknown color.
    /// * Does not return `Err` and you can safely unwrap.
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        Ok(Color::try_from(src).unwrap_or(Color::White))
    }
}

impl From<(u8, u8, u8)> for Color {
    /// Creates a 'Color' from the tuple representation.
    fn from(val: (u8, u8, u8)) -> Self {
        let (r, g, b) = val;
        Self::Rgb { r, g, b }
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

    #[test]
    fn test_know_rgb_color_conversion() {
        assert_eq!(Color::from((0, 0, 0)), Color::Rgb { r: 0, g: 0, b: 0 });
        assert_eq!(
            Color::from((255, 255, 255)),
            Color::Rgb {
                r: 255,
                g: 255,
                b: 255
            }
        );
    }
}
