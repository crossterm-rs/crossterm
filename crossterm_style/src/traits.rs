use std::fmt::Display;

use crate::StyledObject;

/// Provides a set of methods to color any type implementing `Display` with attributes.
///
/// This trait is implemented for `&static str` and `StyledObject` and thus the methods of this trait could be called on them.
///
/// ```ignore
/// use Colorizer;
///
/// let styled_text = "Red forground color on blue background.".red().on_blue();
/// println!("{}", styled_text);
/// ```
pub trait Colorize<D: Display> {
    fn black(self) -> StyledObject<D>;
    fn dark_grey(self) -> StyledObject<D>;
    fn red(self) -> StyledObject<D>;
    fn dark_red(self) -> StyledObject<D>;
    fn green(self) -> StyledObject<D>;
    fn dark_green(self) -> StyledObject<D>;
    fn yellow(self) -> StyledObject<D>;
    fn dark_yellow(self) -> StyledObject<D>;
    fn blue(self) -> StyledObject<D>;
    fn dark_blue(self) -> StyledObject<D>;
    fn magenta(self) -> StyledObject<D>;
    fn dark_magenta(self) -> StyledObject<D>;
    fn cyan(self) -> StyledObject<D>;
    fn dark_cyan(self) -> StyledObject<D>;
    fn white(self) -> StyledObject<D>;
    fn grey(self) -> StyledObject<D>;

    fn on_black(self) -> StyledObject<D>;
    fn on_dark_grey(self) -> StyledObject<D>;
    fn on_red(self) -> StyledObject<D>;
    fn on_dark_red(self) -> StyledObject<D>;
    fn on_green(self) -> StyledObject<D>;
    fn on_dark_green(self) -> StyledObject<D>;
    fn on_yellow(self) -> StyledObject<D>;
    fn on_dark_yellow(self) -> StyledObject<D>;
    fn on_blue(self) -> StyledObject<D>;
    fn on_dark_blue(self) -> StyledObject<D>;
    fn on_magenta(self) -> StyledObject<D>;
    fn on_dark_magenta(self) -> StyledObject<D>;
    fn on_cyan(self) -> StyledObject<D>;
    fn on_dark_cyan(self) -> StyledObject<D>;
    fn on_white(self) -> StyledObject<D>;
    fn on_grey(self) -> StyledObject<D>;
}

/// Provides a set of methods to style any type implementing `Display` with attributes.
///
/// This trait is implemented for `&static str` and `StyledObject` and thus the methods of this trait could be called on them.
///
/// # Example
/// ```ignore
/// use Colorizer;
///
/// println!("{}", "Bold text".bold();
/// println!("{}", "Underlined text".underlined();
/// println!("{}", "Negative text".negative();
/// ```
pub trait Styler<D: Display> {
    fn reset(self) -> StyledObject<D>;
    fn bold(self) -> StyledObject<D>;
    fn underlined(self) -> StyledObject<D>;
    fn reverse(self) -> StyledObject<D>;
    fn dim(self) -> StyledObject<D>;
    fn italic(self) -> StyledObject<D>;
    fn negative(self) -> StyledObject<D>;
    fn slow_blink(self) -> StyledObject<D>;
    fn rapid_blink(self) -> StyledObject<D>;
    fn hidden(self) -> StyledObject<D>;
    fn crossed_out(self) -> StyledObject<D>;
}
