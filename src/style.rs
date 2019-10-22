//! # Style
//!
//! **The `crossterm_style` crate is deprecated and no longer maintained. The GitHub repository will
//! be archived soon. All the code is being moved to the `crossterm`
//! [crate](https://github.com/crossterm-rs/crossterm). You can learn more in
//! the [Merge sub-crates to the crossterm crate](https://github.com/crossterm-rs/crossterm/issues/265)
//! issue.**
//!
//! The `crossterm_style` crate provides a functionality to apply attributes and colors on your text.
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
//! use crossterm::{SetBg, SetFg, ResetColor, Color, Attribute};
//!
//! fn main() -> Result<()> {
//!     execute!(
//!         stdout(),
//!         // Blue foreground
//!         SetFg(Color::Blue),
//!         // Red background
//!         SetBg(Color::Red),
//!         Output("Styled text here.".to_string()),
//!         // Reset to default colors
//!         ResetColor
//!     )
//! }
//! ```
//!
//! The [`Colored`](enum.Colored.html) & [`Color`](enum.Color.html) enums:
//!
//! ```no_run
//! use crossterm::{Colored, Color};
//!
//! println!("{} Red foreground", Colored::Fg(Color::Red));
//! println!("{} Blue background", Colored::Bg(Color::Blue));
//! ```
//!
//! The [`Colorize`](trait.Colorize.html) trait:
//!
//! ```no_run
//! use crossterm::Colorize;
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
//! use crossterm::{SetAttr, Attribute};
//!
//! fn main() -> Result<()> {
//!     execute!(
//!         stdout(),
//!         // Set to bold
//!         SetAttr(Attribute::Bold),
//!         Output("Styled text here.".to_string()),
//!         // Reset all attributes
//!         SetAttr(Attribute::Reset)
//!     )
//! }
//! ```
//!
//! The [`Styler`](trait.Styler.html) trait:
//!
//! ```no_run
//! use crossterm::Styler;
//!
//! println!("{}", "Bold".bold());
//! println!("{}", "Underlined".underlined());
//! println!("{}", "Negative".negative());
//! ```
//!
//! The [`Attribute`](enum.Attribute.html) enum:
//!
//! ```no_run
//! use crossterm::Attribute;
//!
//! println!(
//!     "{} Underlined {} No Underline",
//!     Attribute::Underlined,
//!     Attribute::NoUnderline
//! );
//! ```

use std::env;
use std::fmt::Display;

use style::ansi::{self, AnsiColor};
#[cfg(windows)]
use style::winapi::WinApiColor;
use style::Style;

use crate::impl_display;
#[cfg(windows)]
use crate::utils::supports_ansi;
use crate::utils::{Command, Result};

pub use self::enums::{Attribute, Color, Colored};
pub use self::objectstyle::ObjectStyle;
pub use self::styledobject::StyledObject;
pub use self::traits::{Colorize, Styler};

#[macro_use]
mod macros;
mod enums;
mod objectstyle;
mod style;
mod styledobject;
mod traits;

/// Creates a `StyledObject`.
///
/// This could be used to style any type that implements `Display` with colors and text attributes.
///
/// See [`StyledObject`](struct.StyledObject.html) for more info.
///
/// # Examples
///
/// ```no_run
/// use crossterm::{style, Color};
///
/// let styled_object = style("Blue colored text on yellow background")
///     .with(Color::Blue)
///     .on(Color::Yellow);
///
/// println!("{}", styled_object);
/// ```
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

/// A terminal color.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// // You can replace the following line with `use crossterm::TerminalColor;`
/// // if you're using the `crossterm` crate with the `style` feature enabled.
/// use crossterm::{Result, TerminalColor, Color};
///
/// fn main() -> Result<()> {
///     let color = TerminalColor::new();
///     // Set foreground color
///     color.set_fg(Color::Blue)?;
///     // Set background color
///     color.set_bg(Color::Red)?;
///     // Reset to the default colors
///     color.reset()
/// }
/// ```
pub struct TerminalColor {
    #[cfg(windows)]
    color: Box<(dyn Style + Sync + Send)>,
    #[cfg(unix)]
    color: AnsiColor,
}

impl TerminalColor {
    /// Creates a new `TerminalColor`.
    pub fn new() -> TerminalColor {
        #[cfg(windows)]
        let color = if supports_ansi() {
            Box::from(AnsiColor::new()) as Box<(dyn Style + Sync + Send)>
        } else {
            WinApiColor::new() as Box<(dyn Style + Sync + Send)>
        };

        #[cfg(unix)]
        let color = AnsiColor::new();

        TerminalColor { color }
    }

    /// Sets the foreground color.
    pub fn set_fg(&self, color: Color) -> Result<()> {
        self.color.set_fg(color)
    }

    /// Sets the background color.
    pub fn set_bg(&self, color: Color) -> Result<()> {
        self.color.set_bg(color)
    }

    /// Resets the terminal colors and attributes to the default ones.
    pub fn reset(&self) -> Result<()> {
        self.color.reset()
    }

    /// Returns available color count.
    ///
    /// # Notes
    ///
    /// This does not always provide a good result.
    pub fn available_color_count(&self) -> u16 {
        env::var("TERM")
            .map(|x| if x.contains("256color") { 256 } else { 8 })
            .unwrap_or(8)
    }
}

/// Creates a new `TerminalColor`.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use crossterm::{color, Color, Result};
///
/// fn main() -> Result<()> {
///     let color = color();
///     // Set foreground color
///     color.set_fg(Color::Blue)?;
///     // Set background color
///     color.set_bg(Color::Red)?;
///     // Reset to the default colors
///     color.reset()
/// }
/// ```
pub fn color() -> TerminalColor {
    TerminalColor::new()
}

/// A command to set the foreground color.
///
/// See [`Color`](enum.Color.html) for more info.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
pub struct SetFg(pub Color);

impl Command for SetFg {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::set_fg_csi_sequence(self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        WinApiColor::new().set_fg(self.0)
    }
}

/// A command to set the background color.
///
/// See [`Color`](enum.Color.html) for more info.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
pub struct SetBg(pub Color);

impl Command for SetBg {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::set_bg_csi_sequence(self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        WinApiColor::new().set_bg(self.0)
    }
}

/// A command to set the text attribute.
///
/// See [`Attribute`](enum.Attribute.html) for more info.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
pub struct SetAttr(pub Attribute);

impl Command for SetAttr {
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

/// A command to print the styled object.
///
/// See [`StyledObject`](struct.StyledObject.html) for more info.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
pub struct PrintStyledFont<D: Display + Clone>(pub StyledObject<D>);

impl<D> Command for PrintStyledFont<D>
where
    D: Display + Clone,
{
    type AnsiType = StyledObject<D>;

    fn ansi_code(&self) -> Self::AnsiType {
        self.0.clone()
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        Ok(())
    }
}

/// A command to reset the colors back to default ones.
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
        WinApiColor::new().reset()
    }
}

impl_display!(for SetFg);
impl_display!(for SetBg);
impl_display!(for SetAttr);
impl_display!(for PrintStyledFont<String>);
impl_display!(for PrintStyledFont<&'static str>);
impl_display!(for ResetColor);
