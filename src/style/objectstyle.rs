//! This module contains the `object style` that can be applied to an `styled object`.

use std::fmt::Display;

use super::{Attribute, Color, StyledObject};

/// An object style.
#[derive(Debug, Clone, Default)]
pub struct ObjectStyle {
    /// The foreground color.
    pub fg_color: Option<Color>,
    /// The background color.
    pub bg_color: Option<Color>,
    /// List of attributes.
    pub attrs: Vec<Attribute>,
}

impl ObjectStyle {
    /// Creates a `StyledObject` by applying the style to the given `val`.
    pub fn apply_to<D: Display + Clone>(&self, val: D) -> StyledObject<D> {
        StyledObject {
            object_style: self.clone(),
            content: val,
        }
    }

    /// Creates a new `ObjectStyle`.
    pub fn new() -> ObjectStyle {
        ObjectStyle::default()
    }

    /// Sets the background color.
    pub fn bg(mut self, color: Color) -> ObjectStyle {
        self.bg_color = Some(color);
        self
    }

    /// Sets the foreground color.
    pub fn fg(mut self, color: Color) -> ObjectStyle {
        self.fg_color = Some(color);
        self
    }

    /// Adds the attribute.
    ///
    /// You can add more attributes by calling this method multiple times.
    pub fn add_attr(&mut self, attr: Attribute) {
        self.attrs.push(attr);
    }
}

#[cfg(test)]
mod tests {
    use crate::{Attribute, Color, ObjectStyle};

    #[test]
    fn test_set_fg_bg_add_attr() {
        let mut object_style = ObjectStyle::new().fg(Color::Blue).bg(Color::Red);
        object_style.add_attr(Attribute::Reset);

        assert_eq!(object_style.fg_color, Some(Color::Blue));
        assert_eq!(object_style.bg_color, Some(Color::Red));
        assert_eq!(object_style.attrs[0], Attribute::Reset);
    }

    #[test]
    fn test_apply_object_style_to_text() {
        let mut object_style = ObjectStyle::new().fg(Color::Blue).bg(Color::Red);
        object_style.add_attr(Attribute::Reset);

        let styled_object = object_style.apply_to("test");

        assert_eq!(styled_object.object_style.fg_color, Some(Color::Blue));
        assert_eq!(styled_object.object_style.bg_color, Some(Color::Red));
        assert_eq!(styled_object.object_style.attrs[0], Attribute::Reset);
    }
}
