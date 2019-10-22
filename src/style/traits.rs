use std::fmt::Display;

use crate::StyledObject;

/// Provides a set of methods to set the colors.
///
/// Every method with the `on_` prefix sets the background color. All other methods set
/// the foreground color.
///
/// Method names correspond to the [`Color`](enum.Color.html) enum variants.
///
/// # Examples
///
/// ```no_run
/// use crossterm::Colorize;
///
/// let styled_text = "Red foreground color on blue background.".red().on_blue();
/// println!("{}", styled_text);
/// ```
pub trait Colorize<D: Display + Clone> {
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

/// Provides a set of methods to set the text attributes.
///
/// Method names correspond to the [`Attribute`](enum.Attribute.html) enum variants.
///
/// # Examples
///
/// ```no_run
/// use crossterm::Styler;
///
/// println!("{}", "Bold text".bold());
/// println!("{}", "Underlined text".underlined());
/// println!("{}", "Negative text".negative());
/// ```
pub trait Styler<D: Display + Clone> {
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
