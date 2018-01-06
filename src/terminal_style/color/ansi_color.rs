use std::string::String;

use Construct;
use super::color::{Color, ColorType};
use super::base_color::{ITerminalColor, sum_u16_to_string};

/// This struct will be used for coloring ansi terminals with ansi escape codes.
#[derive(Debug)]
pub struct ANSIColor;

impl Construct for ANSIColor {
    fn new() -> Box<ANSIColor> {
        Box::from(ANSIColor {})
    }
}

impl ITerminalColor for ANSIColor {
    fn set_fg(&self, fg_color: Color) {
        format!(
            csi!("38;2;{}m"),
            self.color_value(fg_color, ColorType::Foreground)
        );
    }

    fn set_bg(&self, bg_color: Color) {
        format!(
            csi!("38;2;{}m"),
            self.color_value(bg_color, ColorType::Background)
        );
    }

    fn reset(&self) {
        format!(csi!("0m"));
    }

    fn color_value(&self, color: Color, color_type: ColorType) -> String {
        let mut ansi_color_code = String::new();

        // The ansi code for forground = 30 and background = 40;
        match color_type {
            ColorType::Foreground => ansi_color_code.push_str("30"),
            ColorType::Background => ansi_color_code.push_str("40"),
        }

        // Construct ANSI escape color code string. ;1 is for the brightness
        match color {
            Color::Black => {
                sum_u16_to_string(0, &mut ansi_color_code);
            }
            Color::Red => sum_u16_to_string(1, &mut ansi_color_code),
            Color::DarkRed => {
                sum_u16_to_string(1, &mut ansi_color_code);
                &ansi_color_code.push_str(";1");
            }
            Color::Green => sum_u16_to_string(2, &mut ansi_color_code),
            Color::DarkGreen => {
                sum_u16_to_string(2, &mut ansi_color_code);
                &ansi_color_code.push_str(";1");
            }
            Color::Yellow => sum_u16_to_string(3, &mut ansi_color_code),
            Color::DarkYellow => {
                sum_u16_to_string(3, &mut ansi_color_code);
                &ansi_color_code.push_str(";1");
            }
            Color::Blue => sum_u16_to_string(4, &mut ansi_color_code),
            Color::DarkBlue => {
                sum_u16_to_string(4, &mut ansi_color_code);
                &ansi_color_code.push_str(";1");
            }
            Color::Magenta => sum_u16_to_string(5, &mut ansi_color_code),
            Color::DarkMagenta => {
                sum_u16_to_string(5, &mut ansi_color_code);
                &ansi_color_code.push_str(";1");
            }
            Color::Cyan => sum_u16_to_string(6, &mut ansi_color_code),
            Color::DarkCyan => {
                sum_u16_to_string(6, &mut ansi_color_code);
                &ansi_color_code.push_str(";1");
            }
            Color::Grey => sum_u16_to_string(7, &mut ansi_color_code),
            Color::White => sum_u16_to_string(7, &mut ansi_color_code),
        }
        ansi_color_code
    }
}
