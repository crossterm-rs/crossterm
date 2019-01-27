# Crossterm Terminal | cross-platform terminal actions.
 ![Lines of Code][s7] [![Latest Version][s1]][l1] [![MIT][s2]][l2] [![docs][s3]][l3] ![Lines of Code][s6]

[s1]: https://img.shields.io/crates/v/crossterm_terminal.svg
[l1]: https://crates.io/crates/crossterm_terminal

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: ./LICENSE

[s3]: https://docs.rs/crossterm_terminal/badge.svg
[l3]: https://docs.rs/crossterm_terminal/

[s3]: https://docs.rs/crossterm_terminal/badge.svg
[l3]: https://docs.rs/crossterm_terminal/

[s6]: https://tokei.rs/b1/github/TimonPost/crossterm_terminal?category=code
[s7]: https://travis-ci.org/TimonPost/crossterm_terminal.svg?branch=master

This crate allows you to perform terminal related actions cross-platform e.g clearing, resizing etc. 
It supports all UNIX and windows terminals down to windows 7 (not all terminals are tested see [Tested Terminals](#tested-terminals) for more info)

This crate is a sub-crate of [crossterm](https://crates.io/crates/crossterm) to perform terminal related actions, and can be use individually.

Other sub-crates are:
- [Crossterm Style](https://crates.io/crates/crossterm_style) 
- [Crossterm Input](https://crates.io/crates/crossterm_input) 
- [Crossterm Screen](https://crates.io/crates/crossterm_screen)
- [Crossterm Cursor](https://crates.io/crates/crossterm_cursor)
 
When you want to use other modules as well you might want to use crossterm with [feature flags](https://doc.rust-lang.org/1.30.0/book/first-edition/conditional-compilation.html)
 
## Table of contents:
- [Getting started](#getting-started)
- [Useful links](#useful-links)
- [Features](#features)
- [Examples](#examples)
- [Tested Terminals](#tested-terminals)
- [Notice](#notice)
- [Contributing](#contributing)
- [Authors](#authors)
- [License](#license)

## Getting Started

This documentation is only for `crossterm_terminal` version `0.1` if you have an older version I suggest you check the [Upgrade Manual](https://github.com/TimonPost/crossterm/blob/master/docs/UpgradeManual.md). Also, check out the [examples](https://github.com/TimonPost/crossterm/tree/master/examples) folders with detailed examples for all functionality of this crate.

Add the `crossterm_terminal` package to your `Cargo.toml` file.

```
[dependencies]
`crossterm_terminal` = "0.1"

```
And import the `crossterm_terminal` modules you want to use.

```rust  
extern crate crossterm_terminal;

pub use crossterm_terminal::{terminal, Terminal, ClearType};
```

### Useful Links

- [Documentation](https://docs.rs/crossterm_terminal/)
- [Crates.io](https://crates.io/crates/crossterm_terminal)
- [Examples](/examples)

## Features
These are the features of this crate:

- Cross-platform
- Everything is multithreaded (Send, Sync)
- Detailed documentation on every item
- Very few dependenties.
- Terminal
    - Clearing (all lines, current line, from cursor down and up, until new line)
    - Scrolling (Up, down)
    - Get the size of the terminal
    - Set the size of the terminal
    - Alternate screen
    - Raw screen   
    - Exit the current process

## Examples
Check out the [examples](/examples/) for more information about how to use this crate.

```rust 
use crossterm::terminal::{terminal,ClearType};

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

This crate supports all Unix terminals and windows terminals down to Windows 7 but not all of them have been tested.
If you have used this library for a terminal other than the above list without issues feel free to add it to the above list, I really would appreciate it.

## Notice 

This library is average stable now, I don't expect it to not to change that much. 
If there are any changes that will affect previous versions I will [describe](https://github.com/TimonPost/crossterm/blob/master/docs/UpgradeManual.md) what to change to upgrade.

## Contributing

I highly appreciate it when you are contributing to this crate. 
Also Since my native language is not English my grammar and sentence order will not be perfect. 
So improving this by correcting these mistakes will help both me and the reader of the docs.

## Authors

* **Timon Post** - *Project Owner & creator*

## License

This project is licensed under the MIT License - see the [LICENSE.md](https://github.com/TimonPost/crossterm/blob/master/LICENSE) file for details
