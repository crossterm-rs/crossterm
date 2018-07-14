[![Latest Version](https://img.shields.io/crates/v/crossterm.svg)](https://crates.io/crates/crossterm) | [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE) | [![docs.rs](https://docs.rs/crossterm/badge.svg)](https://docs.rs/crossterm/) | [Examples](https://github.com/TimonPost/crossterm/tree/master/examples) | [Changelog](https://github.com/TimonPost/crossterm/blob/master/docs/UpgradeManual.md) | [Release Nodes](https://github.com/TimonPost/crossterm/tree/master/docs)
|----|----|----|----|----|----

Ever got disappointed when a terminal library for rust was only written for unix systems? 
Crossterm provides the same core functionalities for both windows and unix systems.

Crossterm aims to be simple and easy to call in code. 
True the simplicity of Crossterm you do not have to worry about the platform your working with. 
You can just call whatever action you want and underwater it will check what to do based on the current platform.

This crate supports all unix and windows terminals down to windows XP (not not all terminals are tested see 'Tested Terminals' for more info)

### Table of contents:
- [Getting started](https://github.com/TimonPost/crossterm#getting-started)
- [Usefull links](https://github.com/TimonPost/crossterm#useful-links)
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

## IMPORTANT When updating to version `0.2.3` 
Version `0.2.3` of Crossterm will have API braking changes. If you are reading this and you version whas autmaticly incremented to the new version, there will be some broken code. An Quiqe solution? no unless you did not used this crate that much. If it is important to keep running you code and you don't want to spend some time to fix the errors I recomend you to keep using version `0.2.2` because in version `0.2.3` alternate screen and some other features are introduced. And if you are not using any of these feature just downgrade. If however you want to have the newest version you can check the [upgrade manual](https://github.com/TimonPost/crossterm/blob/master/docs/UpgradeManual.md) on how to upgrade to te new version. Check [release node](https://github.com/TimonPost/crossterm/blob/master/docs/ReleaseNotesVersion%200.2.3.md) why thise Api braking changes where nessairly.

## Getting Started

This documentation is only for Crossterm version `0.2.3` if you have an older version of Crossterm I suggest you to check the [Upgrade Manual](https://github.com/TimonPost/crossterm/blob/master/docs/UpgradeManual.md) for more infomation about how to upgrade to an newer version or checkout the [README.md](https://github.com/TimonPost/crossterm/tree/master/docs) from the previous versions. Also you could take a look at the [examples](https://github.com/TimonPost/crossterm/tree/master/examples) wich have examples from the previous versions of Crossterm.


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
- Code Examples: 
version [0.1.0](https://github.com/TimonPost/crossterm/tree/master/examples/Crossterm%200.1.0), 
version [0.2.0](https://github.com/TimonPost/crossterm/tree/master/examples/Crossterm%200.2.0), 
version [0.2.1](https://github.com/TimonPost/crossterm/tree/master/examples/Crossterm%200.2.1) 
and version [0.2.3](https://github.com/TimonPost/crossterm/tree/master/examples/Crossterm%200.2.3)

- [Cargo Page](https://crates.io/crates/crossterm)
- [Real life examples](https://github.com/TimonPost/crossterm/tree/master/examples/Crossterm%200.2.3/program_examples)

# Features
These are the fatures from this crate:

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

### Crossterm wrapper | [see more](https://github.com/TimonPost/crossterm/blob/master/examples/Crossterm%200.2.3/crossterm_type/mod.rs)
This is a wrapper for the modules crossterm provides. This is introduced to mange the [`Context`](link_to_context) for the user.
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
### Styled font | [see more](https://github.com/TimonPost/crossterm/blob/master/examples/Crossterm%200.2.3/color/mod.rs)
This module provides the functionalities to style the terminal cursor.
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
### Cursor | [see more](https://github.com/TimonPost/crossterm/blob/master/examples/Crossterm%200.2.3/cursor/mod.rs)
This module provides the functionalities to work with the terminal cursor.

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

### Terminal | [see more](https://github.com/TimonPost/crossterm/blob/master/examples/Crossterm%200.2.3/terminal/terminal.rs)
This module provides the functionalities to work with the terminal in general.

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

Check these links: [AlternateScreen](https://github.com/TimonPost/crossterm/blob/master/examples/Crossterm%200.2.3/terminal/alternate_screen.rs) and [RawScreen](https://github.com/TimonPost/crossterm/blob/master/examples/Crossterm%200.2.3/terminal/raw_mode.rs) for information about how to work with these features.

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
Crossterm is using ANSI escape codes by default for both unix and windows systems. 
But for Windows it is a bit more complicater since Windows versions 8 or lower are not supporting ANSI escape codes. This is why we use WinApi for those machines. For Windows 10 ANSI codes will be the default.

## Notice 
This libary is not stable yet but I expect it ot not to change that mutch anymore. 
And if there are any changes that affect previous versions I will [describe](https://github.com/TimonPost/crossterm/blob/master/docs/UpgradeManual.md) what to change when upgrading Crossterm to an newer version.

## Todo
I still have some things in mind to implement. 

- Handling mouse events 

    I want to be able to do something based on the clicks the use has done with is mouse.
- Handling key events

    I want to be able read key combination inputs. 
- reading from the console.

    I want to be able to read the input of the console.
- Error handling

    Currently I am not doing that mutch with returend errors. This is bad since I suspect that everyting is working. I want to mange this better. When you build this crate you will see the warnings about not used return values. This is what needs to be improved.
- Tests

    Also I want to have tests for this crate, and yes maybe a little late :). But I find it difficult to test some functionalities because how would you ever test if the screen is indeed int alternate, raw modes or how would you ever test if the terminal cursor is moved certainly.

## Contributing

If you would like to contribute to Crossterm, than please design the code as it is now. 
For example a module like cursor has the following file stucture:
- mod.rs

  This file contains some trait, in this case `ITerminalCursor`, for other modules to implement. So that it can work at a specific platform.
  
- cursor.rs

  The end user will call this module to access the cursor functionalities. This module will deside withch implementation to use based on the current platform.
- winapi_cursor

  This is the cursor trait (located in mod.rs) implementation with winapi.
- ansi_cursor

  This is the cursor trait (located in mod.rs) implementation with ANSI escape codes.
  
The above structure is the same for the terminal, color, manager modules. 

Why I have chosen for this design:
- Because you can easaly extend to muliple platforms by implementing the trait int the mod.rs.
- You keep the functionalites for different platforms speperated in different files. 
- Also you have one API the user can call like in the `cursor.rs` above. This file should be avoided to change that mutch. All the other code could change alot because it has not impact on the user side.
  
I higly appriciate when you contributing to this crate. Also Since my native language is not Enlishe my grammer and sentence order will not be perfect. So improving this by correcting these mistakes will help both me and the reader of the docs.

## Authors

* **Timon Post** - *Project Owner & creator*

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details



