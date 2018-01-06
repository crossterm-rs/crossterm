use winapi;
use kernel32;
use super::{handle, kernel};
use terminal_style as style;

/// This will set the forground color by the given winapi color value parsed to u16.
pub fn set_fg_color(fg_color: u16) {
    let csbi = kernel::get_console_screen_buffer_info();

    // Notice that the color values are stored in wAttribute.
    // So wee need to use bitwise operators to check if the values exists or to get current console colors.
    let mut color: u16;
    let attrs = csbi.wAttributes;
    let bg_color = attrs & 0x0070;
    color = fg_color | bg_color;

    // background intensity is a seperate value in attrs,
    // wee need to check if this was applied to the current bg color.
    if (attrs & winapi::BACKGROUND_INTENSITY as u16) != 0 {
        color = color | winapi::BACKGROUND_INTENSITY as u16;
    }

    set_console_text_attribute(color);
}

/// This will set the forground color by the given winapi color value parsed to u16.
pub fn set_bg_color(bg_color: u16) {
    let csbi = kernel::get_console_screen_buffer_info();
    // Notice that the color values are stored in wAttribute.
    // So wee need to use bitwise operators to check if the values exists or to get current console colors.
    let mut color: u16;
    let attrs = csbi.wAttributes;
    let fg_color = attrs & 0x0007;
    color = fg_color | bg_color;

    // foreground intensity is a seperate value in attrs,
    // wee need to check if this was applied to the current fg color.
    if (attrs & winapi::FOREGROUND_INTENSITY as u16) != 0 {
        color = color | winapi::FOREGROUND_INTENSITY as u16;
    }

    set_console_text_attribute(color);
}

/// This will reset the colors to the value given in u16.
pub fn reset(original_color: u16) {
    set_console_text_attribute(original_color);
}

/// This will get the winapi color value from the Color struct
pub fn winapi_color_val(color: style::Color, color_type: style::ColorType) -> u16 {
    use terminal_style::{Color, ColorType};

    let winapi_color: u32;

    let fg_green = winapi::FOREGROUND_GREEN;
    let fg_red = winapi::FOREGROUND_RED;
    let fg_blue = winapi::FOREGROUND_BLUE;
    let fg_intensity = winapi::FOREGROUND_INTENSITY;

    let bg_green = winapi::BACKGROUND_GREEN;
    let bg_red = winapi::BACKGROUND_RED;
    let bg_blue = winapi::BACKGROUND_BLUE;
    let bg_intensity = winapi::BACKGROUND_INTENSITY;

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

    winapi_color as u16
}

/// This will set the console attributes by the given value
fn set_console_text_attribute(value: u16) {
    let output_handle = handle::get_output_handle();

    unsafe {
        kernel32::SetConsoleTextAttribute(output_handle, value);
    }
}
