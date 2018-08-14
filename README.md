# Crossterm | crossplatform terminal library written in rust.

[![Latest Version](https://img.shields.io/crates/v/crossterm.svg)](https://crates.io/crates/crossterm) | [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE) | [![docs.rs](https://docs.rs/crossterm/badge.svg)](https://docs.rs/crossterm/) | [Examples](https://github.com/TimonPost/crossterm/tree/master/examples) | [Changelog](https://github.com/TimonPost/crossterm/blob/master/docs/UpgradeManual.md) | [Release Nodes](https://github.com/TimonPost/crossterm/tree/master/docs)
|----|----|----|----|----|----

Ever got disappointed when a terminal library for rust was only written for unix systems? 
Crossterm provides the same core functionalities for both windows and unix systems.

Crossterm aims to be simple and easy to call in code. 
Though the simplicity of Crossterm, you do not have to worry about the platform you are working with.
You can just call whatever action you want and behind the scenes it will check what to do based on the current platform.

This crate supports all unix and windows terminals down to windows XP (not all terminals are tested see 'Tested Terminals' for more info)

## Table of contents:
- [Getting started](https://github.com/TimonPost/crossterm#getting-started)
- [Useful links](https://github.com/TimonPost/crossterm#useful-links)
- [Features](https://github.com/TimonPost/crossterm#features)
- [Examples](https://github.com/TimonPost/crossterm#examples)
    - [Crossterm Wrapper](https://github.com/TimonPost/crossterm#crossterm-wrapper--see-more)
    - [Styling](https://github.com/TimonPost/crossterm#styled-font--see-more)
    - [Cursor](https://github.com/TimonPost/crossterm#cursor--see-more)
    - [Terminal](https://github.com/TimonPost/crossterm#terminal--see-more)
- [Tested Terminals](https://github.com/TimonPost/crossterm#tested-terminals)
- [How it works](https://github.com/TimonPost/crossterm#how-it-works)
- [Notice](https://github.com/TimonPost/crossterm#notice)
- [Todo](https://github.com/TimonPost/crossterm#todo)
- [Contributing](https://github.com/TimonPost/crossterm#contributing)
- [Authors](https://github.com/TimonPost/crossterm#authors)
- [License](https://github.com/TimonPost/crossterm#license)

## Getting Started

This documentation is only for Crossterm version `0.4.0` if you have an older version of Crossterm I suggest you check the [Upgrade Manual](https://github.com/TimonPost/crossterm/blob/master/docs/UpgradeManual.md) for more information about how to upgrade to a newer version or check the [examples](https://github.com/TimonPost/crossterm/tree/master/examples) folders which contains a readme for every specific version. Also, you could take a look at the [examples](https://github.com/TimonPost/crossterm/tree/master/examples) from the previous versions of Crossterm.


Add the Crossterm package to your `Cargo.toml` file.

```
[dependencies]
crossterm = "0.4.0"

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
// this mudule is used for input related actions
use self::crossterm::terminal::*;
// this type could be used to access the above modules.
use self::crossterm::Crossterm;

```

## Useful Links

- Code [documentation](link).
- Code [Examples]() (see [branches](LINK_TO_BRANCHES) for previous versions)
- [Cargo Page](https://crates.io/crates/crossterm)
- [Program Examples](https://github.com/TimonPost/crossterm/tree/master/examples/program_examples)

# Features
These are the features from this crate:

- Cursor.
    - Moving _n_ times Up, Down, Left, Right.
    - Goto a certain position.
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
    - Get the size of the terminal
    - Set the size of the terminal.
    - Alternate screen
    - Raw screen    
- Input
    - Read character
    - Read line
    - Read async
    - Read async until
- Exit the current process.
- Detailed documentation on every item.
- Crossplatform

## Examples

For detailed examples of all Crossterm functionalities check the [examples](https://github.com/TimonPost/crossterm/tree/master/examples) directory.

### Crossterm Type | [see more](Link)
This is a wrapper for all the modules crossterm provides like terminal, cursor, styling and input.
```
// screen wheron the `Crossterm` methods will be executed.
let screen = Screen::default();
let crossterm = Crossterm::new(&screen);

// get instance of the modules, whereafter you can use the methods the particulary module provides. 
let color = crossterm.color();
let cursor = crossterm.cursor();
let terminal = crossterm.terminal();

// styling
let style = crossterm.style("Black font on Green background color").with(Color::Black).on(Color::Green);
style.paint(&screen);

```
### Styled Font | [see more](Link)
This module provides the functionalities to style the terminal cursor.
```rust    
use crossterm::style::{Color, style};
use crossterm::Screen; 

// store objcets so it could be painted later to the screen.   
let style1 = style("Some Blue font on Black background").with(Color::Blue).on(Color::Black);
let style2 = style("Some Red font on Yellow background").with(Color::Red).on(Color::Yellow);

let screen = Screen::default();

/// ! The following code only works for unix based systems.
// some attributes
let normal = style("Normal text");
let bold = style("Bold text").bold();
let italic = style("Italic text").italic();
let slow_blink = style("Slow blinking text").slow_blink();
let rapid_blink = style("Rapid blinking text").rapid_blink();
let hidden = style("Hidden text").hidden();
let underlined = style("Underlined text").underlined();
let reversed = style("Reversed text").reverse();
let dimmed = style("Dim text").dim();
let crossed_out = style("Crossed out font").crossed_out();

// paint styled text to screen (this could also be called inline)
style1.paint(&screen);
style2.paint(&screen);
bold.paint(&screen);
hidden.paint(&screen);

// cursom rgb value
style("RGB color (10,10,10) ").with(Color::Rgb {
    r: 10,
    g: 10,
    b: 10
}).paint(&screen);

// custom ansi color value
style("ANSI color value (50) ").with(Color::AnsiValue(50)).paint(&screen);

```
### Cursor | [see more](LINK)
This module provides the functionalities to work with the terminal cursor.

```rust 

use crossterm::Screen;
use crossterm::cursor::cursor;

// create Screen to wheron the `cursor()` should function.
let screen = Screen::default();
let mut cursor = cursor(&screen);

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

### Terminal | [see more](LINK)
This module provides the functionalities to work with the terminal in general.

```rust 
use crossterm::terminal::{terminal,ClearType};
use crossterm::Screen;

// create Screen to wheron the `terminal()` should function.
let screen = Screen::default();
let mut terminal = terminal(&screen);

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

Check these links: [AlternateScreen](https://github.com/TimonPost/crossterm/blob/master/examples/Crossterm%200.3.0/terminal/alternate_screen.rs) and [RawScreen](https://github.com/TimonPost/crossterm/blob/master/examples/Crossterm%200.3.0/terminal/raw_mode.rs) for information about how to work with these features.

## Tested terminals

- Windows Powershell
    - Windows 10 (pro)
- Windows CMD
    - Windows 10 (pro)
- Ubuntu Desktop Terminal
    - Ubuntu 17.10
- Arch linux Konsole

This crate supports all Unix terminals and windows terminals down to Windows XP but not all of them have been tested.
If you have used this library for a terminal other than the above list without issues feel free to add it to the above list, I really would appreciate it.

## Notice 
This library is not stable yet but I expect it to not to change that much anymore. 
And if there are any changes that affect previous versions I will [describe](https://github.com/TimonPost/crossterm/blob/master/docs/UpgradeManual.md) what to change when upgrading Crossterm to a newer version.

## Todo
I still have some things in mind to implement. 

- Handling mouse events 
    I want to be able to do something based on the clicks the user has done with its mouse.
- Handling key events
    I want to be able to read key combination inputs. 
- Tests
    Also, I want to have tests for this crate, and yes maybe a little late :). But I find it difficult to test some functionalities because how would you ever test if the screen is indeed int alternate, raw modes or how would you ever test if the terminal cursor is moved certainly.

## Contributing
  
I highly appreciate it when you are contributing to this crate. 
Also Since my native language is not English my grammar and sentence order will not be perfect. 
So improving this by correcting these mistakes will help both me and the reader of the docs.

Check [Contributing](link) for more info about branches and code architecture.

## Authors

* **Timon Post** - *Project Owner & creator*

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details




