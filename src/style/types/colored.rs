use crate::style::Color;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents a foreground or background color.
///
/// This can be converted to a [Colors](struct.Colors.html) struct and then applied using the
/// [SetColors](struct.SetColors.html) command, for example
/// `stdout.execute(SetColors(colored.into()))`.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Colored {
    /// A foreground color.
    ForegroundColor(Color),
    /// A background color.
    BackgroundColor(Color),
}
