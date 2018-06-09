pub mod color;

#[cfg(target_os = "windows")]
mod winapi_color;
mod ansi_color;

#[cfg(target_os = "windows")]
use self::winapi_color::WinApiColor;
use self::ansi_color::AnsiColor;
use super::{Color, ColorType};

use std::rc::Rc;
use std::sync::Mutex;

use { Terminal, ScreenManager };

///! This trait defines the actions that can be preformed with the terminal color.
///! This trait can be implemented so that an concrete implementation of the ITerminalColor can forfill
///! the wishes to work on an specific platform.
///!
///! ## For example:
///!
///! This trait is implemented for `WINAPI` (Windows specific) and `ANSI` (Unix specific),
///! so that color related actions can be preformed on both unix and windows systems.
pub trait ITerminalColor {
    /// Set the foreground color to the given color.
    fn set_fg(&self, fg_color: Color, screen_manager: Rc<Mutex<ScreenManager>>);
    /// Set the background color to the given color.
    fn set_bg(&self, fg_color: Color, screen_manager: Rc<Mutex<ScreenManager>>);
    /// Reset the terminal color to default.
    fn reset(&self,screen_manager: Rc<Mutex<ScreenManager>>);
    /// Gets an value that represents an color from the given `Color` and `ColorType`.
    fn color_value(&self, color: Color, color_type: ColorType) -> String;
}