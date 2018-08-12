//!
//! Examples of coloring the terminal.
//!
extern crate crossterm;

use self::crossterm::style::{Color, style, color};
use self::crossterm::{terminal, Screen};

/// print some red font | demonstration.
pub fn paint_foreground() {
    let screen = Screen::default();
    // Pass an string to the `paint()` method with you want to paint.
    // This will give you an object back wits can be styled and displayed.
    // Call the method `with()` on the object given by `style()` and pass in any Color from the Color enum.
    let mut styledobject = style("Red foreground").with(Color::Red);

    // Print the object to the given screen and.
    styledobject.paint(&screen);

    style("Some colored text").with(Color::Blue).on(Color::Black).paint(&screen);

    // Crossterm provides method chaining for coloring so that the above points can be inlined.
    style(format!("Red foreground color : \t {}", "■")).with(Color::Red).paint(&screen);
}

/// print some font on red background | demonstration.
pub fn paint_background() {
    let screen = Screen::default();
    // Pass an string to the `paint()` method with you want to paint.
    // This will give you an object back wits can be styled and displayed.
    // Call the method `on()` on the object given by `style()` and pass in any Color from the Color enum.
    let mut styledobject = style("Red background color").on(Color::Red);

    // Print the object to the given screen and.
    styledobject.paint(&screen);

    // Crossterm provides method chaining for coloring so that the above points can be inlined.
    style(format!("Red background color : \t {}", "■")).with(Color::Red).paint(&screen);
}

/// Print all available foreground colors | demonstration.
pub fn print_all_foreground_colors() {
    let screen = Screen::default();

    style(format!("Black : \t\t {} \n", "■")).with(Color::Black).paint(&screen);
    style(format!("Red : \t\t {} \n", "■")).with(Color::Red).paint(&screen);
    style(format!("Cyan : \t\t {} \n", "■")).with(Color::Cyan).paint(&screen);
    style(format!("DarkCyan : \t {} \n", "■")).with(Color::DarkCyan).paint(&screen);
    style(format!("DarkRed : \t {} \n", "■")).with(Color::DarkRed).paint(&screen);
    style(format!("Green : \t {} \n", "■")).with(Color::Green).paint(&screen);
    style(format!("DarkGreen : \t {} \n", "■")).with(Color::DarkGreen).paint(&screen);
    style(format!("Blue : \t\t {} \n", "■")).with(Color::Blue).paint(&screen);
    style(format!("DarkBlue : \t {} \n", "■")).with(Color::DarkBlue).paint(&screen);
    style(format!("Magenta : \t {} \n", "■")).with(Color::Magenta).paint(&screen);
    style(format!("DarkMagenta : \t {} \n", "■")).with(Color::DarkMagenta).paint(&screen);
    style(format!("Yellow : \t {} \n", "■")).with(Color::Yellow).paint(&screen);
    style(format!("DarkYellow : \t {} \n", "■")).with(Color::DarkYellow).paint(&screen);
    style(format!("Grey : \t\t {} \n", "■")).with(Color::Grey).paint(&screen);
    style(format!("White : \t {} \n", "■")).with(Color::White).paint(&screen);

    #[cfg(unix)]
        style("RGB color (10,10,10) ").with(Color::Rgb {
        r: 10,
        g: 10,
        b: 10
    }).paint(&screen);

    #[cfg(unix)]
        style("RGB color (10,10,10) ").with(Color::AnsiValue(50)).paint(&screen);
}

/// Print all available foreground colors | demonstration.
pub fn print_all_background_colors() {
    let screen = Screen::default();

    style(format!("Black : \t {} \n", "■")).on(Color::Black).paint(&screen);
    style(format!("Red : \t\t {} \n", "■")).on(Color::Red).paint(&screen);
    style(format!("Cyan : \t\t {} \n", "■")).on(Color::Cyan).paint(&screen);
    style(format!("DarkCyan : \t {} \n", "■")).on(Color::DarkCyan).paint(&screen);
    style(format!("DarkRed : \t {} \n", "■")).on(Color::DarkRed).paint(&screen);
    style(format!("Green : \t {} \n", "■")).on(Color::Green).paint(&screen);
    style(format!("DarkGreen : \t {} \n", "■")).on(Color::DarkGreen).paint(&screen);
    style(format!("Blue : \t\t {} \n", "■")).on(Color::Blue).paint(&screen);
    style(format!("DarkBlue : \t {} \n", "■")).on(Color::DarkBlue).paint(&screen);
    style(format!("Magenta : \t {} \n", "■")).on(Color::Magenta).paint(&screen);
    style(format!("DarkMagenta : \t {} \n", "■")).on(Color::DarkMagenta).paint(&screen);
    style(format!("Yellow : \t {} \n", "■")).on(Color::Yellow).paint(&screen);
    style(format!("DarkYellow : \t {} \n", "■")).on(Color::DarkYellow).paint(&screen);
    style(format!("Grey : \t\t {} \n", "■")).on(Color::Grey).paint(&screen);
    style(format!("White : \t {} \n", "■")).on(Color::White).paint(&screen);
    
    #[cfg(unix)]
    style("RGB color (10,10,10) ").on(Color::Rgb {
        r: 10,
        g: 10,
        b: 10
    }).paint(&screen);

    #[cfg(unix)]
    style("RGB color (10,10,10) ").on(Color::AnsiValue(50)).paint(&screen);
}

/// Print font with all available attributes. Note that this can only be used at unix systems and that some are not supported widely | demonstration..
#[cfg(unix)]
pub fn print_font_with_attributes() {
    let screen = Screen::default();
    style("Normal text").paint(&screen);
    style("Bold text").bold().paint(&screen);
    style("Italic text").italic().paint(&screen);
    style("Slow blinking text").slow_blink().paint(&screen);
    style("Rapid blinking text").rapid_blink().paint(&screen);
    style("Hidden text").hidden().paint(&screen);
    style("Underlined text").underlined().paint(&screen);
    style("Reversed text").reverse().paint(&screen);
    style("Dim text").dim().paint(&screen);
    style("Crossed out font").crossed_out().paint(&screen);
}

/// Print all supported RGB colors  | demonstration.
#[cfg(unix)]
pub fn print_supported_colors() {
    let screen = Screen::default();
    let count = color(&screen)
        .get_available_color_count()
        .unwrap();

    for i in 0..count {
        style(format!("White : \t {}", i)).on(Color::AnsiValue(i as u8)).paint(&screen);
    }
}
