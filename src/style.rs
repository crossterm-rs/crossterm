//! # Style
//!
//! The `style` module provides a functionality to apply attributes and colors on your text.
//!
//! This documentation does not contain a lot of examples. The reason is that it's fairly
//! obvious how to use this crate. Although, we do provide
//! [examples](https://github.com/crossterm-rs/crossterm/tree/master/examples) repository
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
//! A few examples of how to use the style module.
//!
//! ### Colors
//!
//! How to change the terminal text color.
//!
//! Command API:
//!
//! Using the Command API to color text.
//!
//! ```no_run
//! use std::io::{stdout, Write};
//!
//! use crossterm::{execute, Result};
//! use crossterm::style::{Print, SetForegroundColor, SetBackgroundColor, ResetColor, Color, Attribute};
//!
//! fn main() -> Result<()> {
//!     execute!(
//!         stdout(),
//!         // Blue foreground
//!         SetForegroundColor(Color::Blue),
//!         // Red background
//!         SetBackgroundColor(Color::Red),
//!         // Print text
//!         Print("Blue text on Red.".to_string()),
//!         // Reset to default colors
//!         ResetColor
//!     )
//! }
//! ```
//!
//! Functions:
//!
//! Using functions from [`Colorize`](trait.Colorize.html) on a `String` or `&'static str` to color it.
//!
//! ```no_run
//! use crossterm::style::Colorize;
//!
//! println!("{}", "Red foreground color & blue background.".red().on_blue());
//! ```
//!
//! ### Attributes
//!
//! How to appy terminal attributes to text.
//!
//! Command API:
//!
//! Using the Command API to set attributes.
//!
//! ```no_run
//! use std::io::{stdout, Write};
//!
//! use crossterm::{execute, Result, style::Print};
//! use crossterm::style::{SetAttribute, Attribute};
//!
//! fn main() -> Result<()> {
//!     execute!(
//!         stdout(),
//!         // Set to bold
//!         SetAttribute(Attribute::Bold),
//!         Print("Bold text here.".to_string()),
//!         // Reset all attributes
//!         SetAttribute(Attribute::Reset)
//!     )
//! }
//! ```
//!
//! Functions:
//!
//! Using [`Styler`](trait.Styler.html) functions on a `String` or `&'static str` to set attributes to it.
//!
//! ```no_run
//! use crossterm::style::Styler;
//!
//! println!("{}", "Bold".bold());
//! println!("{}", "Underlined".underlined());
//! println!("{}", "Negative".negative());
//! ```
//!
//! Displayable:
//!
//! [`Attribute`](enum.Attribute.html) implements [Display](https://doc.rust-lang.org/beta/std/fmt/trait.Display.html) and therefore it can be formatted like:
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

use std::{env, fmt::Display};

#[cfg(windows)]
use crate::Result;
use crate::{impl_display, Command};

pub(crate) use self::enums::Colored;
pub use self::{
    content_style::ContentStyle,
    enums::{Attribute, Color},
    styled_content::StyledContent,
    traits::{Colorize, Styler},
};

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

/// A command that prints the given displayable type.
///
/// Commands must be executed/queued for execution otherwise they do nothing.
pub struct Print<T: Display + Clone>(pub T);

impl<T: Display + Clone> Command for Print<T> {
    type AnsiType = T;

    fn ansi_code(&self) -> Self::AnsiType {
        self.0.clone()
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        print!("{}", self.0);
        Ok(())
    }
}

impl<T: Display + Clone> Display for Print<T> {
    fn fmt(
        &self,
        f: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        write!(f, "{}", self.ansi_code())
    }
}

impl_display!(for SetForegroundColor);
impl_display!(for SetBackgroundColor);
impl_display!(for SetAttribute);
impl_display!(for PrintStyledContent<String>);
impl_display!(for PrintStyledContent<&'static str>);
impl_display!(for ResetColor);
