use crate::style::Color;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents a foreground or a background color.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Colored {
    /// A foreground color.
    ForegroundColor(Color),
    /// A background color.
    BackgroundColor(Color),
}
