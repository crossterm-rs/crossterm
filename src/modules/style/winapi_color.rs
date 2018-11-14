//! This is an `WINAPI` specific implementation for styling related action.
//! This module is used for non supporting `ANSI` windows terminals.
//!
//! Windows versions lower then windows 10 are not supporting ANSI codes. Those versions will use this implementation instead.

use super::*;
use kernel::windows_kernel::{csbi, kernel};
use winapi::um::wincon;

/// This struct is an windows implementation for color related actions.
pub struct WinApiColor
{
    original_color: u16
}

impl WinApiColor {
    pub fn new() -> WinApiColor {
        WinApiColor { original_color: csbi::get_original_console_color()}
    }
}

impl ITerminalColor for WinApiColor {
    fn set_fg(&self, fg_color: Color, _stdout: &Option<&Arc<TerminalOutput>>) {
        let color_value = &self.color_value(fg_color, ColorType::Foreground);

        let csbi = csbi::get_csbi().unwrap();

        // Notice that the color values are stored in wAttribute.
        // So we need to use bitwise operators to check if the values exists or to get current console colors.
        let mut color: u16;
        let attrs = csbi.wAttributes;
        let bg_color = attrs & 0x0070;
        color = color_value.parse::<u16>().unwrap() | bg_color;

        // background intensity is a separate value in attrs,
        // wee need to check if this was applied to the current bg color.
        if (attrs & wincon::BACKGROUND_INTENSITY as u16) != 0 {
            color = color | wincon::BACKGROUND_INTENSITY as u16;
        }

        kernel::set_console_text_attribute(color);
    }

    fn set_bg(&self, bg_color: Color, _stdout: &Option<&Arc<TerminalOutput>>) {
        let color_value = &self.color_value(bg_color, ColorType::Background);

        let (csbi, handle) = csbi::get_csbi_and_handle().unwrap();

        // Notice that the color values are stored in wAttribute.
        // So wee need to use bitwise operators to check if the values exists or to get current console colors.
        let mut color: u16;
        let attrs = csbi.wAttributes;
        let fg_color = attrs & 0x0007;
        color = fg_color | color_value.parse::<u16>().unwrap();

        // Foreground intensity is a separate value in attrs,
        // So we need to check if this was applied to the current fg color.
        if (attrs & wincon::FOREGROUND_INTENSITY as u16) != 0 {
            color = color | wincon::FOREGROUND_INTENSITY as u16;
        }

        kernel::set_console_text_attribute(color);
    }

    fn reset(&self, _stdout: &Option<&Arc<TerminalOutput>>) {
        kernel::set_console_text_attribute(self.original_color);
    }

    /// This will get the winapi color value from the Color and ColorType struct
    fn color_value(&self, color: Color, color_type: ColorType) -> String {
        use style::{Color, ColorType};

        let winapi_color: u16;

        let fg_green = wincon::FOREGROUND_GREEN;
        let fg_red = wincon::FOREGROUND_RED;
        let fg_blue = wincon::FOREGROUND_BLUE;
        let fg_intensity = wincon::FOREGROUND_INTENSITY;

        let bg_green = wincon::BACKGROUND_GREEN;
        let bg_red = wincon::BACKGROUND_RED;
        let bg_blue = wincon::BACKGROUND_BLUE;
        let bg_intensity = wincon::BACKGROUND_INTENSITY;

        match color_type {
            ColorType::Foreground => {
                winapi_color = match color {
                    Color::Black => 0,
                    Color::Red => fg_intensity | fg_red,
                    Color::DarkRed => fg_red,
                    Color::Green => fg_intensity | fg_green,
                    Color::DarkGreen => fg_green,
                    Color::Yellow => fg_intensity | fg_green | fg_red,
                    Color::DarkYellow => fg_green | fg_red,
                    Color::Blue => fg_intensity | fg_blue,
                    Color::DarkBlue => fg_blue,
                    Color::Magenta => fg_intensity | fg_red | fg_blue,
                    Color::DarkMagenta => fg_red | fg_blue,
                    Color::Cyan => fg_intensity | fg_green | fg_blue,
                    Color::DarkCyan => fg_green | fg_blue,
                    Color::Grey => fg_intensity,
                    Color::White => fg_intensity | fg_red | fg_green | fg_blue,
                };
            }
            ColorType::Background => {
                winapi_color = match color {
                    Color::Black => 0,
                    Color::Red => bg_intensity | bg_red,
                    Color::DarkRed => bg_red,
                    Color::Green => bg_intensity | bg_green,
                    Color::DarkGreen => bg_green,
                    Color::Yellow => bg_intensity | bg_green | bg_red,
                    Color::DarkYellow => bg_green | bg_red,
                    Color::Blue => bg_intensity | bg_blue,
                    Color::DarkBlue => bg_blue,
                    Color::Magenta => bg_intensity | bg_red | bg_blue,
                    Color::DarkMagenta => bg_red | bg_blue,
                    Color::Cyan => bg_intensity | bg_green | bg_blue,
                    Color::DarkCyan => bg_green | bg_blue,
                    Color::Grey => bg_intensity,
                    Color::White => bg_intensity | bg_red | bg_green | bg_blue,
                };
            }
        };

        winapi_color.to_string()
    }
}
