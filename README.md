# Crossterm | crossplatform terminal library written in rust.

Ever got disappointed when a terminal library for rust was only written for unix systems? Crossterm provides the same core functionalities for both windows and unix systems. 

Crossterm aims to be simple and easy to call in code. True the simplicity of crossterm you do not have to worry about the platform your working with. You can just call the action you want to preform and unther water it will check what to do based on the current platform.

## Getting Started

Add the crossterm package to your `Cargo.toml` file.

```
[dependencies]
crossterm = "*"

```

Add the crate to your solution.
And use the crossterm modules withs you want to use.

```rust  
extern crate crossterm;

// this module is used for styling the terminal
use self::crossterm::crossterm_style::*;
// this module is used for cursor related actions
use self::crossterm::crossterm_cursor::*;
// this mudule is used for terminal related actions
use self::crossterm::crossterm_terminal::*;

```
## Links

Documentation for the code can be found [here](https://atcentra.com/crossterm/index.html)

Documentation for the code can be found [here](https://docs.rs/crossterm/0.1.0/crossterm/)

The Cargo Page can be found [here](https://crates.io/search?q=crossterm)

## Examples

For detailed examples of all crossterm functionalities check the [examples](https://github.com/TimonPost/crossterm/tree/master/examples) direcory.

### Styled font
```rust    
    use crossterm::crossterm_style::{paint, Color};
    
    // Crossterm provides method chaining so that you can style the font nicely.
    // the `with()` methods sets the foreground color and the `on()` methods sets the background color
    // You can either store the styled font.
    let mut styledobject = paint("Stored styled font").with(Color::Red).on(Color::Blue);
    println!("{}",styledobject);
    
    // Or you can print it directly.
    println!("{}", paint("Red font on blue background color").with(Color::Red).on(Color::Blue));     
    println!("{}", paint("Red font on default background color").with(Color::Red));
    println!("{}", paint("Default font color on Blue background color").on(Color::Blue));
    
    /// The following code can only be used for unix systems:
    // Set background Color from RGB
    println!("RGB (10,10,10): \t {}", paint("  ").on(Color::Rgb {r: 10, g: 10, b: 10}));
     // Set background Color from RGB
    println!("RGB (10,10,10): \t {}", paint("  ").on(Color::AnsiValue(50)));
    
    // Use attributes to syle the font.
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
    
```
### Cursor
```rust 

     use crossterm::crossterm_cursor::get;
    
     let mut cursor = get();
    
     /// Moving the cursor
     // Set the cursor to position X: 10, Y: 5 in the terminal
     cursor.goto(10,5);
    
     // Move the cursor to position 3 times to the up in the terminal
     cursor.move_up(3);
    
     // Move the cursor to position 3 times to the right in the terminal
     cursor.move_right(3);
    
     // Move the cursor to position 3 times to the down in the terminal
     cursor.move_down(3);
    
     // Move the cursor to position 3 times to the left in the terminal
        cursor.move_left(3);
    
     // Print an character at X: 10, Y: 5 (see examples for more explanation why to use this method).
     // cursor.goto(10,5).print("@");   
     
     /// Safe the current cursor position to recall later     
     // Goto X: 5 Y: 5
     cursor.goto(5,5);
     // Safe cursor position: X: 5 Y: 5
     cursor.safe_position();
     // Goto X: 5 Y: 20
     cursor.goto(5,20);
     // Print at X: 5 Y: 20.
     print!("Yea!");
     // Reset back to X: 5 Y: 5.
     cursor.reset_position();
     // Print 'Back' at X: 5 Y: 5.
     print!("Back");    
```

### Terminal
```rust 
   use crossterm::crossterm_terminal::{get,ClearType};
   
   let mut terminal = get();

   // Clear all lines in terminal;
   terminal.clear(ClearType::All);
   // Clear all cells from current cursor position down.
   terminal.clear(ClearType::FromCursorDown);
   // Clear all cells from current cursor position down.
   terminal.clear(ClearType::FromCursorUp);
   // Clear current line cells.
   terminal.clear(ClearType::CurrentLine);
   // Clear all the cells until next line.
   terminal.clear(ClearType::UntilNewLine);

   // Get terminal size
   let terminal_size = terminal.terminal_size().unwrap();
   // Print results
   print!("X: {}, y: {}", terminal_size.0, terminal_size.1);

   // Scroll down 10 lines.
   terminal.scroll_down(10);

   // Scroll up 10 lines.
   terminal.scroll_up(10);
   
   // Set terminal size
    terminal.set_size(10,10);
```

## Features crossterm 0.1

- Cursor movement.
    - Up, Down, Left, Right.
    - Goto an certain position.
- Styled output
    - Foreground color (16 base colors)
    - Background color (16 base colors)
- Terminal
    - Clearing
    - Scrolling
    - Size
- Detailed documentation on every item.
- Examples for every client callable code.

## Features crossterm 0.2

- 256 color support. 
- Text Attributes like: bold, italic, underscore and crossed word ect. 
- Custom ANSI color code input to set fore- and background color for unix.
- Storing the current cursor position and resetting to that stored cursor position later. 
- Resizing the terminal.

## Tested terminals

- Windows Powershell
    - Windows 10 (pro)
- Windows CMD
    - Windows 10 (pro)
- Ubuntu Desktop Terminal
    - Ubuntu 17.10


The above terminals have been tested. Crossterm should works also for windows 7, 8 consoles and all ansi supportable consoles.
But these are yet to be tested. 
If you have used this library for an terminal other than the above list without issues feel free to add it to the above list.
    
## How it works

Crossterm is using `WINAPI` for windows systems and `ANSI escape codes` for unix systems. Crossterm provides one base trait with can be implemented for a platform specific instance. For example, there is an implementation for windows (`WINAPI`) and unix(`ANSI`) for the `cursor module`. To call the platform specific implementation there is one module that rules them all. Thrue this module the client calls some action and the module will deside what to do based on the current platform. And it will execute that action.

## Notice 
This library is library is stable. There will not be changed mutch in the code design so do not worry to mutch. If there are any changes that affect previous versions I will describe what to change when upgrading crossterm to an newer version.

## Todo

- Handling mouse events 
- Handling key events
- Tests

## Contributing

If you would like to contribute to crossterm, than please design the code as it is now. Each module contains the same structures so we can easely extend to multible platforms. As you study the code you will quiqly see what the architecture is. Maybe later there will be an documentation for how crossterm is design.

## Versioning

The current version is crossterm 0.1, every commit I merge the version go's up like 0.1.0 -> 0.1.1 -> 0.1.2.

When new features arrives the packages will go up like 0.1 -> 0.2 -> 0.3

## Authors

* **Timon Post** - *Project Owner & creator*

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details



