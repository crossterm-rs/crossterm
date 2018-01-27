//!    
//! Color Examples
//!

extern crate crossterm;

use self::crossterm::crossterm_style::{paint, Color};

/// print some red font | demonstration.
pub fn paint_foreground()
{    
    // Pass an string to the `paint()` method with you want to paint. 
    // This will give you an object back wits can be styled and displayed.
    let mut styledobject = paint("Red font");
    // Call the method `with()` on the object given by `paint()` and pass in any Color from the Color enum.
    styledobject = styledobject.with(Color::Red);
    // Print the object to the console and see the result.    
    println!("{}", styledobject);

    // Crossterm provides method chaining so that the above points can be inlined.
    println!("{}", paint("Red font").with(Color::Red));
}

/// print some font on red background | demonstration.
pub fn paint_background()
{   
    // Pass an string to the `paint()` method with you want to paint. 
    // This will give you an object back wits can be styled and displayed.
    let mut styledobject = paint("Red background color");
    // Call the method `on()` on the object given by `paint()` and pass in an Color from the Color enum.
    styledobject = styledobject.on(Color::Red);
    // Print the object to the console and check see the result    
    println!("{}", styledobject);

    // Crossterm provides method chaining so that the above points can be inlined.
    println!("{}", paint("Red background color").on(Color::Red));
}

/// print font with fore- background color | demonstration.
pub fn paint_foreground_and_background()
{    
    // Pass an string to the `paint()` method with you want to paint. 
    // This will give you an object back wits can be styled and displayed.
    let mut styledobject = paint("Red font on blue background color");
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
    println!("{}", paint("Red font on blue background color").with(Color::Red).on(Color::Blue));
}

/// Print all available foreground colors | demonstration.
pub fn print_all_foreground_colors()
{
    println!("Black : \t {}", paint("■").with(Color::Black));
    println!("Red : \t\t {}", paint("■").with(Color::Red));
    println!("Dark Red: \t {}", paint("■").with(Color::DarkRed));
    println!("Green : \t {}", paint("■").with(Color::Green));
    println!("Dark Green : \t {}", paint("■").with(Color::DarkGreen));
    println!("Yellow : \t {}", paint("■").with(Color::Yellow));
    println!("Dark Yellow : \t {}", paint("■").with(Color::DarkYellow));
    println!("Blue : \t\t {}", paint("■").with(Color::Blue));
    println!("Dark Blue : \t {}", paint("■").with(Color::DarkBlue));
    println!("Magenta : \t {}", paint("■").with(Color::Magenta));
    println!("Dark Magenta : \t {}", paint("■").with(Color::DarkMagenta));
    println!("Cyan : \t\t {}", paint("■").with(Color::Cyan));
    println!("Dark Cyan : \t {}", paint("■").with(Color::DarkCyan));
    println!("Grey : \t\t {}", paint("■").with(Color::Grey));
    println!("White : \t {}", paint("■").with(Color::White));
}

/// Print all available foreground colors | demonstration.
pub fn print_all_background_colors()
{
    println!("Black : \t {}", paint("  ").on(Color::Black));
    println!("Red : \t\t {}", paint("  ").on(Color::Red));
    println!("Dark Red: \t {}", paint("  ").on(Color::DarkRed));
    println!("Green : \t {}", paint("  ").on(Color::Green));
    println!("Dark Green : \t {}", paint("  ").on(Color::DarkGreen));
    println!("Yellow : \t {}", paint("  ").on(Color::Yellow));
    println!("Dark Yellow : \t {}", paint("  ").on(Color::DarkYellow));
    println!("Blue : \t\t {}", paint("  ").on(Color::Blue));
    println!("Dark Blue : \t {}", paint("  ").on(Color::DarkBlue));
    println!("Magenta : \t {}", paint("  ").on(Color::Magenta));
    println!("Dark Magenta : \t {}", paint("  ").on(Color::DarkMagenta));
    println!("Cyan : \t\t {}", paint("  ").on(Color::Cyan));
    println!("Dark Cyan : \t {}", paint("  ").on(Color::DarkCyan));
    println!("Grey : \t\t {}", paint("  ").on(Color::Grey));
    println!("White : \t {}", paint("  ").on(Color::White));
    #[cfg(unix)]
    println!("RGB (10,10,10): \t {}", paint("  ").on(Color::Rgb {r: 10, g: 10, b: 10}));
    #[cfg(unix)]
    println!("RGB (10,10,10): \t {}", paint("  ").on(Color::AnsiValue(50)));
}

/// Print font with all available attributes. Note that this can only be used at unix systems and that some are not supported widely.
#[cfg(unix)]
pub fn print_font_with_attributes()
{
    println!("{}", paint("Normal text"));
    println!("{}", paint("Bold text").bold());
    println!("{}", paint("Italic text").italic());
    println!("{}", paint("Slow blinking text").slow_blink());
    println!("{}", paint("Rapid blinking text").rapid_blink());
    println!("{}", paint("Hidden text").hidden());
    println!("{}", paint("Underlined text").underlined());
    println!("{}", paint("Reversed color").reverse());
    println!("{}", paint("Dim text color").dim());
    println!("{}", paint("Crossed out font").crossed_out());
}

/// Print all supported rgb colors 
#[cfg(unix)]#[cfg(unix)]
pub fn print_supported_colors()
{   
    let count = crossterm::crossterm_style::get().get_available_color_count().unwrap();

    for i in 0..count
    {
        println!("{}", paint(format!("Color: {}",i)).with(Color::AnsiValue(i as u8)));

    }
}