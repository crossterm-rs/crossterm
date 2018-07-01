//!    
//! Examples of coloring the terminal.
//!

extern crate crossterm;

use self::crossterm::style::{ Color };
use self::crossterm::terminal;
use self::crossterm::Context;

/// print some red font | demonstration.
pub fn paint_foreground()
{
    let context = Context::new();
    let terminal = terminal::terminal(context.clone());

    // Pass an string to the `paint()` method with you want to paint. 
    // This will give you an object back wits can be styled and displayed.
    let mut styledobject = terminal.paint("Red font");
    // Call the method `with()` on the object given by `paint()` and pass in any Color from the Color enum.
    styledobject = styledobject.with(Color::Red);
    // Print the object to the console and see the result.    
    println!("{}", styledobject);

    // Crossterm provides method chaining so that the above points can be inlined.
    println!("{}", terminal.paint("Red font").with(Color::Red));
}

/// print some font on red background | demonstration.
pub fn paint_background()
{
    let context = Context::new();
    let terminal = terminal::terminal(context.clone());

    // Pass an string to the `paint()` method with you want to paint. 
    // This will give you an object back wits can be styled and displayed.
    let mut styledobject = terminal.paint("Red background color");
    // Call the method `on()` on the object given by `paint()` and pass in an Color from the Color enum.
    styledobject = styledobject.on(Color::Red);
    // Print the object to the console and check see the result    
    println!("{}", styledobject);

    // Crossterm provides method chaining so that the above points can be inlined.
    println!("{}", terminal.paint("Red background color").on(Color::Red));
}

/// print font with fore- background color | demonstration.
pub fn paint_foreground_and_background()
{
    let context = Context::new();
    let terminal = terminal::terminal(context.clone());

    // Pass an string to the `paint()` method with you want to paint. 
    // This will give you an object back wits can be styled and displayed.
    let mut styledobject = terminal.paint("Red font on blue background color");
    /* Foreground color: 
            Call the method `with()` on the object given by `paint()`
            Pass in an Color from the Color enum.
    */
    styledobject = styledobject.with(Color::Red);
    /* Background color: 
            Call the method `on()` on the object given by `paint()`
            Pass in an Color from the Color enum.
    */
    styledobject = styledobject.on(Color::Blue);
    // Print the object to the console and see the result.
    println!("{}", styledobject);

    // Crossterm provides method chaining so that the above points can be inlined.
    println!("{}", terminal.paint("Red font on blue background color").with(Color::Red).on(Color::Blue));
}

/// Print all available foreground colors | demonstration.
pub fn print_all_foreground_colors()
{
    let context = Context::new();
    let terminal = terminal::terminal(context.clone());

    println!("Black : \t {}", terminal.paint("■").with(Color::Black));
    println!("Red : \t\t {}", terminal.paint("■").with(Color::Red));
    println!("Dark Red: \t {}", terminal.paint("■").with(Color::DarkRed));
    println!("Green : \t {}", terminal.paint("■").with(Color::Green));
    println!("Dark Green : \t {}", terminal.paint("■").with(Color::DarkGreen));
    println!("Yellow : \t {}", terminal.paint("■").with(Color::Yellow));
    println!("Dark Yellow : \t {}", terminal.paint("■").with(Color::DarkYellow));
    println!("Blue : \t\t {}", terminal.paint("■").with(Color::Blue));
    println!("Dark Blue : \t {}", terminal.paint("■").with(Color::DarkBlue));
    println!("Magenta : \t {}", terminal.paint("■").with(Color::Magenta));
    println!("Dark Magenta : \t {}", terminal.paint("■").with(Color::DarkMagenta));
    println!("Cyan : \t\t {}", terminal.paint("■").with(Color::Cyan));
    println!("Dark Cyan : \t {}", terminal.paint("■").with(Color::DarkCyan));
    println!("Grey : \t\t {}", terminal.paint("■").with(Color::Grey));
    println!("White : \t {}", terminal.paint("■").with(Color::White));
}

/// Print all available foreground colors | demonstration.
pub fn print_all_background_colors()
{
    let context = Context::new();
    let terminal = terminal::terminal(context.clone());

    println!("Black : \t {}", terminal.paint("  ").on(Color::Black));
    println!("Red : \t\t {}", terminal.paint("  ").on(Color::Red));
    println!("Dark Red: \t {}", terminal.paint("  ").on(Color::DarkRed));
    println!("Green : \t {}", terminal.paint("  ").on(Color::Green));
    println!("Dark Green : \t {}", terminal.paint("  ").on(Color::DarkGreen));
    println!("Yellow : \t {}", terminal.paint("  ").on(Color::Yellow));
    println!("Dark Yellow : \t {}", terminal.paint("  ").on(Color::DarkYellow));
    println!("Blue : \t\t {}", terminal.paint("  ").on(Color::Blue));
    println!("Dark Blue : \t {}", terminal.paint("  ").on(Color::DarkBlue));
    println!("Magenta : \t {}", terminal.paint("  ").on(Color::Magenta));
    println!("Dark Magenta : \t {}", terminal.paint("  ").on(Color::DarkMagenta));
    println!("Cyan : \t\t {}", terminal.paint("  ").on(Color::Cyan));
    println!("Dark Cyan : \t {}", terminal.paint("  ").on(Color::DarkCyan));
    println!("Grey : \t\t {}", terminal.paint("  ").on(Color::Grey));
    println!("White : \t {}", terminal.paint("  ").on(Color::White));
    #[cfg(unix)]
    println!("RGB (10,10,10): \t {}", terminal.paint("  ").on(Color::Rgb {r: 10, g: 10, b: 10}));
    #[cfg(unix)]
    println!("RGB (10,10,10): \t {}", terminal.paint("  ").on(Color::AnsiValue(50)));
}

/// Print font with all available attributes. Note that this can only be used at unix systems and that some are not supported widely | demonstration..
#[cfg(unix)]
pub fn print_font_with_attributes()
{
    let context = Context::new();
    let terminal = terminal::terminal(context.clone());

    println!("{}", terminal.paint("Normal text"));
    println!("{}", terminal.paint("Bold text").bold());
    println!("{}", terminal.paint("Italic text").italic());
    println!("{}", terminal.paint("Slow blinking text").slow_blink());
    println!("{}", terminal.paint("Rapid blinking text").rapid_blink());
    println!("{}", terminal.paint("Hidden text").hidden());
    println!("{}", terminal.paint("Underlined text").underlined());
    println!("{}", terminal.paint("Reversed color").reverse());
    println!("{}", terminal.paint("Dim text color").dim());
    println!("{}", terminal.paint("Crossed out font").crossed_out());
}

/// Print all supported rgb colors  | demonstration.
#[cfg(unix)]
pub fn print_supported_colors()
{
    let context = Context::new();
    let terminal = terminal::terminal(context.clone());

    let count = crossterm::style::color(context.clone()).get_available_color_count().unwrap();

    for i in 0..count
    {
        println!("{}", terminal.paint(format!("Color: {}",i)).with(Color::AnsiValue(i as u8)));
    }
}