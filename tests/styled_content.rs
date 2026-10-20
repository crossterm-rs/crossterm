//! Styled content must not leak the colors it applied to the terminal.
//!
//! `Display` for a `StyledContent` is the formatter behind `println!("{}", ...)`,
//! so it is also what most applications use. Every color it sets has to be reset
//! afterwards, otherwise the next unpainted text keeps the color.

use crossterm::style::{Color, Stylize, style};

#[test]
fn underline_color_is_reset_after_styled_content() {
    let styled = style("x").underline(Color::Red);

    // `58;5;9` sets the underline color, `59` resets it again. Resetting the
    // foreground (`39`) instead leaves the underline color active.
    assert_eq!(format!("{styled}"), "\x1b[58;5;9mx\x1b[59m");
}

#[test]
fn underline_color_and_foreground_are_both_reset() {
    let styled = style("x").underline(Color::Red).with(Color::Blue);

    assert_eq!(
        format!("{styled}"),
        "\x1b[38;5;12m\x1b[58;5;9mx\x1b[39m\x1b[59m"
    );
}
