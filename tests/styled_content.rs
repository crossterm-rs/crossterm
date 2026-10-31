//! Styled content must not leak the colors it applied to the terminal.
//!
//! `Display` for a `StyledContent` is the formatter behind `println!("{}", ...)`,
//! so it is also what most applications use. Every color it sets has to be reset
//! afterwards, otherwise the next unpainted text keeps the color.

use crossterm::style::{Color, Stylize, force_color_output, style};

/// These tests assert the exact ANSI bytes `Display` writes, so two environment
/// effects have to be neutralised first: `NO_COLOR` (force colours back on) and
/// Windows without ANSI support, where `Display` takes the WinAPI path and does
/// not write a byte string at all.
fn ansi_display_supported() -> bool {
    force_color_output(true);
    windows_ansi_supported()
}

#[cfg(windows)]
fn windows_ansi_supported() -> bool {
    crossterm::ansi_support::supports_ansi()
}

#[cfg(not(windows))]
fn windows_ansi_supported() -> bool {
    true
}

#[test]
fn underline_color_is_reset_after_styled_content() {
    if !ansi_display_supported() {
        return;
    }
    let styled = style("x").underline(Color::Red);

    // `58;5;9` sets the underline color, `59` resets it again. Resetting the
    // foreground (`39`) instead leaves the underline color active.
    assert_eq!(format!("{styled}"), "\x1b[58;5;9mx\x1b[59m");
}

#[test]
fn underline_color_and_foreground_are_both_reset() {
    if !ansi_display_supported() {
        return;
    }
    let styled = style("x").underline(Color::Red).with(Color::Blue);

    assert_eq!(
        format!("{styled}"),
        "\x1b[38;5;12m\x1b[58;5;9mx\x1b[39m\x1b[59m"
    );
}
