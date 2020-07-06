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
use crate::{impl_display, Ansi, Command};
use std::fmt;

pub use self::{
    attributes::Attributes,
    content_style::ContentStyle,
    styled_content::StyledContent,
    traits::{Colorize, Styler},
    types::{Attribute, Color, Colored, Colors},
};

#[macro_use]
mod macros;
mod ansi;
mod attributes;
mod content_style;
mod styled_content;
mod sys;
mod traits;
mod types;

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

impl_colorize!(String);
impl_colorize!(char);

// We do actually need the parentheses here because the macro doesn't work without them otherwise
// This is probably a bug somewhere in the compiler, but it isn't that big a deal.
#[allow(unused_parens)]
impl<'a> Colorize<&'a str> for &'a str {
    impl_colorize_callback!(def_color_base!((&'a str)));
}

impl_styler!(String);
impl_styler!(char);

#[allow(unused_parens)]
impl<'a> Styler<&'a str> for &'a str {
    impl_styler_callback!(def_attr_base!((&'a str)));
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
/// [`SetColors`](struct.SetColors.html) can also be used to set both the foreground and background
/// color in one command.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetForegroundColor(pub Color);

impl fmt::Display for Ansi<SetForegroundColor> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ansi::set_fg_csi_sequence(f, (self.0).0)
    }
}

impl Command for SetForegroundColor {
    type AnsiType = Ansi<Self>;

    #[inline]
    fn ansi_code(&self) -> Self::AnsiType {
        Ansi(*self)
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
/// [`SetColors`](struct.SetColors.html) can also be used to set both the foreground and background
/// color with one command.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetBackgroundColor(pub Color);

impl fmt::Display for Ansi<SetBackgroundColor> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ansi::set_bg_csi_sequence(f, (self.0).0)
    }
}

impl Command for SetBackgroundColor {
    type AnsiType = Ansi<Self>;

    #[inline]
    fn ansi_code(&self) -> Self::AnsiType {
        Ansi(*self)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::windows::set_background_color(self.0)
    }
}

/// A command that optionally sets the foreground and/or background color.
///
/// For example:
/// ```no_run
/// use std::io::{stdout, Write};
/// use crossterm::execute;
/// use crossterm::style::{Color::{Green, Black}, Colors, Print, SetColors};
///
/// execute!(
///     stdout(),
///     SetColors(Colors::new(Green, Black)),
///     Print("Hello, world!".to_string()),
/// ).unwrap();
/// ```
///
/// See [`Colors`](struct.Colors.html) for more info.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetColors(pub Colors);

impl fmt::Display for Ansi<SetColors> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(color) = (self.0).0.foreground {
            ansi::set_fg_csi_sequence(f, color)?;
        }
        if let Some(color) = (self.0).0.background {
            ansi::set_bg_csi_sequence(f, color)?;
        }
        Ok(())
    }
}

impl Command for SetColors {
    type AnsiType = Ansi<Self>;

    #[inline]
    fn ansi_code(&self) -> Self::AnsiType {
        Ansi(*self)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        if let Some(color) = self.0.foreground {
            sys::windows::set_foreground_color(color)?;
        }
        if let Some(color) = self.0.background {
            sys::windows::set_background_color(color)?;
        }
        Ok(())
    }
}

/// A command that sets an attribute.
///
/// See [`Attribute`](enum.Attribute.html) for more info.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetAttribute(pub Attribute);

impl fmt::Display for Ansi<SetAttribute> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ansi::set_attr_csi_sequence(f, (self.0).0)
    }
}

impl Command for SetAttribute {
    type AnsiType = Ansi<Self>;

    #[inline]
    fn ansi_code(&self) -> Self::AnsiType {
        Ansi(*self)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        // attributes are not supported by WinAPI.
        Ok(())
    }
}

/// A command that sets several attributes.
///
/// See [`Attributes`](struct.Attributes.html) for more info.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetAttributes(pub Attributes);

impl fmt::Display for Ansi<SetAttributes> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ansi::set_attrs_csi_sequence(f, (self.0).0)
    }
}

impl Command for SetAttributes {
    type AnsiType = Ansi<Self>;

    fn ansi_code(&self) -> Self::AnsiType {
        Ansi(*self)
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResetColor;

impl Command for ResetColor {
    type AnsiType = &'static str;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::RESET_CSI_SEQUENCE
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::windows::reset()
    }
}

/// A command that prints the given displayable type.
///
/// Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
impl_display!(for SetColors);
impl_display!(for SetAttribute);
impl_display!(for PrintStyledContent<String>);
impl_display!(for PrintStyledContent<&'static str>);
impl_display!(for ResetColor);
