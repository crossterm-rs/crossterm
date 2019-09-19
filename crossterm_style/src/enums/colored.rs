use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::color::color;
use crate::enums::Color;

/// Could be used to color the foreground or background color.
///
/// `Colored::Fg` represents the foreground color.
/// `Color::Bg` represents the background color.
///
/// # Example
///
/// You can use `Colored` in a write statement to apply the attribute to the terminal output.
///
/// ```ignore
/// println!("{} Red foreground color", Colored::Fg(Color::Red));
/// println!("{} Blue background color", Colored::Bg(Color::Blue));
/// ```
///
/// You can also call coloring functions on a `&'static str`:
/// ```ignore
/// let styled_text = "Red forground color on blue background.".red().on_blue();
/// println!("{}", styled_text);
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Colored {
    Fg(Color),
    Bg(Color),
}

impl Display for Colored {
    fn fmt(&self, _f: &mut ::std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
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
