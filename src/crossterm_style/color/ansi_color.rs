use std::io;
use std::io::Write;
use std::string::String;

use Construct;
use super::color::{Color, ColorType};
use super::base_color::ITerminalColor;


/// This struct is an ansi implementation for color related actions.
#[derive(Debug)]
pub struct ANSIColor;

impl Construct for ANSIColor {
    fn new() -> Box<ANSIColor> {
        Box::from(ANSIColor {})
    }
}

impl ITerminalColor for ANSIColor {
    fn set_fg(&self, fg_color: Color) {

        let mut some_writer = io::stdout();
        write!(&mut some_writer, csi!("38;5;{}m"), self.color_value(fg_color, ColorType::Foreground));
    }

    fn set_bg(&self, bg_color: Color) {
        let mut some_writer = io::stdout();
        write!(&mut some_writer, csi!("48;5;{}m"), self.color_value(bg_color, ColorType::Background));
    }

    fn reset(&self) {
        let mut some_writer = io::stdout();
        write!(&mut some_writer, csi!("0m"));
    }

    fn color_value(&self, color: Color, color_type: ColorType) -> String {
        
        // Construct ANSI escape color code string. ;1 is for the brightness
        match color {
            Color::Black => "0",
            Color::Red => "9",
            Color::DarkRed =>"1",
            Color::Green => "10",
            Color::DarkGreen =>  "2",
            Color::Yellow => "11",
            Color::DarkYellow => "3",
            Color::Blue => "12",
            Color::DarkBlue => "4",
            Color::Magenta => "13",
            Color::DarkMagenta => "5",
            Color::Cyan => "14",
            Color::DarkCyan => "6",
            Color::Grey =>  "15",
            Color::White => "7",
        }.to_string()
    }
}
