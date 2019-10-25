//! This module contains the `content style` that can be applied to an `styled content`.

use std::fmt::Display;

use super::{Attribute, Color, StyledContent};

/// A content style.
#[derive(Debug, Clone, Default)]
pub struct ContentStyle {
    /// The foreground color.
    pub fg_color: Option<Color>,
    /// The background color.
    pub bg_color: Option<Color>,
    /// List of attributes.
    pub attrs: Vec<Attribute>,
}

impl ContentStyle {
    /// Creates a `StyledContent` by applying the style to the given `val`.
    pub fn apply<D: Display + Clone>(&self, val: D) -> StyledContent<D> {
        StyledContent::new(self.clone(), val)
    }

    /// Creates a new `ContentStyle`.
    pub fn new() -> ContentStyle {
        ContentStyle::default()
    }

    /// Sets the background color.
    pub fn background(mut self, color: Color) -> ContentStyle {
        self.bg_color = Some(color);
        self
    }

    /// Sets the foreground color.
    pub fn foreground(mut self, color: Color) -> ContentStyle {
        self.fg_color = Some(color);
        self
    }

    /// Adds the attribute.
    ///
    /// You can add more attributes by calling this method multiple times.
    pub fn attribute(mut self, attr: Attribute) -> ContentStyle {
        self.attrs.push(attr);
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::{Attribute, Color, ContentStyle};

    #[test]
    fn test_set_fg_bg_add_attr() {
        let content_style = ContentStyle::new()
            .foreground(Color::Blue)
            .background(Color::Red)
            .attribute(Attribute::Reset);

        assert_eq!(content_style.fg_color, Some(Color::Blue));
        assert_eq!(content_style.bg_color, Some(Color::Red));
        assert_eq!(content_style.attrs[0], Attribute::Reset);
    }

    #[test]
    fn test_apply_content_style_to_text() {
        let content_style = ContentStyle::new()
            .foreground(Color::Blue)
            .background(Color::Red)
            .attribute(Attribute::Reset);

        let styled_content = content_style.apply("test");

        assert_eq!(styled_content.content_style().fg_color, Some(Color::Blue));
        assert_eq!(styled_content.content_style().bg_color, Some(Color::Red));
        assert_eq!(styled_content.content_style().attrs[0], Attribute::Reset);
    }
}
