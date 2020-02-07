//! This module contains the `content style` that can be applied to an `styled content`.

use std::fmt::Display;

use crate::style::{Attribute, Attributes, Color, StyledContent};

/// The style that can be put on content.
#[derive(Debug, Clone, Default)]
pub struct ContentStyle {
    /// The foreground color.
    pub foreground_color: Option<Color>,
    /// The background color.
    pub background_color: Option<Color>,
    /// List of attributes.
    pub attributes: Attributes,
}

impl ContentStyle {
    /// Creates a `StyledContent` by applying the style to the given `val`.
    #[inline]
    pub fn apply<D: Display>(self, val: D) -> StyledContent<D> {
        StyledContent::new(self, val)
    }

    /// Creates a new `ContentStyle`.
    #[inline]
    pub fn new() -> ContentStyle {
        ContentStyle::default()
    }

    /// Sets the background color.
    #[inline]
    pub fn background(self, color: Color) -> ContentStyle {
        Self {
            background_color: Some(color),
            ..self
        }
    }

    /// Sets the foreground color.
    #[inline]
    pub fn foreground(self, color: Color) -> ContentStyle {
        Self {
            foreground_color: Some(color),
            ..self
        }
    }

    /// Adds the attribute.
    ///
    /// You can add more attributes by calling this method multiple times.
    #[inline]
    pub fn attribute(mut self, attr: Attribute) -> ContentStyle {
        self.attributes.set(attr);
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::style::{Attribute, Color, ContentStyle};

    #[test]
    fn test_set_fg_bg_add_attr() {
        let content_style = ContentStyle::new()
            .foreground(Color::Blue)
            .background(Color::Red)
            .attribute(Attribute::Bold);

        assert_eq!(content_style.foreground_color, Some(Color::Blue));
        assert_eq!(content_style.background_color, Some(Color::Red));
        assert!(content_style.attributes.has(Attribute::Bold));
    }

    #[test]
    fn test_apply_content_style_to_text() {
        let content_style = ContentStyle::new()
            .foreground(Color::Blue)
            .background(Color::Red)
            .attribute(Attribute::Reset);

        let styled_content = content_style.apply("test");

        assert_eq!(styled_content.style().foreground_color, Some(Color::Blue));
        assert_eq!(styled_content.style().background_color, Some(Color::Red));
        assert!(styled_content.style().attributes.has(Attribute::Reset));
    }
}
