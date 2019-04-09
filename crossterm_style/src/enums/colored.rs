use crate::color::color;
use crate::enums::Color;
use std::fmt::Display;

/// Could be used to color the foreground or background color.
///
/// `Colored::Fg` represents the foreground color.
/// `Color::Bg` represents the background color.
///
/// # Example
///
/// You can use `Colored` in a write statement to apply the attribute to the terminal output.
///
/// ```rust
/// println!("{} Red foreground color", Colored::Fg(Color::Red));
/// println!("{} Blue background color", Colored::Bg(Color::Blue));
/// ```
///
/// You can also call coloring functions on a `&'static str`:
/// ```rust
/// use crossterm(_style)::Colorizer;
/// let styled_text = "Red forground color on blue background.".red().on_blue();
/// println!("{}", styled_text);
/// ```
pub enum Colored {
    Fg(Color),
    Bg(Color),
}

impl Display for Colored {
    fn fmt(&self, _f: &mut ::std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        let colored_terminal = color();

        match *self {
            Colored::Fg(color) => {
                colored_terminal.set_fg(color).unwrap();
            }
            Colored::Bg(color) => {
                colored_terminal.set_bg(color).unwrap();
            }
        }

        Ok(())
    }
}
