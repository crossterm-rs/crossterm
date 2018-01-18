use Construct;
use super::color::{ColorType, Color};
use super::base_color::ITerminalColor;
use kernel::windows_kernel;

/// This struct is an windows implementation for color related actions.
#[derive(Debug)]
pub struct WinApiColor {
    original_console_color: u16,
}

impl Construct for WinApiColor {
    fn new() -> Box<WinApiColor> {
        Box::from(WinApiColor {
            original_console_color: windows_kernel::kernel::get_original_console_color(),
        })
    }
}

impl ITerminalColor for WinApiColor {
    fn set_fg(&self, fg_color: Color) {
        let color_value = &self.color_value(fg_color, ColorType::Foreground);
        windows_kernel::color::set_fg_color(color_value.parse().unwrap());
    }

    fn set_bg(&self, bg_color: Color) {
        let color_value = &self.color_value(bg_color, ColorType::Background);
        windows_kernel::color::set_bg_color(color_value.parse().unwrap());
    }

    fn reset(&self) {
        windows_kernel::color::reset(self.original_console_color);
    }

    fn color_value(&self, color: Color, color_type: ColorType) -> String {
        windows_kernel::color::winapi_color_val(color, color_type).to_string()
    }
}
