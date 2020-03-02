//! This is a ANSI specific implementation for styling related action.
//! This module is used for Windows 10 terminals and Unix terminals by default.

use crate::{
    csi,
    style::{Attribute, Attributes, Color, Colored},
};

use std::fmt::{self, Formatter};

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

#[cfg(test)]
mod tests {
    use crate::style::{Color, Colored};

    #[test]
    fn test_parse_fg_color() {
        let colored = Colored::ForegroundColor(Color::Red);
        assert_eq!(colored.to_string(), "38;5;9");
    }

    #[test]
    fn test_parse_bg_color() {
        let colored = Colored::BackgroundColor(Color::Red);
        assert_eq!(colored.to_string(), "48;5;9");
    }

    #[test]
    fn test_parse_reset_fg_color() {
        let colored = Colored::ForegroundColor(Color::Reset);
        assert_eq!(colored.to_string(), "39");
    }

    #[test]
    fn test_parse_reset_bg_color() {
        let colored = Colored::BackgroundColor(Color::Reset);
        assert_eq!(colored.to_string(), "49");
    }

    #[test]
    fn test_parse_fg_rgb_color() {
        let colored = Colored::BackgroundColor(Color::Rgb { r: 1, g: 2, b: 3 });
        assert_eq!(colored.to_string(), "48;2;1;2;3");
    }

    #[test]
    fn test_parse_fg_ansi_color() {
        let colored = Colored::ForegroundColor(Color::AnsiValue(255));
        assert_eq!(colored.to_string(), "38;5;255");
    }
}
