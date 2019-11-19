//! This is a ANSI specific implementation for styling related action.
//! This module is used for Windows 10 terminals and Unix terminals by default.

use crate::{
    csi,
    style::{Attribute, Color, Colored},
};

pub(crate) fn set_fg_csi_sequence(fg_color: Color) -> String {
    format!(
        csi!("{}m"),
        Into::<String>::into(Colored::ForegroundColor(fg_color))
    )
}

pub(crate) fn set_bg_csi_sequence(bg_color: Color) -> String {
    format!(
        csi!("{}m"),
        Into::<String>::into(Colored::BackgroundColor(bg_color))
    )
}

pub(crate) fn set_attr_csi_sequence(attribute: Attribute) -> String {
    format!(csi!("{}m"), attribute as i16)
}

pub(crate) const RESET_CSI_SEQUENCE: &str = csi!("0m");

impl From<Colored> for String {
    fn from(colored: Colored) -> Self {
        let mut ansi_value = String::new();

        let color;

        match colored {
            Colored::ForegroundColor(new_color) => {
                if new_color == Color::Reset {
                    ansi_value.push_str("39");
                    return ansi_value;
                } else {
                    ansi_value.push_str("38;");
                    color = new_color;
                }
            }
            Colored::BackgroundColor(new_color) => {
                if new_color == Color::Reset {
                    ansi_value.push_str("49");
                    return ansi_value;
                } else {
                    ansi_value.push_str("48;");
                    color = new_color;
                }
            }
        }

        let color_val = match color {
            Color::Black => "5;0",
            Color::DarkGrey => "5;8",
            Color::Red => "5;9",
            Color::DarkRed => "5;1",
            Color::Green => "5;10",
            Color::DarkGreen => "5;2",
            Color::Yellow => "5;11",
            Color::DarkYellow => "5;3",
            Color::Blue => "5;12",
            Color::DarkBlue => "5;4",
            Color::Magenta => "5;13",
            Color::DarkMagenta => "5;5",
            Color::Cyan => "5;14",
            Color::DarkCyan => "5;6",
            Color::White => "5;15",
            Color::Grey => "5;7",
            Color::Rgb { r, g, b } => {
                ansi_value.push_str(format!("2;{};{};{}", r, g, b).as_str());
                ""
            }
            Color::AnsiValue(val) => {
                ansi_value.push_str(format!("5;{}", val).as_str());
                ""
            }
            _ => "",
        };

        ansi_value.push_str(color_val);
        ansi_value
    }
}

#[cfg(test)]
mod tests {
    use crate::style::{Color, Colored};

    #[test]
    fn test_parse_fg_color() {
        let colored = Colored::ForegroundColor(Color::Red);
        assert_eq!(Into::<String>::into(colored), "38;5;9");
    }

    #[test]
    fn test_parse_bg_color() {
        let colored = Colored::BackgroundColor(Color::Red);
        assert_eq!(Into::<String>::into(colored), "48;5;9");
    }

    #[test]
    fn test_parse_reset_fg_color() {
        let colored = Colored::ForegroundColor(Color::Reset);
        assert_eq!(Into::<String>::into(colored), "39");
    }

    #[test]
    fn test_parse_reset_bg_color() {
        let colored = Colored::BackgroundColor(Color::Reset);
        assert_eq!(Into::<String>::into(colored), "49");
    }

    #[test]
    fn test_parse_fg_rgb_color() {
        let colored = Colored::BackgroundColor(Color::Rgb { r: 1, g: 2, b: 3 });
        assert_eq!(Into::<String>::into(colored), "48;2;1;2;3");
    }

    #[test]
    fn test_parse_fg_ansi_color() {
        let colored = Colored::ForegroundColor(Color::AnsiValue(255));
        assert_eq!(Into::<String>::into(colored), "38;5;255");
    }
}
