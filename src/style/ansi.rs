//! This is a ANSI specific implementation for styling related action.
//! This module is used for Windows 10 terminals and Unix terminals by default.

use std::fmt::{self, Formatter};

use crate::{
    csi,
    style::{Attribute, Attributes, Color, Colored},
};

pub(crate) fn set_fg_csi_sequence(f: &mut Formatter, fg_color: Color) -> fmt::Result {
    write!(f, csi!("{}m"), Colored::ForegroundColor(fg_color))
}

pub(crate) fn set_bg_csi_sequence(f: &mut Formatter, bg_color: Color) -> fmt::Result {
    write!(f, csi!("{}m"), Colored::BackgroundColor(bg_color))
}

pub(crate) fn set_attr_csi_sequence(f: &mut Formatter, attribute: Attribute) -> fmt::Result {
    write!(f, csi!("{}m"), attribute.sgr())
}

pub(crate) fn set_attrs_csi_sequence(f: &mut Formatter, attributes: Attributes) -> fmt::Result {
    for attr in Attribute::iterator() {
        if attributes.has(attr) {
            write!(f, csi!("{}m"), attr.sgr())?;
        }
    }
    Ok(())
}

pub(crate) const RESET_CSI_SEQUENCE: &str = csi!("0m");

impl fmt::Display for Colored {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let color;

        match *self {
            Colored::ForegroundColor(new_color) => {
                if new_color == Color::Reset {
                    return f.write_str("39");
                } else {
                    f.write_str("38;")?;
                    color = new_color;
                }
            }
            Colored::BackgroundColor(new_color) => {
                if new_color == Color::Reset {
                    return f.write_str("49");
                } else {
                    f.write_str("48;")?;
                    color = new_color;
                }
            }
        }

        match color {
            Color::Black => f.write_str("5;0"),
            Color::DarkGrey => f.write_str("5;8"),
            Color::Red => f.write_str("5;9"),
            Color::DarkRed => f.write_str("5;1"),
            Color::Green => f.write_str("5;10"),
            Color::DarkGreen => f.write_str("5;2"),
            Color::Yellow => f.write_str("5;11"),
            Color::DarkYellow => f.write_str("5;3"),
            Color::Blue => f.write_str("5;12"),
            Color::DarkBlue => f.write_str("5;4"),
            Color::Magenta => f.write_str("5;13"),
            Color::DarkMagenta => f.write_str("5;5"),
            Color::Cyan => f.write_str("5;14"),
            Color::DarkCyan => f.write_str("5;6"),
            Color::White => f.write_str("5;15"),
            Color::Grey => f.write_str("5;7"),
            Color::Rgb { r, g, b } => write!(f, "2;{};{};{}", r, g, b),
            Color::AnsiValue(val) => write!(f, "5;{}", val),
            _ => Ok(()),
        }
    }
}

/// Utility function for ANSI parsing in Color and Colored.
/// Gets the next element of `iter` and tries to parse it as a u8.
fn parse_next_u8<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Option<u8> {
    iter.next()
        .and_then(|s| u8::from_str_radix(s, 10).map(Some).unwrap_or(None))
}

impl Colored {
    /// Parse an ANSI foreground or background color.
    /// This is the string that would appear within an `ESC [ <str> m` escape sequence, as found in
    /// various configuration files.
    ///
    /// For example: `38;5;0 -> ForegroundColor(Black)`,
    ///              `38;5;26 -> ForegroundColor(AnsiValue(26))`
    ///              `48;2;50;60;70 -> BackgroundColor(Rgb(50, 60, 70))`
    ///              `49 -> BackgroundColor(Reset)`
    /// Invalid sequences map to None.
    ///
    /// Currently, 3/4 bit color values aren't supported so return None.
    ///
    /// See also: [Color::parse_ansi](enum.Color.html#method.parse_ansi)
    pub fn parse_ansi(ansi: &str) -> Option<Self> {
        use Colored::{BackgroundColor, ForegroundColor};

        let values = &mut ansi.split(';');

        let output = match parse_next_u8(values)? {
            38 => return Color::parse_ansi_iter(values).map(ForegroundColor),
            48 => return Color::parse_ansi_iter(values).map(BackgroundColor),

            39 => ForegroundColor(Color::Reset),
            49 => BackgroundColor(Color::Reset),

            _ => return None,
        };

        if values.next().is_some() {
            return None;
        }

        Some(output)
    }
}

impl<'a> Color {
    /// Parses an ANSI color sequence.
    /// For example: `5;0 -> Black`, `5;26 -> AnsiValue(26)`, `2;50;60;70 -> Rgb(50, 60, 70)`.
    /// Invalid sequences map to None.
    ///
    /// Currently, 3/4 bit color values aren't supported so return None.
    ///
    /// See also: [Colored::parse_ansi](enum.Colored.html#method.parse_ansi)
    pub fn parse_ansi(ansi: &str) -> Option<Self> {
        Self::parse_ansi_iter(&mut ansi.split(';'))
    }

    /// The logic for parse_ansi, takes an iterator of the sequences terms (the numbers between the
    /// ';'). It's a separate function so it can be used by both Color::parse_ansi and
    /// colored::parse_ansi.
    /// Tested in Colored tests.
    fn parse_ansi_iter(values: &mut impl Iterator<Item = &'a str>) -> Option<Self> {
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

#[cfg(test)]
mod tests {
    use crate::style::{Color, Colored};

    #[test]
    fn test_format_fg_color() {
        let colored = Colored::ForegroundColor(Color::Red);
        assert_eq!(colored.to_string(), "38;5;9");
    }

    #[test]
    fn test_format_bg_color() {
        let colored = Colored::BackgroundColor(Color::Red);
        assert_eq!(colored.to_string(), "48;5;9");
    }

    #[test]
    fn test_format_reset_fg_color() {
        let colored = Colored::ForegroundColor(Color::Reset);
        assert_eq!(colored.to_string(), "39");
    }

    #[test]
    fn test_format_reset_bg_color() {
        let colored = Colored::BackgroundColor(Color::Reset);
        assert_eq!(colored.to_string(), "49");
    }

    #[test]
    fn test_format_fg_rgb_color() {
        let colored = Colored::BackgroundColor(Color::Rgb { r: 1, g: 2, b: 3 });
        assert_eq!(colored.to_string(), "48;2;1;2;3");
    }

    #[test]
    fn test_format_fg_ansi_color() {
        let colored = Colored::ForegroundColor(Color::AnsiValue(255));
        assert_eq!(colored.to_string(), "38;5;255");
    }

    #[test]
    fn test_parse_ansi_fg() {
        test_parse_ansi(Colored::ForegroundColor)
    }

    #[test]
    fn test_parse_ansi_bg() {
        test_parse_ansi(Colored::ForegroundColor)
    }

    /// Used for test_parse_ansi_fg and test_parse_ansi_bg
    fn test_parse_ansi(bg_or_fg: impl Fn(Color) -> Colored) {
        /// Formats a re-parses `color` to check the result.
        macro_rules! test {
            ($color:expr) => {
                let colored = bg_or_fg($color);
                assert_eq!(Colored::parse_ansi(&format!("{}", colored)), Some(colored));
            };
        }

        use Color::*;

        test!(Reset);
        test!(Black);
        test!(DarkGrey);
        test!(Red);
        test!(DarkRed);
        test!(Green);
        test!(DarkGreen);
        test!(Yellow);
        test!(DarkYellow);
        test!(Blue);
        test!(DarkBlue);
        test!(Magenta);
        test!(DarkMagenta);
        test!(Cyan);
        test!(DarkCyan);
        test!(White);
        test!(Grey);

        // n in 0..=15 will give us the color values above back.
        for n in 16..=255 {
            test!(AnsiValue(n));
        }

        for r in 0..=255 {
            for g in [0, 2, 18, 19, 60, 100, 200, 250, 254, 255].iter().copied() {
                for b in [0, 12, 16, 99, 100, 161, 200, 255].iter().copied() {
                    test!(Rgb { r, g, b });
                }
            }
        }
    }

    #[test]
    fn test_parse_invalid_ansi_color() {
        /// Checks that trying to parse `s` yields None.
        fn test(s: &str) {
            assert_eq!(Colored::parse_ansi(s), None);
        }
        test("");
        test(";");
        test(";;");
        test(";;");
        test("0");
        test("1");
        test("12");
        test("100");
        test("100048949345");
        test("39;");
        test("49;");
        test("39;2");
        test("49;2");
        test("38");
        test("38;");
        test("38;0");
        test("38;5");
        test("38;5;0;");
        test("38;5;0;2");
        test("38;5;80;");
        test("38;5;80;2");
        test("38;5;257");
        test("38;2");
        test("38;2;");
        test("38;2;0");
        test("38;2;0;2");
        test("38;2;0;2;257");
        test("38;2;0;2;25;");
        test("38;2;0;2;25;3");
        test("48");
        test("48;");
        test("48;0");
        test("48;5");
        test("48;5;0;");
        test("48;5;0;2");
        test("48;5;80;");
        test("48;5;80;2");
        test("48;5;257");
        test("48;2");
        test("48;2;");
        test("48;2;0");
        test("48;2;0;2");
        test("48;2;0;2;257");
        test("48;2;0;2;25;");
        test("48;2;0;2;25;3");
    }
}
