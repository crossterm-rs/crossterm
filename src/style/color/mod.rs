pub mod color;

mod ansi_color;
#[cfg(target_os = "windows")]
mod winapi_color;

use self::ansi_color::AnsiColor;
#[cfg(target_os = "windows")]
use self::winapi_color::WinApiColor;

use super::{Color, ColorType};
use ScreenManager;

use std::rc::Rc;
use std::sync::Mutex;

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
    fn set_fg(&mut self, fg_color: Color);
    /// Set the background color to the given color.
    fn set_bg(&mut self, fg_color: Color);
    /// Reset the terminal color to default.
    fn reset(&mut self);
    /// Gets an value that represents an color from the given `Color` and `ColorType`.
    fn color_value(&self, color: Color, color_type: ColorType) -> String;
}
