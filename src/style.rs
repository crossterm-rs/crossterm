//! # Style
//!
//! The `style` module provides a functionality to apply attributes and colors on your text.
//!
//! This documentation does not contain a lot of examples. The reason is that it's fairly
//! obvious how to use this crate. Although, we do provide
//! [examples](https://github.com/crossterm-rs/examples) repository
//! to demonstrate the capabilities.
//!
//! ## Platform-specific Notes
//!
//! Not all features are supported on all terminals/platforms. You should always consult
//! platform-specific notes of the following types:
//!
//! * [Color](enum.Color.html#platform-specific-notes)
//! * [Attribute](enum.Attribute.html#platform-specific-notes)
//!
//! ## Examples
//!
//! ### Colors
//!
//! The command API:
//!
//! ```no_run
//! use std::io::{stdout, Write};
//!
//! use crossterm::{execute, Result, Output};
//! use crossterm::style::{SetForegroundColor, SetBackgroundColor, ResetColor, Color, Attribute};
//!
//! fn main() -> Result<()> {
//!     execute!(
//!         stdout(),
//!         // Blue foreground
//!         SetForegroundColor(Color::Blue),
//!         // Red background
//!         SetBackgroundColor(Color::Red),
//!         // output text
//!         Output("Styled text here.".to_string()),
//!         // Reset to default colors
//!         ResetColor
//!     )
//! }
//! ```
//!
//! The [`Colorize`](trait.Colorize.html) trait:
//!
//! ```no_run
//! use crossterm::style::Colorize;
//!
//! println!("{}", "Red foreground color & blue background.".red().on_blue());
//! ```
//!
//! ### Attributes
//!
//! The command API:
//!
//! ```no_run
//! use std::io::{stdout, Write};
//!
//! use crossterm::{execute, Result, Output};
//! use crossterm::style::{SetAttribute, Attribute};
//!
//! fn main() -> Result<()> {
//!     execute!(
//!         stdout(),
//!         // Set to bold
//!         SetAttribute(Attribute::Bold),
//!         Output("Styled text here.".to_string()),
//!         // Reset all attributes
//!         SetAttribute(Attribute::Reset)
//!     )
//! }
//! ```
//!
//! The [`Styler`](trait.Styler.html) trait:
//!
//! ```no_run
//! use crossterm::style::Styler;
//!
//! println!("{}", "Bold".bold());
//! println!("{}", "Underlined".underlined());
//! println!("{}", "Negative".negative());
//! ```
//!
//! The [`Attribute`](enum.Attribute.html) enum:
//!
//! ```no_run
//! use crossterm::style::Attribute;
//!
//! println!(
//!     "{} Underlined {} No Underline",
//!     Attribute::Underlined,
//!     Attribute::NoUnderline
//! );
//! ```

use std::env;
use std::fmt::Display;

use crate::impl_display;
use crate::utils::{Command, Result};

pub use self::content_style::ContentStyle;
pub(crate) use self::enums::Colored;
pub use self::enums::{Attribute, Color};
pub use self::styled_content::StyledContent;
pub use self::traits::{Colorize, Styler};

#[macro_use]
mod macros;
mod ansi;
mod content_style;
mod enums;
mod styled_content;
mod sys;
mod traits;

/// Creates a `StyledContent`.
///
/// This could be used to style any type that implements `Display` with colors and text attributes.
///
/// See [`StyledContent`](struct.StyledContent.html) for more info.
///
/// # Examples
///
/// ```no_run
/// use crossterm::style::{style, Color};
///
/// let styled_content = style("Blue colored text on yellow background")
///     .with(Color::Blue)
///     .on(Color::Yellow);
///
/// println!("{}", styled_content);
/// ```
pub fn style<'a, D: 'a>(val: D) -> StyledContent<D>
where
    D: Display + Clone,
{
    ContentStyle::new().apply(val)
}

impl Colorize<&'static str> for &'static str {
    // foreground colors
    def_str_color!(foreground_color: black => Color::Black);
    def_str_color!(foreground_color: dark_grey => Color::DarkGrey);
    def_str_color!(foreground_color: red => Color::Red);
    def_str_color!(foreground_color: dark_red => Color::DarkRed);
    def_str_color!(foreground_color: green => Color::Green);
    def_str_color!(foreground_color: dark_green => Color::DarkGreen);
    def_str_color!(foreground_color: yellow => Color::Yellow);
    def_str_color!(foreground_color: dark_yellow => Color::DarkYellow);
    def_str_color!(foreground_color: blue => Color::Blue);
    def_str_color!(foreground_color: dark_blue => Color::DarkBlue);
    def_str_color!(foreground_color: magenta => Color::Magenta);
    def_str_color!(foreground_color: dark_magenta => Color::DarkMagenta);
    def_str_color!(foreground_color: cyan => Color::Cyan);
    def_str_color!(foreground_color: dark_cyan => Color::DarkCyan);
    def_str_color!(foreground_color: white => Color::White);
    def_str_color!(foreground_color: grey => Color::Grey);

    // background colors
    def_str_color!(background_color: on_black => Color::Black);
    def_str_color!(background_color: on_dark_grey => Color::DarkGrey);
    def_str_color!(background_color: on_red => Color::Red);
    def_str_color!(background_color: on_dark_red => Color::DarkRed);
    def_str_color!(background_color: on_green => Color::Green);
    def_str_color!(background_color: on_dark_green => Color::DarkGreen);
    def_str_color!(background_color: on_yellow => Color::Yellow);
    def_str_color!(background_color: on_dark_yellow => Color::DarkYellow);
    def_str_color!(background_color: on_blue => Color::Blue);
    def_str_color!(background_color: on_dark_blue => Color::DarkBlue);
    def_str_color!(background_color: on_magenta => Color::Magenta);
    def_str_color!(background_color: on_dark_magenta => Color::DarkMagenta);
    def_str_color!(background_color: on_cyan => Color::Cyan);
    def_str_color!(background_color: on_dark_cyan => Color::DarkCyan);
    def_str_color!(background_color: on_white => Color::White);
    def_str_color!(background_color: on_grey => Color::Grey);
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

/// Returns available color count.
///
/// # Notes
///
/// This does not always provide a good result.
pub fn available_color_count() -> u16 {
    env::var("TERM")
        .map(|x| if x.contains("256color") { 256 } else { 8 })
        .unwrap_or(8)
}

/// A command that sets the the foreground color.
///
/// See [`Color`](enum.Color.html) for more info.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
pub struct SetForegroundColor(pub Color);

impl Command for SetForegroundColor {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::set_fg_csi_sequence(self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::windows::set_foreground_color(self.0)
    }
}

/// A command that sets the the background color.
///
/// See [`Color`](enum.Color.html) for more info.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
pub struct SetBackgroundColor(pub Color);

impl Command for SetBackgroundColor {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::set_bg_csi_sequence(self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::windows::set_background_color(self.0)
    }
}

/// A command that sets an attribute.
///
/// See [`Attribute`](enum.Attribute.html) for more info.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
pub struct SetAttribute(pub Attribute);

impl Command for SetAttribute {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::set_attr_csi_sequence(self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        // attributes are not supported by WinAPI.
        Ok(())
    }
}

/// A command that prints styled content.
///
/// See [`StyledContent`](struct.StyledContent.html) for more info.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
pub struct PrintStyledContent<D: Display + Clone>(pub StyledContent<D>);

impl<D> Command for PrintStyledContent<D>
where
    D: Display + Clone,
{
    type AnsiType = StyledContent<D>;

    fn ansi_code(&self) -> Self::AnsiType {
        self.0.clone()
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        Ok(())
    }
}

/// A command that resets the colors back to default.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
pub struct ResetColor;

impl Command for ResetColor {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::RESET_CSI_SEQUENCE.to_string()
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::windows::reset()
    }
}

impl_display!(for SetForegroundColor);
impl_display!(for SetBackgroundColor);
impl_display!(for SetAttribute);
impl_display!(for PrintStyledContent<String>);
impl_display!(for PrintStyledContent<&'static str>);
impl_display!(for ResetColor);
