# Crossterm | cross-platform terminal manipulating library.
 ![Lines of Code][s7] [![Latest Version][s1]][l1] [![MIT][s2]][l2] [![docs][s3]][l3] ![Lines of Code][s6] [![Join us on Discord][s5]][l5]

[s1]: https://img.shields.io/crates/v/crossterm.svg
[l1]: https://crates.io/crates/crossterm

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: crossterm/LICENSE

[s3]: https://docs.rs/crossterm/badge.svg
[l3]: https://docs.rs/crossterm/

[s3]: https://docs.rs/crossterm/badge.svg
[l3]: https://docs.rs/crossterm/

[s5]: https://img.shields.io/discord/560857607196377088.svg?logo=discord
[l5]: https://discord.gg/K4nyTDB

[s6]: https://tokei.rs/b1/github/TimonPost/crossterm?category=code
[s7]: https://travis-ci.org/TimonPost/crossterm.svg?branch=master

Have you ever been disappointed when a terminal library for rust was only written for UNIX systems? 
Crossterm provides clearing, input handling, styling, cursor movement, and terminal actions for both Windows and UNIX systems.

Crossterm aims to be simple and easy to call in code. 
Through the simplicity of Crossterm, you do not have to worry about the platform you are working with.

This crate supports all UNIX and Windows terminals down to Windows 7 (not all terminals are tested see [Tested Terminals](#tested-terminals) for more info).

This crate consists of five modules that are provided behind [feature flags](http://atcentra.com/crossterm/feature_flags.html) so that you can define which features you'd like to have; by default, all features are enabled.
- [Crossterm Style](https://crates.io/crates/crossterm_style) 
- [Crossterm Input](https://crates.io/crates/crossterm_input) 
- [Crossterm Screen](https://crates.io/crates/crossterm_screen)
- [Crossterm Cursor](https://crates.io/crates/crossterm_cursor)
- [Crossterm Terminal](https://crates.io/crates/crossterm_terminal)

## Table of contents:
- [Getting started](#getting-started)
- [Useful links](#useful-links)
- [Features](#features)
- [Examples](#examples)
    - [Crossterm Wrapper](#crossterm-type--see-more)
    - [Styling](#crossterm-type--see-more)
    - [Cursor](#cursor--see-more)
    - [Input](#input--see-more)
    - [Terminal](#terminal--see-more)
- [Tested Terminals](#tested-terminals)
- [Notice](#notice)
- [Todo](#todo)
- [Contributing](#contributing)
- [Authors](#authors)
- [License](#license)

## Getting Started

This documentation is only for Crossterm version `0.9`. If you have an older version of Crossterm, then I suggest you check the [Upgrade Manual](https://github.com/TimonPost/crossterm/blob/master/docs/UPGRADE.md). Also, check out the [examples](https://github.com/TimonPost/crossterm/tree/master/examples) folders with detailed examples for all functionality of this crate.

Add the Crossterm package to your `Cargo.toml` file.

```
[dependencies]
crossterm = "0.9"
```

### Useful Links

- [Book](http://atcentra.com/crossterm/)
- [Documentation](https://docs.rs/crossterm/)
- [Crates.io](https://crates.io/crates/crossterm)
- [Program Examples](https://github.com/TimonPost/crossterm/tree/master/examples/program_examples)
- [Examples](https://github.com/TimonPost/crossterm/tree/master/examples)

## Features
These are the features from this crate:

- Cross-platform
- Multithreaded (send, sync)
- Detailed Documentation
- Few Dependencies
- Cursor
    - Moving _n_ times (up, down, left, right)
    - Position (set/get)
    - Store cursor position and resetting to that later
    - Hiding/Showing
    - Blinking Cursor (supported by only some terminals)
- Styled output
    - Foreground Color (16 base colors)
    - Background Color (16 base colors)
    - 256 (ANSI) Color Support (Windows 10 and UNIX Only)
    - RGB Color Support (Windows 10 and UNIX only)
    - Text Attributes: bold, italic, underscore and crossed word and [more](http://atcentra.com/crossterm/styling.html#attributes) (Windows 10 and UNIX only)
- Terminal
    - Clearing (all lines, current line, from cursor down and up, until new line)
    - Scrolling (up, down)
    - Terminal Size (get/set)
    - Alternate Screen
    - Raw Screen   
    - Exit Current Process
- Input
    - Read character
    - Read line
    - Read key input events (async / sync)
    - Read mouse input events (press, release, position, button)

## Examples
These are some basic examples demonstrating how to use this crate. See [examples](https://github.com/TimonPost/crossterm/blob/master/examples/) for more.

### Crossterm Type
This is a wrapper for all the modules crossterm provides like terminal, cursor, styling and input.

Good documentation can be found at the following places: [docs](https://docs.rs/crossterm/), [examples](https://github.com/TimonPost/crossterm/blob/master/examples/crossterm.rs).

```rust
// screen whereon the `Crossterm` methods will be executed.
let crossterm = Crossterm::new();

// get instance of the modules, whereafter you can use the methods the particularly module provides. 
let color = crossterm.color();
let cursor = crossterm.cursor();
let terminal = crossterm.terminal();
let input = crossterm.input();
```

### Styled Font
This module enables you to style the terminal font.

Good documentation can be found at the following places: [docs](https://docs.rs/crossterm_style/), [book](http://atcentra.com/crossterm/styling.html), [examples](https://github.com/TimonPost/crossterm/tree/master/examples/key_events.rs)

_imports_
```rust 
use crossterm::{Colored, Color, Colorize, Styler, Attribute};
```
_style font with attributes_
```rust
// pass any `Attribute` value to the formatting braces.
println!("{} Underlined {} No Underline", Attribute::Underlined, Attribute::NoUnderline);

// you could also call different attribute methods on a `&str` and keep on chaining if needed.
let styled_text = "Bold Underlined".bold().underlined();
println!("{}", styled_text);

// old-way but still usable
let styled_text = style("Bold Underlined").bold().underlined();
```

_style font with colors_
```rust
println!("{} Red foreground color", Colored::Fg(Color::Red));
println!("{} Blue background color", Colored::Bg(Color::Blue));

// you can also call different coloring methods on a `&str`.
let styled_text = "Bold Underlined".red().on_blue();
println!("{}", styled_text);

// old-way but still usable
let styled_text = style("Bold Underlined").with(Color::Red).on(Color::Blue);
```
_style font with RGB and ANSI Value_
```rust
// custom rgb value (Windows 10 and UNIX systems)
println!("{} some colored text", Colored::Fg(Color::Rgb {
    r: 10,
    g: 10,
    b: 10
}));

// custom ansi color value (Windows 10 and UNIX systems)
println!("{} some colored text", Colored::Fg(Color::AnsiValue(10)));
```


### Cursor
This module enables you to work with the terminal cursor.

Good documentation could be found on the following places: [docs](https://docs.rs/crossterm_cursor/), [examples](https://github.com/TimonPost/crossterm/tree/master/examples/cursor.rs)

```rust 
use crossterm::cursor;

let mut cursor = cursor();

/// Moving the cursor
// Set the cursor to position X: 10, Y: 5 in the terminal
cursor.goto(10,5);

// Move the cursor up,right,down,left 3 cells.
cursor.move_up(3);
cursor.move_right(3);
cursor.move_down(3);
cursor.move_left(3);

/// Safe the current cursor position to recall later
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

### Terminal
This module enables you to work with the terminal in general.

Good documentation could be found on the following places: [docs](https://docs.rs/crossterm_terminal/), [examples](https://github.com/TimonPost/crossterm/tree/master/examples/terminal.rs).

```rust 
use crossterm::{terminal,ClearType};

let mut terminal = terminal();

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
let (width, height) = terminal.terminal_size();
print!("X: {}, y: {}", width, height);

// Scroll down, up 10 lines.
terminal.scroll_down(10);
terminal.scroll_up(10);

// Set terminal size (width, height)
terminal.set_size(10,10);

// exit the current process.
terminal.exit();

// write to the terminal whether you are on the main screen or alternate screen.
terminal.write("Some text\n Some text on new line");
```

### Input Reading
This module enables you to read user input events.

Good documentation could be found on the following places: [docs](https://docs.rs/crossterm_input/), [book](http://atcentra.com/crossterm/input.html), [examples](https://github.com/TimonPost/crossterm/tree/master/examples/key_events.rs)

_available imports_
```rust
use crossterm_input::{
    input, InputEvent, KeyEvent, MouseButton, MouseEvent, TerminalInput, AsyncReader, SyncReader, Screen
};
```

_Simple Readings_
```rust 
let mut input = input();

 match input.read_char() {
    Ok(s) => println!("char typed: {}", s),
    Err(e) => println!("char error : {}", e),
 }
 
 match input.read_line() {
     Ok(s) => println!("string typed: {}", s),
     Err(e) => println!("error: {}", e),
 }
```

_Read input events synchronously or asynchronously._
```rust
// make sure to enable raw mode, this will make sure key events won't be handled by the terminal it's self and allows crossterm to read the input and pass it back to you.
let screen = RawScreen::into_raw_mode();
    
let mut input = input();

// either read the input synchronously 
let stdin = input.read_sync();
 
// or asynchronously
let stdin = input.read_async();

if let Some(key_event) = stdin.next() {
     match key_event {
         InputEvent::Keyboard(event: KeyEvent) => match event { /* check key event */ }
         InputEvent::Mouse(event: MouseEvent) => match event { /* check mouse event */ }
     }
 }
```

_Enable mouse input events._
```rust
let input = input();

// enable mouse events to be captured.
input.enable_mouse_mode().unwrap();

// disable mouse events to be captured.
input.disable_mouse_mode().unwrap();
```

### Alternate and Raw Screen
These concepts are a little more complex and would take over the README, please checkout the [docs](https://docs.rs/crossterm_screen/), [book](http://atcentra.com/crossterm/screen.html), and [examples](https://github.com/TimonPost/crossterm/tree/master/examples).

## Tested terminals

- Windows Powershell
    - Windows 10 (pro)
- Windows CMD
    - Windows 10 (pro)
    - Windows 8.1 (N)
- Ubuntu Desktop Terminal
    - Ubuntu 17.10
- (Arch, Manjaro) KDE Konsole
- Linux Mint

This crate supports all Unix terminals and Windows terminals down to Windows 7; however, not all of the terminals have been tested.
If you have used this library for a terminal other than the above list without issues, then feel free to add it to the above list - I really would appreciate it!

## Notice 
This library is mostly stable now, and I don't expect it to change much.
If there are any changes that will affect previous versions I will [describe](https://github.com/TimonPost/crossterm/blob/master/docs/UPGRADE.md) what to change to upgrade.

## Todo
- Tests
   Find a way to test: color, alternate screen, rawscreen

## Contributing
  
I highly appreciate it when you contribute to this crate. 
Also, since my native language is not English my grammar and sentence order will not be perfect. 
So improving this by correcting these mistakes will help both me and the reader of the docs.

Check [Contributing](https://github.com/TimonPost/crossterm/blob/master/docs/Contributing.md) for more info about branches and code architecture.

## Authors

* **Timon Post** - *Project Owner & creator*

## License

This project, crossterm and all it's sub-modules: crossterm_screen, crossterm_cursor, crossterm_style, crossterm_input, crossterm_terminal, crossterm_winapi, crossterm_utils are licensed under the MIT License - see the [LICENSE.md](https://github.com/TimonPost/crossterm/blob/master/LICENSE) file for details
