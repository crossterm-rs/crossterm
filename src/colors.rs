//! # Colors
//!
//! The `colors` module provides functionality to query the terminal for colors
//! and color-related capabilities.
//!
//! * [`query_terminal_colors`] — fetch the RGB values of foreground, background,
//!   cursor, or palette colors.
//! * [`query_color_scheme`] — detect whether the terminal is in light or dark mode.
//!
//! Color scheme *change* notifications are delivered as
//! [`Event::ColorSchemeChanged`](crate::event::Event::ColorSchemeChanged) when
//! [`EnableColorSchemeDetection`](crate::event::EnableColorSchemeDetection) is active.

pub(crate) mod sys;

pub use sys::{query_color_scheme, query_terminal_colors};

/// Terminal color type, used in queries and responses.
///
/// `Palette(n)` uses OSC 4. The remaining variants use OSC 10..=19.
#[derive(Debug, PartialOrd, PartialEq, Hash, Clone, Copy, Eq)]
pub enum ColorType {
    Palette(u8),
    Foreground,
    Background,
    Cursor,
    PointerForeground,
    PointerBackground,
    TektronixForeground,
    TektronixBackground,
    HighlightBackground,
    TektronixCursor,
    HighlightForeground,
}

impl ColorType {
    /// Maps an OSC number (10..=19) to the corresponding `ColorType` variant.
    pub(crate) fn from_osc_number(n: u8) -> Option<Self> {
        match n {
            10 => Some(Self::Foreground),
            11 => Some(Self::Background),
            12 => Some(Self::Cursor),
            13 => Some(Self::PointerForeground),
            14 => Some(Self::PointerBackground),
            15 => Some(Self::TektronixForeground),
            16 => Some(Self::TektronixBackground),
            17 => Some(Self::HighlightBackground),
            18 => Some(Self::TektronixCursor),
            19 => Some(Self::HighlightForeground),
            _ => None,
        }
    }

    /// Returns the OSC number for this color type.
    pub(crate) fn osc_number(&self) -> u8 {
        match self {
            Self::Palette(_) => 4,
            Self::Foreground => 10,
            Self::Background => 11,
            Self::Cursor => 12,
            Self::PointerForeground => 13,
            Self::PointerBackground => 14,
            Self::TektronixForeground => 15,
            Self::TektronixBackground => 16,
            Self::HighlightBackground => 17,
            Self::TektronixCursor => 18,
            Self::HighlightForeground => 19,
        }
    }
}

/// The terminal's color scheme preference (dark or light).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialOrd, Ord, PartialEq, Hash, Clone, Copy, Eq)]
pub enum ColorScheme {
    Dark,
    Light,
}

/// A parsed color response from the terminal.
#[derive(Debug, PartialOrd, PartialEq, Hash, Clone, Eq)]
pub(crate) struct ColorEntry {
    pub color_type: ColorType,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
