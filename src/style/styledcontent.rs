//! This module contains the logic to style some content.

use std::fmt::{self, Display, Formatter};
use std::result;

use crate::queue;

use super::{Attribute, Color, Colorize, ContentStyle, ResetColor, SetAttr, SetBg, SetFg, Styler};

/// A styled content.
///
/// # Examples
///
/// ```rust
/// use crossterm::{style, Color, Attribute};
///
/// let styled = style("Hello there")
///     .with(Color::Yellow)
///     .on(Color::Blue)
///     .attribute(Attribute::Bold);
///
/// println!("{}", styled);
/// ```
#[derive(Clone)]
pub struct StyledContent<D: Display + Clone> {
    /// The style (colors, content attributes).
    style: ContentStyle,
    /// A content to apply the style on.
    content: D,
}

impl<'a, D: Display + 'a + Clone> StyledContent<D> {
    /// Creates a new `StyledContent`.
    pub fn new(style: ContentStyle, content: D) -> StyledContent<D> {
        StyledContent { style, content }
    }

    /// Sets the foreground color.
    pub fn with(mut self, foreground_color: Color) -> StyledContent<D> {
        self.style = self.style.foreground(foreground_color);
        self
    }

    /// Sets the background color.
    pub fn on(mut self, background_color: Color) -> StyledContent<D> {
        self.style = self.style.background(background_color);
        self
    }

    /// Adds the attribute.
    ///
    /// You can add more attributes by calling this method multiple times.
    pub fn attribute(mut self, attr: Attribute) -> StyledContent<D> {
        self.style = self.style.attribute(attr);
        self
    }

    /// Returns the content.
    pub fn content(&self) -> &D {
        &self.content
    }

    /// Returns the style.
    pub fn style(&self) -> &ContentStyle {
        &self.style
    }
}

impl<D: Display + Clone> Display for StyledContent<D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> result::Result<(), fmt::Error> {
        let mut reset = false;

        if let Some(bg) = self.style.bg_color {
            queue!(f, SetBg(bg)).map_err(|_| fmt::Error)?;
            reset = true;
        }
        if let Some(fg) = self.style.fg_color {
            queue!(f, SetFg(fg)).map_err(|_| fmt::Error)?;
            reset = true;
        }

        for attr in self.style.attrs.iter() {
            queue!(f, SetAttr(*attr)).map_err(|_| fmt::Error)?;
            reset = true;
        }

        fmt::Display::fmt(&self.content, f)?;

        if reset {
            queue!(f, ResetColor).map_err(|_| fmt::Error)?;
        }

        Ok(())
    }
}

impl<D: Display + Clone> Colorize<D> for StyledContent<D> {
    // foreground colors
    def_color!(fg_color: black => Color::Black);
    def_color!(fg_color: dark_grey => Color::DarkGrey);
    def_color!(fg_color: red => Color::Red);
    def_color!(fg_color: dark_red => Color::DarkRed);
    def_color!(fg_color: green => Color::Green);
    def_color!(fg_color: dark_green => Color::DarkGreen);
    def_color!(fg_color: yellow => Color::Yellow);
    def_color!(fg_color: dark_yellow => Color::DarkYellow);
    def_color!(fg_color: blue => Color::Blue);
    def_color!(fg_color: dark_blue => Color::DarkBlue);
    def_color!(fg_color: magenta => Color::Magenta);
    def_color!(fg_color: dark_magenta => Color::DarkMagenta);
    def_color!(fg_color: cyan => Color::Cyan);
    def_color!(fg_color: dark_cyan => Color::DarkCyan);
    def_color!(fg_color: white => Color::White);
    def_color!(fg_color: grey => Color::Grey);

    // background colors
    def_color!(bg_color: on_black => Color::Black);
    def_color!(bg_color: on_dark_grey => Color::DarkGrey);
    def_color!(bg_color: on_red => Color::Red);
    def_color!(bg_color: on_dark_red => Color::DarkRed);
    def_color!(bg_color: on_green => Color::Green);
    def_color!(bg_color: on_dark_green => Color::DarkGreen);
    def_color!(bg_color: on_yellow => Color::Yellow);
    def_color!(bg_color: on_dark_yellow => Color::DarkYellow);
    def_color!(bg_color: on_blue => Color::Blue);
    def_color!(bg_color: on_dark_blue => Color::DarkBlue);
    def_color!(bg_color: on_magenta => Color::Magenta);
    def_color!(bg_color: on_dark_magenta => Color::DarkMagenta);
    def_color!(bg_color: on_cyan => Color::Cyan);
    def_color!(bg_color: on_dark_cyan => Color::DarkCyan);
    def_color!(bg_color: on_white => Color::White);
    def_color!(bg_color: on_grey => Color::Grey);
}

impl<D: Display + Clone> Styler<D> for StyledContent<D> {
    def_attr!(reset => Attribute::Reset);
    def_attr!(bold => Attribute::Bold);
    def_attr!(underlined => Attribute::Underlined);
    def_attr!(reverse => Attribute::Reverse);
    def_attr!(dim => Attribute::Dim);
    def_attr!(italic => Attribute::Italic);
    def_attr!(negative => Attribute::Reverse);
    def_attr!(slow_blink => Attribute::SlowBlink);
    def_attr!(rapid_blink => Attribute::RapidBlink);
    def_attr!(hidden => Attribute::Hidden);
    def_attr!(crossed_out => Attribute::CrossedOut);
}

#[cfg(test)]
mod tests {
    use super::{Attribute, Color, ContentStyle};

    #[test]
    fn test_set_fg_bg_add_attr() {
        let style = ContentStyle::new()
            .foreground(Color::Blue)
            .background(Color::Red)
            .attribute(Attribute::Reset);

        let mut styled_content = style.apply("test");

        styled_content = styled_content
            .with(Color::Green)
            .on(Color::Magenta)
            .attribute(Attribute::NoItalic);

        assert_eq!(styled_content.style.fg_color, Some(Color::Green));
        assert_eq!(styled_content.style.bg_color, Some(Color::Magenta));
        assert_eq!(styled_content.style.attrs.len(), 2);
        assert_eq!(styled_content.style.attrs[0], Attribute::Reset);
        assert_eq!(styled_content.style.attrs[1], Attribute::NoItalic);
    }
}
