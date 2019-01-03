//! Ever got disappointed when a terminal library for rust was only written for UNIX systems?
//! Crossterm provides the same core functionalities for both Windows and UNIX systems.
//!
//! Crossterm aims to be simple and easy to call in code.
//! Through the simplicity of Crossterm, you do not have to worry about the platform you are working with.
//!
//! This crate supports all UNIX and windows terminals down to windows 7 (not all terminals are tested see [Tested Terminals](#tested-terminals) for more info)
//!
//! ## Table of contents:
//! - [Getting started](#getting-started)
//! - [Useful links](#useful-links)
//! - [Features](#features)
//! - [Examples](#examples)
//!     - [Crossterm Wrapper](#crossterm-type--see-more)
//!     - [Styling](#crossterm-type--see-more)
//!     - [Cursor](#cursor--see-more)
//!     - [Input](#input--see-more)
//!     - [Terminal](#terminal--see-more)
//! - [Tested Terminals](#tested-terminals)
//! - [Notice](#notice)
//! - [Todo](#todo)
//! - [Contributing](#contributing)
//! - [Authors](#authors)
//! - [License](#license)
//!
//! ## Getting Started
//!
//! This documentation is only for Crossterm version `0.5` if you have an older version of Crossterm I suggest you check the [Upgrade Manual](https://github.com/TimonPost/crossterm/blob/master/docs/UpgradeManual.md). Also, check out the [examples](https://github.com/TimonPost/crossterm/tree/master/examples) folders with detailed examples for all functionality of this crate.
//!
//! Add the Crossterm package to your `Cargo.toml` file.
//!
//! ```
//! [dependencies]
//! crossterm = "0.5.2"
//!
//! ```
//! And import the Crossterm modules you want to use.
//!
//! ```rust
//! extern crate crossterm;
//!
//! // this module is used for styling the terminal
//! use crossterm::style::*;
//! // this module is used for cursor related actions
//! use crossterm::cursor::*;
//! // this module is used for terminal related actions
//! use crossterm::terminal::*;
//! // this module is used for input related actions
//! use crossterm::input::*;
//!
//! ```
//!
//! ### Useful Links
//!
//! - [Book](http://atcentra.com/crossterm/)
//! - [Documentation](https://docs.rs/crossterm/)
//! - [Crates.io](https://crates.io/crates/crossterm)
//! - [Program Examples](https://github.com/TimonPost/crossterm/tree/master/examples/program_examples)
//! - [Examples](https://github.com/TimonPost/crossterm/tree/master/examples)
//!
//! ## Features
//! These are the features from this crate:
//!
//! - Cross-platform
//! - Everything is multithreaded (Send, Sync)
//! - Detailed documentation on every item
//! - Very few dependenties.
//! - Cursor.
//!     - Moving _n_ times Up, Down, Left, Right
//!     - Goto a certain position
//!     - Get cursor position
//!     - Storing the current cursor position and resetting to that stored cursor position later
//!     - Hiding an showing the cursor
//!     - Control over blinking of the terminal cursor (only some terminals are supporting this)
//! - Styled output
//!     - Foreground color (16 base colors)
//!     - Background color (16 base colors)
//!     - 256 color support (Windows 10 and UNIX only)
//!     - RGB support (Windows 10 and UNIX only)
//!     - Text Attributes like: bold, italic, underscore and crossed word ect (unix only)
//! - Terminal
//!     - Clearing (all lines, current line, from cursor down and up, until new line)
//!     - Scrolling (Up, down)
//!     - Get the size of the terminal
//!     - Set the size of the terminal
//!     - Alternate screen
//!     - Raw screen
//!     - Exit the current process
//! - Input
//!     - Read character
//!     - Read line
//!     - Read async
//!     - Read async until
//!     - Wait for key event (terminal pause)
//!
//! ## Examples
//! These are some basic examples demonstrating how to use this crate. See [examples](https://github.com/TimonPost/crossterm/blob/master/examples/) for more.
//!
//! ### Crossterm Type | [see more](https://github.com/TimonPost/crossterm/blob/master/examples/some_types/mod.rs)
//! This is a wrapper for all the modules crossterm provides like terminal, cursor, styling and input.
//!
//! ```rust
//! // screen wheron the `Crossterm` methods will be executed.
//! let crossterm = Crossterm::new();
//!
//! // get instance of the modules, whereafter you can use the methods the particulary module provides.
//! let color = crossterm.color();
//! let cursor = crossterm.cursor();
//! let terminal = crossterm.terminal();
//!
//! // styling
//! println!("{}", crossterm.style("Black font on Green background color").with(Color::Black).on(Color::Green));
//!
//! ```
//! ### Styled Font | [see more](http://atcentra.com/crossterm/styling.html)
//! This module provides the functionalities to style the terminal.
//! ```rust
//! use crossterm::style::{Color, style};
//!
//! // store objcets so it could be painted later to the screen.
//! let style1 = style("Some Blue font on Black background").with(Color::Blue).on(Color::Black);
//! let style2 = style("Some Red font on Yellow background").with(Color::Red).on(Color::Yellow);
//!
//! // attributes are only supported for UNIX terminals.
//! let normal = style("Normal text");
//! let bold = style("Bold text").bold();
//! let italic = style("Italic text").italic();
//! let slow_blink = style("Slow blinking text").slow_blink();
//! let rapid_blink = style("Rapid blinking text").rapid_blink();
//! let hidden = style("Hidden text").hidden();
//! let underlined = style("Underlined text").underlined();
//! let reversed = style("Reversed text").reverse();
//! let dimmed = style("Dim text").dim();
//! let crossed_out = style("Crossed out font").crossed_out();
//!
//! // paint styled text to screen (this could also be called inline)
//! println!("{}", style1);
//! println!("{}", style2);
//! println!("{}", bold);
//! println!("{}", hidden);
//! ...
//!
//! // cursom rgb value
//! style("RGB color (10,10,10) ").with(Color::Rgb {
//!     r: 10,
//!     g: 10,
//!     b: 10
//! }));
//!
//! // custom ansi color value
//! style("ANSI color value (50) ").with(Color::AnsiValue(50));
//!
//! ```
//! ### Cursor | [see more](https://github.com/TimonPost/crossterm/blob/master/examples/cursor/mod.rs)
//! This module provides the functionalities to work with the terminal cursor.
//!
//! ```rust
//! use crossterm::cursor;
//!
//! let mut cursor = cursor();
//!
//! /// Moving the cursor
//! // Set the cursor to position X: 10, Y: 5 in the terminal
//! cursor.goto(10,5);
//!
//! // Move the cursor up,right,down,left 3 cells.
//! cursor.move_up(3);
//! cursor.move_right(3);
//! cursor.move_down(3);
//! cursor.move_left(3);
//!
//! /// Safe the current cursor position to recall later
//! // Goto X: 5 Y: 5
//! cursor.goto(5,5);
//! // Safe cursor position: X: 5 Y: 5
//! cursor.save_position();
//! // Goto X: 5 Y: 20
//! cursor.goto(5,20);
//! // Print at X: 5 Y: 20.
//! print!("Yea!");
//! // Reset back to X: 5 Y: 5.
//! cursor.reset_position();
//! // Print 'Back' at X: 5 Y: 5.
//! print!("Back");
//!
//! // hide cursor
//! cursor.hide();
//! // show cursor
//! cursor.show();
//! // blink or not blinking of the cursor (not widely supported)
//! cursor.blink(true)
//!
//! ```
//!
//! ### Input | [see more](http://atcentra.com/crossterm/input.html)
//! This module provides the functionalities to work with terminal input.
//!
//! ```rust
//! use crossterm::input;
//!
//! let mut input = input();
//!
//!  match input.read_char() {
//!     Ok(s) => println!("char typed: {}", s),
//!     Err(e) => println!("char error : {}", e),
//!  }
//!
//!  match input.read_line() {
//!      Ok(s) => println!("string typed: {}", s),
//!      Err(e) => println!("error: {}", e),
//!  }
//!
//! ```
//!
//! ### Terminal | [see more](https://github.com/TimonPost/crossterm/blob/master/examples/terminal/terminal.rs)
//! This module provides the functionalities to work with the terminal in general.
//!
//! ```rust
//! use crossterm::terminal::{terminal,ClearType};
//!
//! let mut terminal = terminal();
//!
//! // Clear all lines in terminal;
//! terminal.clear(ClearType::All);
//! // Clear all cells from current cursor position down.
//! terminal.clear(ClearType::FromCursorDown);
//! // Clear all cells from current cursor position down.
//! terminal.clear(ClearType::FromCursorUp);
//! // Clear current line cells.
//! terminal.clear(ClearType::CurrentLine);
//! // Clear all the cells until next line.
//! terminal.clear(ClearType::UntilNewLine);
//!
//! // Get terminal size
//! let (width, height) = terminal.terminal_size();
//! print!("X: {}, y: {}", width, height);
//!
//! // Scroll down, up 10 lines.
//! terminal.scroll_down(10);
//! terminal.scroll_up(10);
//!
//! // Set terminal size (width, height)
//! terminal.set_size(10,10);
//!
//! // exit the current process.
//! terminal.exit();
//!
//! // write to the terminal whether you are on the main screen or alternate screen.
//! terminal.write("Some text\n Some text on new line");
//! ```
//!
//! ### Alternate and Raw Screen
//! These concepts are a little more complex, please checkout the [book](http://atcentra.com/crossterm/screen.html) topics about these subjects.
//!
//! ## Tested terminals
//!
//! - Windows Powershell
//!     - Windows 10 (pro)
//! - Windows CMD
//!     - Windows 10 (pro)
//!     - Windows 8.1 (N)
//! - Ubuntu Desktop Terminal
//!     - Ubuntu 17.10
//! - (Arch, Manjaro) KDE Konsole
//! - Linux Mint
//!
//! This crate supports all Unix terminals and windows terminals down to Windows 7 but not all of them have been tested.
//! If you have used this library for a terminal other than the above list without issues feel free to add it to the above list, I really would appreciate it.
//!
//! ## Notice
//! This library is average stable now but I don't expect it to not to change that much.
//! If there are any changes that will affect previous versions I will [describe](https://github.com/TimonPost/crossterm/blob/master/docs/UpgradeManual.md) what to change to upgrade.
//!
//! ## Todo
//! I still have some things in mind to implement.
//!
//! - Handling mouse events
//!     I want to be able to do something based on the clicks the user has done with its mouse.
//! - Handling key events
//!     I want to be able to read key combination inputs.
//! - Tests
//!    Find a way to test: color, alternate screen, rawscreen
//!
//! ## Contributing
//!
//! I highly appreciate it when you are contributing to this crate.
//! Also Since my native language is not English my grammar and sentence order will not be perfect.
//! So improving this by correcting these mistakes will help both me and the reader of the docs.
//!
//! Check [Contributing](https://github.com/TimonPost/crossterm/blob/master/docs/Contributing.md) for more info about branches and code architecture.
//!
//! ## Authors
//!
//! * **Timon Post** - *Project Owner & creator*
//!
//! ## License
//!
//! This project is licensed under the MIT License - see the [LICENSE.md](https://github.com/TimonPost/crossterm/blob/master/LICENSE) file for details

#[cfg(unix)]
extern crate libc;
#[cfg(unix)]
extern crate termios;

#[cfg(windows)]
extern crate crossterm_winapi;
#[cfg(windows)]
extern crate winapi;

#[macro_use]
mod common;

mod kernel;
mod modules;

pub use modules::cursor;
pub use modules::input;
pub use modules::output;
pub use modules::style;
pub use modules::terminal;

pub use self::cursor::{cursor, TerminalCursor};
pub use self::input::{input, AsyncReader, KeyEvent, TerminalInput};
pub use self::output::TerminalOutput;
pub use self::style::{
    color, style, Attribute, Color, ColorType, DisplayableObject, ObjectStyle, StyledObject,
    TerminalColor,
};
pub use self::terminal::{terminal, Terminal};
pub use common::screen::{AlternateScreen, Screen};
pub use common::Crossterm;
