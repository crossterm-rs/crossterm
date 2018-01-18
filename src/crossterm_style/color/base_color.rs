///! 
///! This trait defines the actions that can be preformed with the termial color.
///! This trait can be inplemented so that an concrete inplementation of the ITerminalColor can forfill
///! the wishes to work on an specific platform.
///!
///! ## For example:
///!
///! This trait is inplemented for winapi (Windows specific) and ansi (Unix specific),
///! so that the color related actions can be preformed on both unix and windows systems.
///!

use super::color::{Color, ColorType};

pub trait ITerminalColor {
    /// Set the forground color to the given color.
    fn set_fg(&self, fg_color: Color);
    /// Set the background color to the given color.
    fn set_bg(&self, fg_color: Color);
    /// Reset the terminal color to default.
    fn reset(&self);
    /// Gets an value that represents an color from the given `Color` and `ColorType`.
    fn color_value(&self, color: Color, color_type: ColorType) -> String;
}
