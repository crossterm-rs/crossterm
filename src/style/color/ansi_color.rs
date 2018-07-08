//! This is an ANSI specific implementation for styling related action.
//! This module is used for windows 10 terminals and unix terminals by default.

use super::super::{Color, ColorType};
use super::ITerminalColor;
use ScreenManager;

use std::rc::Rc;
use std::sync::Mutex;

/// This struct is an ansi implementation for color related actions.
pub struct AnsiColor
{
    screen_manager: Rc<Mutex<ScreenManager>>
}

impl AnsiColor {
    pub fn new(screen_manager: Rc<Mutex<ScreenManager>>) -> Box<AnsiColor> {
        Box::from(AnsiColor { screen_manager })
    }
}

impl ITerminalColor for AnsiColor {
    fn set_fg(&mut self, fg_color: Color ) {
        let mx_guard = &self.screen_manager;
        let mut screen = mx_guard.lock().unwrap();

        screen.write_string(format!(
            csi!("{}m"),
            self.color_value(fg_color, ColorType::Foreground)
        ));
    }

    fn set_bg(&mut self, bg_color: Color) {
        let mx_guard = &self.screen_manager;
        let mut screen = mx_guard.lock().unwrap();

        screen.write_string(format!(
            csi!("{}m"),
            self.color_value(bg_color, ColorType::Background)
        ));
    }

    fn reset(&mut self) {
        let mut screen = self.screen_manager.lock().unwrap();
        {
            screen.write_str(csi!("0m"));
        }
    }

    fn color_value(&self, color: Color, color_type: ColorType) -> String {
        let mut ansi_value = String::new();

        match color_type {
            ColorType::Foreground => ansi_value.push_str("38;"),
            ColorType::Background => ansi_value.push_str("48;"),
        }

        #[cfg(unix)]
        let rgb_val: String;

        let color_val = match color {
            Color::Black => "5;0",
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
            Color::Grey => "5;15",
            Color::White => "5;7",
            #[cfg(unix)]
            Color::Rgb { r, g, b } => {
                rgb_val = format!("2;{};{};{}", r, g, b);
                rgb_val.as_str()
            }
            #[cfg(unix)]
            Color::AnsiValue(val) => {
                rgb_val = format!("5;{}", val);
                rgb_val.as_str()
            }
        };

        ansi_value.push_str(color_val);
        ansi_value
    }
}
