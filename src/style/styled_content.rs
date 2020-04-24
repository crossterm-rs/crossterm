//! This module contains the logic to style some content.

use std::{
    fmt::{self, Display, Formatter},
    result,
};

use crate::{
    queue,
    style::{
        Attribute, Color, Colorize, ContentStyle, ResetColor, SetAttributes, SetBackgroundColor,
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

        if !self.style.attributes.is_empty() {
            queue!(f, SetAttributes(self.style.attributes)).map_err(|_| fmt::Error)?;
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
    impl_colorize_callback!(def_color_generic!(()));
}

impl<D: Display + Clone> Styler<D> for StyledContent<D> {
    impl_styler_callback!(def_attr_generic!(()));
}

#[cfg(test)]
mod tests {
    use super::{Attribute, Color, ContentStyle};

    #[test]
    fn test_set_fg_bg_add_attr() {
        let style = ContentStyle::new()
            .foreground(Color::Blue)
            .background(Color::Red)
            .attribute(Attribute::Bold);

        let mut styled_content = style.apply("test");

        styled_content = styled_content
            .with(Color::Green)
            .on(Color::Magenta)
            .attribute(Attribute::NoItalic);

        assert_eq!(styled_content.style.foreground_color, Some(Color::Green));
        assert_eq!(styled_content.style.background_color, Some(Color::Magenta));
        assert!(styled_content.style.attributes.has(Attribute::Bold));
        assert!(styled_content.style.attributes.has(Attribute::NoItalic));
    }
}
