use std::sync::Mutex;

use crossterm_winapi::{Console, Handle, HandleType, ScreenBuffer};
use winapi::um::wincon;

use lazy_static::lazy_static;

use crate::Result;

use super::super::{Color, Colored};

const FG_GREEN: u16 = wincon::FOREGROUND_GREEN;
const FG_RED: u16 = wincon::FOREGROUND_RED;
const FG_BLUE: u16 = wincon::FOREGROUND_BLUE;
const FG_INTENSITY: u16 = wincon::FOREGROUND_INTENSITY;

const BG_GREEN: u16 = wincon::BACKGROUND_GREEN;
const BG_RED: u16 = wincon::BACKGROUND_RED;
const BG_BLUE: u16 = wincon::BACKGROUND_BLUE;
const BG_INTENSITY: u16 = wincon::BACKGROUND_INTENSITY;

pub(crate) fn set_foreground_color(fg_color: Color) -> Result<()> {
    init_console_color()?;

    let color_value: u16 = Colored::ForegroundColor(fg_color).into();

    let screen_buffer = ScreenBuffer::current()?;
    let csbi = screen_buffer.info()?;

    // Notice that the color values are stored in wAttribute.
    // So we need to use bitwise operators to check if the values exists or to get current console colors.
    let mut color: u16;
    let attrs = csbi.attributes();
    let bg_color = attrs & 0x0070;
    color = color_value | bg_color;

    // background intensity is a separate value in attrs,
    // wee need to check if this was applied to the current bg color.
    if (attrs & wincon::BACKGROUND_INTENSITY as u16) != 0 {
        color |= wincon::BACKGROUND_INTENSITY as u16;
    }

    Console::from(screen_buffer.handle().clone()).set_text_attribute(color)?;
    Ok(())
}

pub(crate) fn set_background_color(bg_color: Color) -> Result<()> {
    init_console_color()?;

    let color_value: u16 = Colored::BackgroundColor(bg_color).into();

    let screen_buffer = ScreenBuffer::current()?;
    let csbi = screen_buffer.info()?;

    // Notice that the color values are stored in wAttribute.
    // So wee need to use bitwise operators to check if the values exists or to get current console colors.
    let mut color: u16;
    let attrs = csbi.attributes();
    let fg_color = attrs & 0x0007;
    color = fg_color | color_value;

    // Foreground intensity is a separate value in attrs,
    // So we need to check if this was applied to the current fg color.
    if (attrs & wincon::FOREGROUND_INTENSITY as u16) != 0 {
        color |= wincon::FOREGROUND_INTENSITY as u16;
    }

    Console::from(screen_buffer.handle().clone()).set_text_attribute(color)?;
    Ok(())
}

pub(crate) fn reset() -> Result<()> {
    let original_color = original_console_color();

    Console::from(Handle::new(HandleType::CurrentOutputHandle)?)
        .set_text_attribute(original_color)?;

    Ok(())
}

/// Initializes the default console color. It will will be skipped if it has already been initialized.
pub(crate) fn init_console_color() -> Result<()> {
    let mut locked_pos = ORIGINAL_CONSOLE_COLOR.lock().unwrap();

    if locked_pos.is_none() {
        let screen_buffer = ScreenBuffer::current()?;
        let attr = screen_buffer.info()?.attributes();
        *locked_pos = Some(attr);
    }

    Ok(())
}

/// Returns the original console color, make sure to call `init_console_color` before calling this function. Otherwise this function will panic.
pub(crate) fn original_console_color() -> u16 {
    // safe unwrap, initial console color was set with `init_console_color` in `WinApiColor::new()`
    ORIGINAL_CONSOLE_COLOR
        .lock()
        .unwrap()
        .expect("Initial console color not set")
}

lazy_static! {
    static ref ORIGINAL_CONSOLE_COLOR: Mutex<Option<u16>> = Mutex::new(None);
}

impl From<Colored> for u16 {
    /// Returns the WinApi color value (u16) from the `Colored` struct.
    fn from(colored: Colored) -> Self {
        match colored {
            Colored::ForegroundColor(color) => {
                match color {
                    Color::Black => 0,
                    Color::DarkGrey => FG_INTENSITY,
                    Color::Red => FG_INTENSITY | FG_RED,
                    Color::DarkRed => FG_RED,
                    Color::Green => FG_INTENSITY | FG_GREEN,
                    Color::DarkGreen => FG_GREEN,
                    Color::Yellow => FG_INTENSITY | FG_GREEN | FG_RED,
                    Color::DarkYellow => FG_GREEN | FG_RED,
                    Color::Blue => FG_INTENSITY | FG_BLUE,
                    Color::DarkBlue => FG_BLUE,
                    Color::Magenta => FG_INTENSITY | FG_RED | FG_BLUE,
                    Color::DarkMagenta => FG_RED | FG_BLUE,
                    Color::Cyan => FG_INTENSITY | FG_GREEN | FG_BLUE,
                    Color::DarkCyan => FG_GREEN | FG_BLUE,
                    Color::White => FG_INTENSITY | FG_RED | FG_GREEN | FG_BLUE,
                    Color::Grey => FG_RED | FG_GREEN | FG_BLUE,

                    Color::Reset => {
                        // safe unwrap, initial console color was set with `init_console_color`.
                        let original_color = original_console_color();

                        const REMOVE_BG_MASK: u16 = BG_INTENSITY | BG_RED | BG_GREEN | BG_BLUE;
                        // remove all background values from the original color, we don't want to reset those.

                        original_color & !REMOVE_BG_MASK
                    }

                    /* WinApi will be used for systems that do not support ANSI, those are windows version less then 10. RGB and 255 (AnsiBValue) colors are not supported in that case.*/
                    Color::Rgb { .. } => 0,
                    Color::AnsiValue(_val) => 0,
                }
            }
            Colored::BackgroundColor(color) => {
                match color {
                    Color::Black => 0,
                    Color::DarkGrey => BG_INTENSITY,
                    Color::Red => BG_INTENSITY | BG_RED,
                    Color::DarkRed => BG_RED,
                    Color::Green => BG_INTENSITY | BG_GREEN,
                    Color::DarkGreen => BG_GREEN,
                    Color::Yellow => BG_INTENSITY | BG_GREEN | BG_RED,
                    Color::DarkYellow => BG_GREEN | BG_RED,
                    Color::Blue => BG_INTENSITY | BG_BLUE,
                    Color::DarkBlue => BG_BLUE,
                    Color::Magenta => BG_INTENSITY | BG_RED | BG_BLUE,
                    Color::DarkMagenta => BG_RED | BG_BLUE,
                    Color::Cyan => BG_INTENSITY | BG_GREEN | BG_BLUE,
                    Color::DarkCyan => BG_GREEN | BG_BLUE,
                    Color::White => BG_INTENSITY | BG_RED | BG_GREEN | BG_BLUE,
                    Color::Grey => BG_RED | BG_GREEN | BG_BLUE,

                    Color::Reset => {
                        let original_color = original_console_color();

                        const REMOVE_FG_MASK: u16 = FG_INTENSITY | FG_RED | FG_GREEN | FG_BLUE;
                        // remove all foreground values from the original color, we don't want to reset those.

                        original_color & !REMOVE_FG_MASK
                    }
                    /* WinApi will be used for systems that do not support ANSI, those are windows version less then 10. RGB and 255 (AnsiBValue) colors are not supported in that case.*/
                    Color::Rgb { .. } => 0,
                    Color::AnsiValue(_val) => 0,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::style::sys::windows::set_foreground_color;

    use super::{
        Color, Colored, BG_INTENSITY, BG_RED, FG_INTENSITY, FG_RED, ORIGINAL_CONSOLE_COLOR,
    };

    #[test]
    fn test_parse_fg_color() {
        let colored = Colored::ForegroundColor(Color::Red);
        assert_eq!(Into::<u16>::into(colored), FG_INTENSITY | FG_RED);
    }

    #[test]
    fn test_parse_bg_color() {
        let colored = Colored::BackgroundColor(Color::Red);
        assert_eq!(Into::<u16>::into(colored), BG_INTENSITY | BG_RED);
    }

    #[test]
    fn test_original_console_color_is_set() {
        assert!(ORIGINAL_CONSOLE_COLOR.lock().unwrap().is_none());

        // will call `init_console_color`
        set_foreground_color(Color::Blue).unwrap();

        assert!(ORIGINAL_CONSOLE_COLOR.lock().unwrap().is_some());
    }
}
