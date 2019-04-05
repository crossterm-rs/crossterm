# Crossterm Input | cross-platform input reading .
 ![Lines of Code][s7] [![Latest Version][s1]][l1] [![MIT][s2]][l2] [![docs][s3]][l3]

[s1]: https://img.shields.io/crates/v/crossterm_input.svg
[l1]: https://crates.io/crates/crossterm_input

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: ./LICENSE

[s3]: https://docs.rs/crossterm_input/badge.svg
[l3]: https://docs.rs/crossterm_input/

[s3]: https://docs.rs/crossterm_input/badge.svg
[l3]: https://docs.rs/crossterm_input/

[s7]: https://travis-ci.org/TimonPost/crossterm.svg?branch=master

This crate allows you to read the user input cross-platform. 
It supports all UNIX and windows terminals down to windows 7 (not all terminals are tested see [Tested Terminals](#tested-terminals) for more info)

This crate is a sub-crate of [crossterm](https://crates.io/crates/crossterm) to read the user input and can be used individually.

Other sub-crates are:
- [Crossterm Style](https://crates.io/crates/crossterm_style) 
- [Crossterm Terminal](https://crates.io/crates/crossterm_terminal) 
- [Crossterm Screen](https://crates.io/crates/crossterm_screen)
- [Crossterm Cursor](https://crates.io/crates/crossterm_cursor)
 
When you want to use other modules as well you might want to use crossterm with [feature flags](http://atcentra.com/crossterm/feature_flags.html).
 
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

This documentation is only for `crossterm_input` version `0.2` if you have an older version I suggest you check the [Upgrade Manual](https://github.com/TimonPost/crossterm/blob/master/docs/UPGRADE.md). Also, check out the [examples](https://github.com/TimonPost/crossterm/tree/master/crossterm_input/examples) folders with detailed examples for all functionalities of this crate.

Add the `crossterm_input` package to your `Cargo.toml` file.

```
[dependencies]
`crossterm_input` = "0.2"
```
And import the `crossterm_input` modules you want to use.

```rust  
extern crate crossterm_input;

pub use crossterm_input::{input, AsyncReader, KeyEvent, TerminalInput};
```

### Useful Links

- [Documentation](https://docs.rs/crossterm_input/)
- [Crates.io](https://crates.io/crates/crossterm_input)
- [Book](http://atcentra.com/crossterm/input.html)
- [Examples](/examples)

## Features
These are the features of this crate:

- Cross-platform
- Everything is multithreaded (Send, Sync)
- Detailed documentation on every item
- Input
    - Read character
    - Read line
    - Read key input events async / sync (ALT + Key, CTRL + Key, FN, Arrows, ESC, BackSpace, HOME, DELETE. INSERT, PAGEUP/DOWN, and more)
    - Read mouse input events (Press, Release, Position, Button)
    
## Examples
The examples folder has more complete and verbose examples, please have a look at that as well.

Good documentation could be found in the  [docs][l3] as well in the [book](http://atcentra.com/crossterm/input.html).

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
let screen = Screen::new(true);
    
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
If there are any changes that will affect previous versions I will [describe](https://github.com/TimonPost/crossterm/blob/master/docs/UPGRADE.md) what to change to upgrade.

## Contributing

I highly appreciate it when you are contributing to this crate. 
Also Since my native language is not English my grammar and sentence order will not be perfect. 
So improving this by correcting these mistakes will help both me and the reader of the docs.

## Authors

* **Timon Post** - *Project Owner & creator*
* **Dave Ho** - *Contributor*

## License

This project is licensed under the MIT License - see the [LICENSE.md](https://github.com/TimonPost/crossterm/blob/master/LICENSE) file for details
