# Crossterm Screen | cross-platform alternate, raw screen.
 ![Lines of Code][s7] [![Latest Version][s1]][l1] [![MIT][s2]][l2] [![docs][s3]][l3] ![Lines of Code][s6]

[s1]: https://img.shields.io/crates/v/crossterm_screen.svg
[l1]: https://crates.io/crates/crossterm_screen

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: ./LICENSE

[s3]: https://docs.rs/crossterm_screen/badge.svg
[l3]: https://docs.rs/crossterm_screen/

[s3]: https://docs.rs/crossterm_screen/badge.svg
[l3]: https://docs.rs/crossterm_screen/

[s6]: https://tokei.rs/b1/github/TimonPost/crossterm_screen?category=code
[s7]: https://travis-ci.org/TimonPost/crossterm_screen.svg?branch=master

This crate allows you to work with alternate and raw screen cross-platform. 
It supports all UNIX and windows terminals down to windows 7 (not all terminals are tested see [Tested Terminals](#tested-terminals) for more info)

This crate is a sub-crate of [crossterm](https://crates.io/crates/crossterm) to move between screen buffers and switch to raw-mode, it can be use individually.

Other sub-crates are:
- [Crossterm Style](https://crates.io/crates/crossterm_style) 
- [Crossterm Terminal](https://crates.io/crates/crossterm_terminal) 
- [Crossterm Input](https://crates.io/crates/crossterm_input)
- [Crossterm Cursor](https://crates.io/crates/crossterm_cursor)
 
When you want to use other modules as well you might want to use crossterm with [feature flags](https://doc.rust-lang.org/1.30.0/book/first-edition/conditional-compilation.html)

When we want to print some text to the alternate screen we can't just write on it using print!(), println!(), or write!(). 
The same goes for coloring, cursor movement, input, and terminal actions.
However it is possible to do so, and [crossterm](https://crates.io/crates/crossterm) offers some help with that. 
  
In case you are wondering what 'alternate' or 'raw' screen is, you could checkout the [book](http://atcentra.com/crossterm/screen.html) describing this in more detail.
  
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

This documentation is only for `crossterm_screen` version `0.1` if you have an older version I suggest you check the [Upgrade Manual](https://github.com/TimonPost/crossterm/blob/master/docs/UpgradeManual.md).
Also, check out the [examples](https://github.com/TimonPost/crossterm/tree/master/examples) folders with detailed examples for all functionality of this crate
and the [book](http://atcentra.com/crossterm/screen.html) for more information about how to use the alternate or raw screen options.

Add the `crossterm_screen` package to your `Cargo.toml` file.

```
[dependencies]
`crossterm_screen` = "0.1"

```
And import the `crossterm_screen` modules you want to use.

```rust  
extern crate crossterm_screen;

pub use crossterm_screen::{AlternateScreen, RawScreen, Screen};
```

### Useful Links

- [Documentation](https://docs.rs/crossterm_screen/)
- [Crates.io](https://crates.io/crates/crossterm_screen)
- [Book](http://atcentra.com/crossterm/screen.html)
- [Examples](/examples)

## Features
These are the features of this crate:

- Cross-platform
- Everything is multithreaded (Send, Sync)
- Detailed documentation on every item
- Very few dependenties.
- Alternate screen
- Raw screen   
    
Planned features:
- make is possible to switch between multiple buffers.

## Examples
Check out the [examples](/examples/) for more information about how to use this crate.

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
