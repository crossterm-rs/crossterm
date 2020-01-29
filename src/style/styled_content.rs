//! This module contains the logic to style some content.

use std::{
    fmt::{self, Display, Formatter},
    result,
};

use crate::{
    queue,
    style::{
        Attribute, Color, Colorize, ContentStyle, ResetColor, SetAttribute, SetBackgroundColor,
        SetForegroundColor, Styler,
    },
};

/// The style with the content to be styled.
///
/// # Examples
///
/// ```rust
/// use crossterm::style::{style, Color, Attribute};
///
/// let styled = style("Hello there")
///     .with(Color::Yellow)
///     .on(Color::Blue)
///     .attribute(Attribute::Bold);
///
/// println!("{}", styled);
/// ```
#[derive(Clone, Debug)]
pub struct StyledContent<D: Display> {
    /// The style (colors, content attributes).
    style: ContentStyle,
    /// A content to apply the style on.
    content: D,
}

impl<D: Display> StyledContent<D> {
    /// Creates a new `StyledContent`.
    #[inline]
    pub fn new(style: ContentStyle, content: D) -> StyledContent<D> {
        StyledContent { style, content }
    }

    /// Sets the foreground color.
    #[inline]
    pub fn with(self, foreground_color: Color) -> StyledContent<D> {
        Self {
            style: self.style.foreground(foreground_color),
            ..self
        }
    }

    /// Sets the background color.
    #[inline]
    pub fn on(self, background_color: Color) -> StyledContent<D> {
        Self {
            style: self.style.background(background_color),
            ..self
        }
    }

    /// Adds the attribute.
    ///
    /// You can add more attributes by calling this method multiple times.
    #[inline]
    pub fn attribute(self, attr: Attribute) -> StyledContent<D> {
        Self {
            style: self.style.attribute(attr),
            ..self
        }
    }

    /// Returns the content.
    #[inline]
    pub fn content(&self) -> &D {
        &self.content
    }

    /// Returns the style.
    #[inline]
    pub fn style(&self) -> &ContentStyle {
        &self.style
    }

    /// Returns a mutable reference to the style, so that it can be futher
    /// manipulated
    #[inline]
    pub fn style_mut(&mut self) -> &mut ContentStyle {
        &mut self.style
    }
}

impl<D: Display> Display for StyledContent<D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> result::Result<(), fmt::Error> {
        let mut reset = false;

        if let Some(bg) = self.style.background_color {
            queue!(f, SetBackgroundColor(bg)).map_err(|_| fmt::Error)?;
            reset = true;
        }
        if let Some(fg) = self.style.foreground_color {
            queue!(f, SetForegroundColor(fg)).map_err(|_| fmt::Error)?;
            reset = true;
        }

        for attr in &self.style.attributes {
            queue!(f, SetAttribute(*attr)).map_err(|_| fmt::Error)?;
            reset = true;
        }

        self.content.fmt(f)?;

        // TODO: There are specific command sequences for "reset forground
        // color (39m)" and "reset background color (49m)"; consider using
        // these.
        if reset {
            queue!(f, ResetColor).map_err(|_| fmt::Error)?;
        }

        Ok(())
    }
}

impl<D: Display + Clone> Colorize<D> for StyledContent<D> {
    // foreground colors
    def_color!(foreground_color: black => Color::Black);
    def_color!(foreground_color: dark_grey => Color::DarkGrey);
    def_color!(foreground_color: red => Color::Red);
    def_color!(foreground_color: dark_red => Color::DarkRed);
    def_color!(foreground_color: green => Color::Green);
    def_color!(foreground_color: dark_green => Color::DarkGreen);
    def_color!(foreground_color: yellow => Color::Yellow);
    def_color!(foreground_color: dark_yellow => Color::DarkYellow);
    def_color!(foreground_color: blue => Color::Blue);
    def_color!(foreground_color: dark_blue => Color::DarkBlue);
    def_color!(foreground_color: magenta => Color::Magenta);
    def_color!(foreground_color: dark_magenta => Color::DarkMagenta);
    def_color!(foreground_color: cyan => Color::Cyan);
    def_color!(foreground_color: dark_cyan => Color::DarkCyan);
    def_color!(foreground_color: white => Color::White);
    def_color!(foreground_color: grey => Color::Grey);

    // background colors
    def_color!(background_color: on_black => Color::Black);
    def_color!(background_color: on_dark_grey => Color::DarkGrey);
    def_color!(background_color: on_red => Color::Red);
    def_color!(background_color: on_dark_red => Color::DarkRed);
    def_color!(background_color: on_green => Color::Green);
    def_color!(background_color: on_dark_green => Color::DarkGreen);
    def_color!(background_color: on_yellow => Color::Yellow);
    def_color!(background_color: on_dark_yellow => Color::DarkYellow);
    def_color!(background_color: on_blue => Color::Blue);
    def_color!(background_color: on_dark_blue => Color::DarkBlue);
    def_color!(background_color: on_magenta => Color::Magenta);
    def_color!(background_color: on_dark_magenta => Color::DarkMagenta);
    def_color!(background_color: on_cyan => Color::Cyan);
    def_color!(background_color: on_dark_cyan => Color::DarkCyan);
    def_color!(background_color: on_white => Color::White);
    def_color!(background_color: on_grey => Color::Grey);
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

        assert_eq!(styled_content.style.foreground_color, Some(Color::Green));
        assert_eq!(styled_content.style.background_color, Some(Color::Magenta));
        assert_eq!(styled_content.style.attributes.len(), 2);
        assert_eq!(styled_content.style.attributes[0], Attribute::Reset);
        assert_eq!(styled_content.style.attributes[1], Attribute::NoItalic);
    }
}
