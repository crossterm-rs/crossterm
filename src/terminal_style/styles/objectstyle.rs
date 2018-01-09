use terminal_style::{Color, StyledObject};
use std::fmt::Display;

/// This struct contains the style properties that can be applied to an displayable object.
#[derive(Clone)]
pub struct ObjectStyle {
    pub fg_color: Option<Color>,
    pub bg_color: Option<Color>,
}

impl Default for ObjectStyle {
    fn default() -> ObjectStyle {
        ObjectStyle {
            fg_color: Some(Color::White),
            bg_color: Some(Color::Black),
        }
    }
}

impl ObjectStyle {
    /// Apply an `StyledObject` to the passed displayable object.
    pub fn apply_to<D>(&self, val: D) -> StyledObject<D>
    where
        D: Display,
    {
        StyledObject {
            object_style: self.clone(),
            content: val,
        }
    }

    /// Get an new instance of `ObjectStyle`
    pub fn new() -> ObjectStyle {
        return ObjectStyle {
            fg_color: None,
            bg_color: None,
        };
    }

    /// Set the background color of `ObjectStyle` to the passed color.
    pub fn bg(mut self, color: Color) -> ObjectStyle {
        self.bg_color = Some(color);
        self
    }

    /// Set the foreground color of `ObjectStyle` to the passed color.
    pub fn fg(mut self, color: Color) -> ObjectStyle {
        self.fg_color = Some(color);
        self
    }
}
