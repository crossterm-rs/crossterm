use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{color::Color, super::color};

/// Represents a foreground or a background color.
///
/// Can be used to easily change the text colors.
///
/// # Examples
///
/// `Colored` implements `Display` therefore you can use it in any `write` operation.
///
/// ```no_run
/// use crossterm::{Colored, Color};
///
/// println!("{} Red foreground color", Colored::Fg(Color::Red));
/// println!("{} Blue background color", Colored::Bg(Color::Blue));
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Colored {
    /// A foreground color.
    Fg(Color),
    /// A background color.
    Bg(Color),
}

impl Display for Colored {
    fn fmt(&self, _f: &mut ::std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let colored_terminal = color();

        match *self {
            Colored::Fg(color) => colored_terminal
                .set_fg(color)
                .map_err(|_| std::fmt::Error)?,
            Colored::Bg(color) => colored_terminal
                .set_bg(color)
                .map_err(|_| std::fmt::Error)?,
        }

        Ok(())
    }
}
