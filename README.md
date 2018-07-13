[![Latest Version](https://img.shields.io/crates/v/crossterm.svg)](https://crates.io/crates/crossterm) | [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE) | [![docs.rs](https://docs.rs/crossterm/badge.svg)](https://docs.rs/crossterm/) | [Examples](link_to_examples) | [Changelog](link_to_change_log) | [Release Nodes](link_to_release_nodes)
|----|----|----|----|----|----

Ever got disappointed when a terminal library for rust was only written for unix systems? 
Crossterm provides the same core functionalities for both windows and unix systems.

Crossterm aims to be simple and easy to call in code. 
True the simplicity of Crossterm you do not have to worry about the platform your working with. 
You can just call whatever action you want and underwater it will check what to do based on the current platform.

This crate supports all unix terminals and windows terminals down to windows XP (not not all terminals are tested see 'tested terminals' for more info)

## Getting Started

This documentation is only for Crossterm version `0.2.3` check the [Upgrade Manual](link) for more info. 
Also the [examples](link) directory contains examples for each version of Crossterm. 

Add the Crossterm package to your `Cargo.toml` file.

```
[dependencies]
crossterm = "*"

```
And import the Crossterm modules you want to use.

```rust  
extern crate crossterm;

// this module is used for styling the terminal
use self::crossterm::style::*;
// this module is used for cursor related actions
use self::crossterm::cursor::*;
// this mudule is used for terminal related actions
use self::crossterm::terminal::*;

```

## Useful Links

- Code documentation: 
version [0.1.0](https://docs.rs/crossterm/0.1.0/crossterm/), 
version [0.2.0](https://docs.rs/crossterm/0.2.0/crossterm/), 
version [0.2.1](https://docs.rs/crossterm/0.2.1/crossterm/) 
and [0.2.3](link)

- Code functionalities Examples: 
version [0.1.0](link_examples_01), 
version [0.2.0](link_examples_02), 
version [0.2.1](link_examples_03) 
and version [0.2.3](link_examples_04)

- [Cargo Page](https://crates.io/crates/crossterm)
- [Examples for specific versions](link_to_specific_version)
- [Real life examples](example_link)

# Features
These are the futures that this crate supports:

- Cursor.
    - Moving _n_ times Up, Down, Left, Right.
    - Goto an certain position.
    - Get cursor position
    - Storing the current cursor position and resetting to that stored cursor position later.
    - Hiding an showing the cursor. 
    - Control over blinking of the terminal cursor (only some terminals are supporting this).
- Styled output
    - Foreground color (16 base colors)
    - Background color (16 base colors)
    - 256 color support (unix only). 
    - Text Attributes like: bold, italic, underscore and crossed word ect (unix only). 
    - Custom ANSI color code input to set fore- and background color (unix only).
- Terminal
    - Clearing (all lines, current line, from cursor down and up, until new line)
    - Scrolling (Up, down)
    - Get size of terminal
    - Set size of the terminal.
    - Alternate screen
    - Raw screen    
- Exit the current process.
- Detailed documentation on every item.
- Examples for every client callable code.
- Real life examples.

## Examples

For detailed examples of all Crossterm functionalities check the [examples](https://github.com/TimonPost/crossterm/tree/master/examples) directory.

### Crossterm wrapper | [see more](example_link)
This is a wrapper for the modules crossterm provides. This is introduced to mange the `Context` for the user.
```
let crossterm = Crossterm::new();

// get instance of the modules, whereafter you can use the methods the particulary module provides. 
let color = crossterm.color();
let cursor = crossterm.cursor();
let terminal = crossterm.terminal();

// write text to console wheter it be the main screen or the alternate screen.
crossterm.write("some text");
// print some styled font.
println!("{}", crossterm.paint("Red font on blue background").with(Color::Red).on(Color::Blue));
```
### Styled font | [see more](example_link)
```rust    
use crossterm::style::{Color};
use crossterm::Crossterm; 
    
// Crossterm provides method chaining so that you can style the font nicely.
// the `with()` methods sets the foreground color and the `on()` methods sets the background color
// You can either store the styled font.
   
// create instance of `Crossterm`
let crossterm = Crossterm::new();

// store style in styled object and print it
let mut styledobject = crossterm.paint("stored styled font in variable").with(Color::Green).on(Color::Yellow);
println!("{}",styledobject);

// Or you can print it directly.
println!("{}", crossterm.paint("Red font on blue background color").with(Color::Red).on(Color::Blue));
println!("{}", crossterm.paint("Red font on default background color").with(Color::Red));
println!("{}", crossterm.paint("Default font color on Blue background color").on(Color::Blue));

/// The following code can only be used for unix systems:

// Set background Color from RGB
println!("RGB (10,10,10): \t {}", crossterm.paint("  ").on(Color::Rgb {r: 10, g: 10, b: 10}));
// Set background Color from RGB
println!("ANSI value (50): \t {}", crossterm.paint("  ").on(Color::AnsiValue(50)));

// Use attributes to syle the font.
println!("{}", crossterm.paint("Normal text"));
println!("{}", crossterm.paint("Bold text").bold());
println!("{}", crossterm.paint("Italic text").italic());
println!("{}", crossterm.paint("Slow blinking text").slow_blink());
println!("{}", crossterm.paint("Rapid blinking text").rapid_blink());
println!("{}", crossterm.paint("Hidden text").hidden());
println!("{}", crossterm.paint("Underlined text").underlined());
println!("{}", crossterm.paint("Reversed color").reverse());
println!("{}", crossterm.paint("Dim text color").dim());
println!("{}", crossterm.paint("Crossed out font").crossed_out());
```
### Cursor | [see more](example_link)
```rust 

use crossterm::Context;
use crossterm::cursor::cursor;

// create context to pass to the `cursor()` function.
let context = Context::new();
let mut cursor = cursor(&context);

/// Moving the cursor | demo
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

/// Safe the current cursor position to recall later | demo
// Goto X: 5 Y: 5
cursor.goto(5,5);
// Safe cursor position: X: 5 Y: 5
cursor.save_position();
// Goto X: 5 Y: 20
cursor.goto(5,20);
// Print at X: 5 Y: 20.
print!("Yea!");
// Reset back to X: 5 Y: 5.
cursor.reset_position();
// Print 'Back' at X: 5 Y: 5.
print!("Back");

// hide cursor
cursor.hide();
// show cursor
cursor.show();
// blink or not blinking of the cursor (not widely supported)
cursor.blink(true)

```

### Terminal | [see more](example_link)
```rust 
use crossterm::terminal::{terminal,ClearType};
use crossterm::Context;

let mut context = Context::new();
let mut terminal = terminal(&context);

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
let terminal_size = terminal.terminal_size();
// Print results
print!("X: {}, y: {}", terminal_size.0, terminal_size.1);

// Scroll down 10 lines.
terminal.scroll_down(10);

// Scroll up 10 lines.
terminal.scroll_up(10);

// Set terminal size
terminal.set_size(10,10);

// exit the current process.
terminal.exit();

// write to the terminal whether you are on the main screen or alternate screen.
terminal.write("Some text\n Some text on new line");

// use the `paint()` for styling font
println!("{}", terminal.paint("x").with(Color::Red).on(Color::Blue));
```

For alternate screen and raw screen I recommend you to check this [link](example_link) for better examples.

## Tested terminals

- Windows Powershell
    - Windows 10 (pro)
- Windows CMD
    - Windows 10 (pro)
- Ubuntu Desktop Terminal
    - Ubuntu 17.10
- Arch linux Konsole

This crate supports all unix terminals and windows terminals down to windows XP but not all of them have been tested.
If you have used this library for an terminal other than the above list without issues feel free to add it to the above list, I really would appreciate it.
    
## How it works

Crossterm is using ANSI escape codes by default for all systems unix and windows systems. 
For Windows systems it is a different story since Windows version lower than 10 will use WinApi instead because they are not supporting ANSI escape codes. 

## Notice 
This library is library is stable. There will not be changed much in the code design so do not worry to much.
If there are any changes that affect previous versions I will describe what to change when upgrading Crossterm to an newer version.

## Todo
- Handling mouse events 
- Handling key events
- Tests
## Contributing

If you would like to contribute to Crossterm, than please design the code as it is now.
Each module contains the same structures so we can easily extend to multiple platforms. 
As you study the code you will quickly see what the architecture is. 
Maybe later there will be an documentation for Crossterm architecture design.

## Authors

* **Timon Post** - *Project Owner & creator*

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details



