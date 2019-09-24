# Crossterm Cursor | cross-platform cursor movement.
 ![Lines of Code][s7] [![Latest Version][s1]][l1] [![MIT][s2]][l2] [![docs][s3]][l3] [![Join us on Discord][s5]][l5]

[s1]: https://img.shields.io/crates/v/crossterm_cursor.svg
[l1]: https://crates.io/crates/crossterm_cursor

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: ./LICENSE

[s3]: https://docs.rs/crossterm_cursor/badge.svg
[l3]: https://docs.rs/crossterm_cursor/

[s5]: https://img.shields.io/discord/560857607196377088.svg?logo=discord
[l5]: https://discord.gg/K4nyTDB

[s7]: https://travis-ci.org/crossterm-rs/crossterm.svg?branch=master

This crate allows you to move the terminal cursor cross-platform. 
It supports all UNIX and windows terminals down to windows 7 (not all terminals are tested see [Tested Terminals](#tested-terminals) for more info)

This crate is a sub-crate of [crossterm](https://crates.io/crates/crossterm) to move the cursor, and can be use individually.

Other sub-crates are:
- [Crossterm Style](https://crates.io/crates/crossterm_style) 
- [Crossterm Terminal](https://crates.io/crates/crossterm_terminal) 
- [Crossterm Screen](https://crates.io/crates/crossterm_screen)
- [Crossterm Input](https://crates.io/crates/crossterm_input)
 
When you want to use other modules as well you might want to use crossterm with [feature flags](https://timonpost.github.io/crossterm/docs/feature_flags.html).
 
## Table of contents:
- [Getting started](#getting-started)
- [Useful links](#useful-links)
- [Features](#features)
- [Examples](#examples)
- [Tested Terminals](#tested-terminals)
- [Authors](#authors)
- [License](#license)

## Getting Started

This documentation is only for `crossterm_cursor` version `0.2`. Also, check out the [examples](examples/cursor.rs) folders with detailed examples for all functionality of this crate.

Add the `crossterm_cursor` package to your `Cargo.toml` file.

```
[dependencies]
crossterm_cursor = "0.2"
```
Import the `crossterm_cursor` modules you want to use.

```rust  
pub use crossterm_cursor::{cursor, TerminalCursor};
```

### Useful Links

- [Documentation](https://docs.rs/crossterm_cursor/)
- [Crates.io](https://crates.io/crates/crossterm_cursor)
- [Examples](/examples)

## Features
These are the features of this crate:

- Cross-platform
- Multithreaded (send, sync)
- Detailed Documentation
- Few Dependencies
- Cursor
    - Moving _n_ times (up, down, left, right)
    - Position (set/get)
    - Store cursor position and resetting to that later
    - Hiding/Showing
    - Blinking Cursor (only some terminals are supporting this)
   
## Command API

My first recommendation is to use the [command API](https://timonpost.github.io/crossterm/docs/command.html) because this might replace some of the existing API in the future. 
Because it is more convenient, faster, and easier to use.

## Examples 
The [examples](https://github.com/crossterm-rs/crossterm/tree/master/examples) folder has more complete and verbose examples.

```rust 
use crossterm_cursor::cursor;

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
cursor.restore_position();
// Print 'Back' at X: 5 Y: 5.
print!("Back");

// hide cursor
cursor.hide();
// show cursor
cursor.show();
// blink or not blinking of the cursor (not widely supported)
cursor.blink(true)

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

## Authors

* **Timon Post** - *Project Owner & creator*

## License
This project is licensed under the MIT License - see the [LICENSE.md](./LICENSE) file for details
