use crate::style::Color;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::parse_next_u8;

/// Represents a foreground or a background color.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Colored {
    /// A foreground color.
    ForegroundColor(Color),
    /// A background color.
    BackgroundColor(Color),
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
        use Colored::{BackgroundColor as BG, ForegroundColor as FG};

        let values = &mut ansi.split(';');

        let output = match parse_next_u8(values)? {
            38 => return Color::parse_ansi_iter(values).map(FG),
            48 => return Color::parse_ansi_iter(values).map(BG),

            39 => FG(Color::Reset),
            49 => BG(Color::Reset),

            _ => return None,
        };

        if values.next().is_some() {
            return None;
        }

        Some(output)
    }
}

#[cfg(test)]
mod tests {
    use super::super::Color;
    use super::Colored;

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
    fn test_parse_ansi_fg() {
        test_parse_ansi(Colored::ForegroundColor)
    }

    #[test]
    fn test_parse_ansi_bg() {
        test_parse_ansi(Colored::ForegroundColor)
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
