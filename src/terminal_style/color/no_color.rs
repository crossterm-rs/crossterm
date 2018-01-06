use Construct;
use super::color::{Color, ColorType};
use super::base_color::ITerminalColor;

/// Struct that will be instantiated when something went wrong or when an platform does not suport
/// the current concrete color inplementations.
#[derive(Debug)]
pub struct NoTerminalColor;

impl Construct for NoTerminalColor {
    fn new() -> Box<NoTerminalColor> {
        Box::from(NoTerminalColor {})
    }
}

impl ITerminalColor for NoTerminalColor {
    fn set_fg(&self, fg_color: Color) {}

    fn set_bg(&self, bg_color: Color) {}

    fn reset(&self) {}

    fn color_value(&self, color: Color, color_type: ColorType) -> String {
        String::from("0")
    }
}
