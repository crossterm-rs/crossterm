//! A module that contains all the actions related to the styling of the terminal.
//! Like applying attributes to text and changing the foreground and background.

#[macro_use]
mod macros;
mod color;
mod enums;
pub mod objectstyle;
pub mod styledobject;
mod traits;

mod ansi_color;
#[cfg(windows)]
mod winapi_color;

use std::fmt::Display;

use self::ansi_color::AnsiColor;
#[cfg(windows)]
use self::winapi_color::WinApiColor;
pub use crossterm_utils::{execute, queue, Command, ExecutableCommand, QueueableCommand, Result};

pub use self::color::{color, PrintStyledFont, SetAttr, SetBg, SetFg, TerminalColor};
pub use self::enums::{Attribute, Color, Colored};
pub use self::objectstyle::ObjectStyle;
pub use self::styledobject::StyledObject;
pub use self::traits::{Colorize, Styler};

/// This trait defines the actions that can be performed with terminal colors.
/// This trait can be implemented so that a concrete implementation of the ITerminalColor can fulfill
/// the wishes to work on a specific platform.
///
/// ## For example:
///
/// This trait is implemented for `WinApi` (Windows specific) and `ANSI` (Unix specific),
/// so that color-related actions can be performed on both UNIX and Windows systems.
trait ITerminalColor {
    /// Set the foreground color to the given color.
    fn set_fg(&self, fg_color: Color) -> Result<()>;
    /// Set the background color to the given color.
    fn set_bg(&self, fg_color: Color) -> Result<()>;
    /// Reset the terminal color to default.
    fn reset(&self) -> Result<()>;
}

/// This could be used to style a type that implements `Display` with colors and attributes.
///
/// # Example
/// ```ignore
/// // get a styled object which could be painted to the terminal.
/// let styled_object = style("Some Blue colored text on black background")
///     .with(Color::Blue)
///     .on(Color::Black);
///
/// // print the styled text * times to the current screen.
/// for i in 1..10
/// {
///     println!("{}", styled_object);
/// }
/// ```
///
/// # Important Remark
///
/// - Please checkout the documentation for `Colorizer` or `Styler`.
/// Those types will make it a bit easier to style a string.
pub fn style<'a, D: 'a>(val: D) -> StyledObject<D>
where
    D: Display + Clone,
{
    ObjectStyle::new().apply_to(val)
}

impl Colorize<&'static str> for &'static str {
    // foreground colors
    def_str_color!(fg_color: black => Color::Black);
    def_str_color!(fg_color: dark_grey => Color::DarkGrey);
    def_str_color!(fg_color: red => Color::Red);
    def_str_color!(fg_color: dark_red => Color::DarkRed);
    def_str_color!(fg_color: green => Color::Green);
    def_str_color!(fg_color: dark_green => Color::DarkGreen);
    def_str_color!(fg_color: yellow => Color::Yellow);
    def_str_color!(fg_color: dark_yellow => Color::DarkYellow);
    def_str_color!(fg_color: blue => Color::Blue);
    def_str_color!(fg_color: dark_blue => Color::DarkBlue);
    def_str_color!(fg_color: magenta => Color::Magenta);
    def_str_color!(fg_color: dark_magenta => Color::DarkMagenta);
    def_str_color!(fg_color: cyan => Color::Cyan);
    def_str_color!(fg_color: dark_cyan => Color::DarkCyan);
    def_str_color!(fg_color: white => Color::White);
    def_str_color!(fg_color: grey => Color::Grey);

    // background colors
    def_str_color!(bg_color: on_black => Color::Black);
    def_str_color!(bg_color: on_dark_grey => Color::DarkGrey);
    def_str_color!(bg_color: on_red => Color::Red);
    def_str_color!(bg_color: on_dark_red => Color::DarkRed);
    def_str_color!(bg_color: on_green => Color::Green);
    def_str_color!(bg_color: on_dark_green => Color::DarkGreen);
    def_str_color!(bg_color: on_yellow => Color::Yellow);
    def_str_color!(bg_color: on_dark_yellow => Color::DarkYellow);
    def_str_color!(bg_color: on_blue => Color::Blue);
    def_str_color!(bg_color: on_dark_blue => Color::DarkBlue);
    def_str_color!(bg_color: on_magenta => Color::Magenta);
    def_str_color!(bg_color: on_dark_magenta => Color::DarkMagenta);
    def_str_color!(bg_color: on_cyan => Color::Cyan);
    def_str_color!(bg_color: on_dark_cyan => Color::DarkCyan);
    def_str_color!(bg_color: on_white => Color::White);
    def_str_color!(bg_color: on_grey => Color::Grey);
}

impl Styler<&'static str> for &'static str {
    def_str_attr!(reset => Attribute::Reset);
    def_str_attr!(bold => Attribute::Bold);
    def_str_attr!(underlined => Attribute::Underlined);
    def_str_attr!(reverse => Attribute::Reverse);
    def_str_attr!(dim => Attribute::Dim);
    def_str_attr!(italic => Attribute::Italic);
    def_str_attr!(negative => Attribute::Reverse);
    def_str_attr!(slow_blink => Attribute::SlowBlink);
    def_str_attr!(rapid_blink => Attribute::RapidBlink);
    def_str_attr!(hidden => Attribute::Hidden);
    def_str_attr!(crossed_out => Attribute::CrossedOut);
}
