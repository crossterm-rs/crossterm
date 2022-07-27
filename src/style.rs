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
//! Using functions from [`Stylize`](crate::style::Stylize) on a `String` or `&'static str` to color
//! it.
//!
//! ```no_run
//! use crossterm::style::Stylize;
//!
//! println!("{}", "Red foreground color & blue background.".red().on_blue());
//! ```
//!
//! ### Attributes
//!
//! How to apply terminal attributes to text.
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
//! Using [`Stylize`](crate::style::Stylize) functions on a `String` or `&'static str` to set
//! attributes to it.
//!
//! ```no_run
//! use crossterm::style::Stylize;
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

use std::{
    env,
    fmt::{self, Display},
};

use crate::command::execute_fmt;
#[cfg(windows)]
use crate::Result;
use crate::{csi, impl_display, Command};

pub use self::{
    attributes::Attributes,
    content_style::ContentStyle,
    styled_content::StyledContent,
    stylize::Stylize,
    types::{Attribute, Color, Colored, Colors},
};

mod attributes;
mod content_style;
mod styled_content;
mod stylize;
mod sys;
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
/// use crossterm::style::{style, Stylize, Color};
///
/// let styled_content = style("Blue colored text on yellow background")
///     .with(Color::Blue)
///     .on(Color::Yellow);
///
/// println!("{}", styled_content);
/// ```
pub fn style<D: Display>(val: D) -> StyledContent<D> {
    ContentStyle::new().apply(val)
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

impl Command for SetForegroundColor {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        write!(f, csi!("{}m"), Colored::ForegroundColor(self.0))
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

impl Command for SetBackgroundColor {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        write!(f, csi!("{}m"), Colored::BackgroundColor(self.0))
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        sys::windows::set_background_color(self.0)
    }
}

/// A command that sets the the underline color.
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
pub struct SetUnderlineColor(pub Color);

impl Command for SetUnderlineColor {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        write!(f, csi!("{}m"), Colored::UnderlineColor(self.0))
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "SetUnderlineColor not supported by winapi.",
        ))
    }
}

/// A command that optionally sets the foreground and/or background color.
///
/// For example:
/// ```no_run
/// use std::io::{stdout, Write};
///
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

impl Command for SetColors {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        if let Some(color) = self.0.foreground {
            SetForegroundColor(color).write_ansi(f)?;
        }
        if let Some(color) = self.0.background {
            SetBackgroundColor(color).write_ansi(f)?;
        }
        Ok(())
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

impl Command for SetAttribute {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        write!(f, csi!("{}m"), self.0.sgr())
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

impl Command for SetAttributes {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        for attr in Attribute::iterator() {
            if self.0.has(attr) {
                SetAttribute(attr).write_ansi(f)?;
            }
        }
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        // attributes are not supported by WinAPI.
        Ok(())
    }
}

/// A command that sets a style (colors and attributes).
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetStyle(pub ContentStyle);

impl Command for SetStyle {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        if let Some(bg) = self.0.background_color {
            execute_fmt(f, SetBackgroundColor(bg)).map_err(|_| fmt::Error)?;
        }
        if let Some(fg) = self.0.foreground_color {
            execute_fmt(f, SetForegroundColor(fg)).map_err(|_| fmt::Error)?;
        }
        if let Some(ul) = self.0.underline_color {
            execute_fmt(f, SetUnderlineColor(ul)).map_err(|_| fmt::Error)?;
        }
        if !self.0.attributes.is_empty() {
            execute_fmt(f, SetAttributes(self.0.attributes)).map_err(|_| fmt::Error)?;
        }

        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        panic!("tried to execute SetStyle command using WinAPI, use ANSI instead");
    }

    #[cfg(windows)]
    fn is_ansi_code_supported(&self) -> bool {
        true
    }
}

/// A command that prints styled content.
///
/// See [`StyledContent`](struct.StyledContent.html) for more info.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Copy, Clone)]
pub struct PrintStyledContent<D: Display>(pub StyledContent<D>);

impl<D: Display> Command for PrintStyledContent<D> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let style = self.0.style();

        let mut reset_background = false;
        let mut reset_foreground = false;
        let mut reset = false;

        if let Some(bg) = style.background_color {
            execute_fmt(f, SetBackgroundColor(bg)).map_err(|_| fmt::Error)?;
            reset_background = true;
        }
        if let Some(fg) = style.foreground_color {
            execute_fmt(f, SetForegroundColor(fg)).map_err(|_| fmt::Error)?;
            reset_foreground = true;
        }
        if let Some(ul) = style.underline_color {
            execute_fmt(f, SetUnderlineColor(ul)).map_err(|_| fmt::Error)?;
            reset_foreground = true;
        }

        if !style.attributes.is_empty() {
            execute_fmt(f, SetAttributes(style.attributes)).map_err(|_| fmt::Error)?;
            reset = true;
        }

        write!(f, "{}", self.0.content())?;

        if reset {
            // NOTE: This will reset colors even though self has no colors, hence produce unexpected
            // resets.
            // TODO: reset the set attributes only.
            execute_fmt(f, ResetColor).map_err(|_| fmt::Error)?;
        } else {
            // NOTE: Since the above bug, we do not need to reset colors when we reset attributes.
            if reset_background {
                execute_fmt(f, SetBackgroundColor(Color::Reset)).map_err(|_| fmt::Error)?;
            }
            if reset_foreground {
                execute_fmt(f, SetForegroundColor(Color::Reset)).map_err(|_| fmt::Error)?;
            }
        }

        Ok(())
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
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        f.write_str(csi!("0m"))
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
pub struct Print<T: Display>(pub T);

impl<T: Display> Command for Print<T> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        write!(f, "{}", self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        panic!("tried to execute Print command using WinAPI, use ANSI instead");
    }

    #[cfg(windows)]
    fn is_ansi_code_supported(&self) -> bool {
        true
    }
}

impl<T: Display> Display for Print<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl_display!(for SetForegroundColor);
impl_display!(for SetBackgroundColor);
impl_display!(for SetColors);
impl_display!(for SetAttribute);
impl_display!(for PrintStyledContent<String>);
impl_display!(for PrintStyledContent<&'static str>);
impl_display!(for ResetColor);

/// Utility function for ANSI parsing in Color and Colored.
/// Gets the next element of `iter` and tries to parse it as a `u8`.
fn parse_next_u8<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Option<u8> {
    iter.next().and_then(|s| s.parse().ok())
}
