use std::fmt::Display;

use super::StyledContent;

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
/// use crossterm::style::Colorize;
///
/// let styled_text = "Red foreground color on blue background.".red().on_blue();
/// println!("{}", styled_text);
/// ```
pub trait Colorize<D: Display + Clone> {
    fn black(self) -> StyledContent<D>;
    fn dark_grey(self) -> StyledContent<D>;
    fn red(self) -> StyledContent<D>;
    fn dark_red(self) -> StyledContent<D>;
    fn green(self) -> StyledContent<D>;
    fn dark_green(self) -> StyledContent<D>;
    fn yellow(self) -> StyledContent<D>;
    fn dark_yellow(self) -> StyledContent<D>;
    fn blue(self) -> StyledContent<D>;
    fn dark_blue(self) -> StyledContent<D>;
    fn magenta(self) -> StyledContent<D>;
    fn dark_magenta(self) -> StyledContent<D>;
    fn cyan(self) -> StyledContent<D>;
    fn dark_cyan(self) -> StyledContent<D>;
    fn white(self) -> StyledContent<D>;
    fn grey(self) -> StyledContent<D>;

    fn on_black(self) -> StyledContent<D>;
    fn on_dark_grey(self) -> StyledContent<D>;
    fn on_red(self) -> StyledContent<D>;
    fn on_dark_red(self) -> StyledContent<D>;
    fn on_green(self) -> StyledContent<D>;
    fn on_dark_green(self) -> StyledContent<D>;
    fn on_yellow(self) -> StyledContent<D>;
    fn on_dark_yellow(self) -> StyledContent<D>;
    fn on_blue(self) -> StyledContent<D>;
    fn on_dark_blue(self) -> StyledContent<D>;
    fn on_magenta(self) -> StyledContent<D>;
    fn on_dark_magenta(self) -> StyledContent<D>;
    fn on_cyan(self) -> StyledContent<D>;
    fn on_dark_cyan(self) -> StyledContent<D>;
    fn on_white(self) -> StyledContent<D>;
    fn on_grey(self) -> StyledContent<D>;
}

/// Provides a set of methods to set the text attributes.
///
/// Method names correspond to the [`Attribute`](enum.Attribute.html) enum variants.
///
/// # Examples
///
/// ```no_run
/// use crossterm::style::Styler;
///
/// println!("{}", "Bold text".bold());
/// println!("{}", "Underlined text".underlined());
/// println!("{}", "Negative text".negative());
/// ```
pub trait Styler<D: Display + Clone> {
    fn reset(self) -> StyledContent<D>;
    fn bold(self) -> StyledContent<D>;
    fn underlined(self) -> StyledContent<D>;
    fn reverse(self) -> StyledContent<D>;
    fn dim(self) -> StyledContent<D>;
    fn italic(self) -> StyledContent<D>;
    fn negative(self) -> StyledContent<D>;
    fn slow_blink(self) -> StyledContent<D>;
    fn rapid_blink(self) -> StyledContent<D>;
    fn hidden(self) -> StyledContent<D>;
    fn crossed_out(self) -> StyledContent<D>;
}
