//!
//! Examples of coloring the terminal.
//!
extern crate crossterm;

use self::crossterm::{style, Color};

/// print some red font | demonstration.
pub fn paint_foreground() {
    // Create a styled object.
    // Call the method `with()` on the object given by `style()` and pass in any Color from the Color enum.
    let styledobject = style("Red foreground").with(Color::Red);

    // Print the object to the given screen and.
    println!("Colored text: {}", styledobject);

    // Or print inline
    println!(
        "Colored text: {}",
        style("Blue foreground").with(Color::Blue)
    );
}

/// print some font on red background | demonstration.
pub fn paint_background() {
    // Create a styled object.
    // Call the method `with()` on the object given by `style()` and pass in any Color from the Color enum.
    let styledobject = style("Red foreground").on(Color::Red);

    // Print the object to the given screen and.
    println!("Colored text: {}", styledobject);

    // Or print inline
    println!("Colored text: {}", style("Red foreground").on(Color::Blue));
}

/// Print all available foreground colors | demonstration.
pub fn print_all_foreground_colors() {
    println!(
        "{}",
        style(format!("Black : \t\t {} \n", "■")).with(Color::Black)
    );
    println!(
        "{}",
        style(format!("Red : \t\t {} \n", "■")).with(Color::Red)
    );
    println!(
        "{}",
        style(format!("Cyan : \t\t {} \n", "■")).with(Color::Cyan)
    );
    println!(
        "{}",
        style(format!("DarkCyan : \t {} \n", "■")).with(Color::DarkCyan)
    );
    println!(
        "{}",
        style(format!("DarkRed : \t {} \n", "■")).with(Color::DarkRed)
    );
    println!(
        "{}",
        style(format!("Green : \t {} \n", "■")).with(Color::Green)
    );
    println!(
        "{}",
        style(format!("DarkGreen : \t {} \n", "■")).with(Color::DarkGreen)
    );
    println!(
        "{}",
        style(format!("Blue : \t\t {} \n", "■")).with(Color::Blue)
    );
    println!(
        "{}",
        style(format!("DarkBlue : \t {} \n", "■")).with(Color::DarkBlue)
    );
    println!(
        "{}",
        style(format!("Magenta : \t {} \n", "■")).with(Color::Magenta)
    );
    println!(
        "{}",
        style(format!("DarkMagenta : \t {} \n", "■")).with(Color::DarkMagenta)
    );
    println!(
        "{}",
        style(format!("Yellow : \t {} \n", "■")).with(Color::Yellow)
    );
    println!(
        "{}",
        style(format!("DarkYellow : \t {} \n", "■")).with(Color::DarkYellow)
    );
    println!(
        "{}",
        style(format!("Grey : \t\t {} \n", "■")).with(Color::Grey)
    );
    println!(
        "{}",
        style(format!("White : \t {} \n", "■")).with(Color::White)
    );

    #[cfg(unix)]
    println!(
        "{}",
        style("RGB color (10,10,10) ").with(Color::Rgb {
            r: 10,
            g: 10,
            b: 10
        })
    );

    #[cfg(unix)]
    println!(
        "{}",
        style("RGB color (10,10,10) ").with(Color::AnsiValue(50))
    );
}

/// Print all available foreground colors | demonstration.
pub fn print_all_background_colors() {
    println!(
        "{}",
        style(format!("Black : \t {} \n", "■")).on(Color::Black)
    );
    println!(
        "{}",
        style(format!("Red : \t\t {} \n", "■")).on(Color::Red)
    );
    println!(
        "{}",
        style(format!("Cyan : \t\t {} \n", "■")).on(Color::Cyan)
    );
    println!(
        "{}",
        style(format!("DarkCyan : \t {} \n", "■")).on(Color::DarkCyan)
    );
    println!(
        "{}",
        style(format!("DarkRed : \t {} \n", "■")).on(Color::DarkRed)
    );
    println!(
        "{}",
        style(format!("Green : \t {} \n", "■")).on(Color::Green)
    );
    println!(
        "{}",
        style(format!("DarkGreen : \t {} \n", "■")).on(Color::DarkGreen)
    );
    println!(
        "{}",
        style(format!("Blue : \t\t {} \n", "■")).on(Color::Blue)
    );
    println!(
        "{}",
        style(format!("DarkBlue : \t {} \n", "■")).on(Color::DarkBlue)
    );
    println!(
        "{}",
        style(format!("Magenta : \t {} \n", "■")).on(Color::Magenta)
    );
    println!(
        "{}",
        style(format!("DarkMagenta : \t {} \n", "■")).on(Color::DarkMagenta)
    );
    println!(
        "{}",
        style(format!("Yellow : \t {} \n", "■")).on(Color::Yellow)
    );
    println!(
        "{}",
        style(format!("DarkYellow : \t {} \n", "■")).on(Color::DarkYellow)
    );
    println!(
        "{}",
        style(format!("Grey : \t\t {} \n", "■")).on(Color::Grey)
    );
    println!(
        "{}",
        style(format!("White : \t {} \n", "■")).on(Color::White)
    );

    #[cfg(unix)]
    println!(
        "{}",
        style("RGB color (10,10,10) ").on(Color::Rgb {
            r: 10,
            g: 10,
            b: 10
        })
    );

    #[cfg(unix)]
    println!(
        "{}",
        style("RGB color (10,10,10) ").on(Color::AnsiValue(50))
    );
}

/// Print font with all available attributes. Note that this can only be used at unix systems and that some are not supported widely | demonstration..
#[cfg(unix)]
pub fn print_font_with_attributes() {
    println!("{}", style("Normal text"));
    println!("{}", style("Bold text").bold());
    println!("{}", style("Italic text").italic());
    println!("{}", style("Slow blinking text").slow_blink());
    println!("{}", style("Rapid blinking text").rapid_blink());
    println!("{}", style("Hidden text").hidden());
    println!("{}", style("Underlined text").underlined());
    println!("{}", style("Reversed text").reverse());
    println!("{}", style("Dim text").dim());
    println!("{}", style("Crossed out font").crossed_out());
}

/// Print font with all available attributes. Note that this can only be used at unix systems and that some are not supported widely | demonstration..
#[cfg(windows)]
pub fn print_font_with_attributes() {
    println!("{}", style("Normal text"));
    println!("{}", style("Bold text").bold());
    println!("{}", style("Underlined text").underlined());
    println!("{}", style("Negative text").negative());
}

/// Print all supported RGB colors  | demonstration.
#[cfg(unix)]
pub fn print_supported_colors() {
    let count = color().get_available_color_count().unwrap();

    for i in 0..count {
        println!(
            "{}",
            style(format!("White : \t {}", i)).on(Color::AnsiValue(i as u8))
        );
    }
}

fn main() {
    print_all_background_colors();
    print_all_foreground_colors();
    print_font_with_attributes();
}
