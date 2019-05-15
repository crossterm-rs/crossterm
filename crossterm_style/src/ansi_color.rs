//! This is a ANSI specific implementation for styling related action.
//! This module is used for Windows 10 terminals and Unix terminals by default.

use crate::{Color, ITerminalColor};
use crossterm_utils::Result;

use crate::Colored;
use std::io::Write;

/// This struct is an ANSI escape code implementation for color related actions.
pub struct AnsiColor;

impl AnsiColor {
    pub fn new() -> AnsiColor {
        AnsiColor
    }
}

impl ITerminalColor for AnsiColor {
    fn set_fg(&self, fg_color: Color) -> Result<()> {
        write_cout!(&format!(
            csi!("{}m"),
            self.color_value(Colored::Fg(fg_color)),
        ))?;
        Ok(())
    }

    fn set_bg(&self, bg_color: Color) -> Result<()> {
        write_cout!(&format!(
            csi!("{}m"),
            self.color_value(Colored::Bg(bg_color))
        ))?;
        Ok(())
    }

    fn reset(&self) -> Result<()> {
        write_cout!(csi!("0m"))?;
        Ok(())
    }

    fn color_value(&self, colored: Colored) -> String {
        let mut ansi_value = String::new();

        let color;

        match colored {
            Colored::Fg(new_color) => {
                if new_color == Color::Reset {
                    ansi_value.push_str("39");
                    return ansi_value;
                } else {
                    ansi_value.push_str("38;");
                    color = new_color;
                }
            }
            Colored::Bg(new_color) => {
                if new_color == Color::Reset {
                    ansi_value.push_str("49");
                    return ansi_value;
                } else {
                    ansi_value.push_str("48;");
                    color = new_color;
                }
            }
        }

        let rgb_val: String;

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
                rgb_val = format!("2;{};{};{}", r, g, b);
                rgb_val.as_str()
            }
            Color::AnsiValue(val) => {
                rgb_val = format!("5;{}", val);
                rgb_val.as_str()
            }
            _ => "",
        };

        ansi_value.push_str(color_val);
        ansi_value
    }
}
