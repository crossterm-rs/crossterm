# Crossterm Style | cross-platform styling.
 ![Lines of Code][s7] [![Latest Version][s1]][l1] [![MIT][s2]][l2] [![docs][s3]][l3] [![Join us on Discord][s5]][l5]

[s1]: https://img.shields.io/crates/v/crossterm_style.svg
[l1]: https://crates.io/crates/crossterm_style

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: ./LICENSE

[s3]: https://docs.rs/crossterm_style/badge.svg
[l3]: https://docs.rs/crossterm_style/

[s5]: https://img.shields.io/discord/560857607196377088.svg?logo=discord
[l5]: https://discord.gg/K4nyTDB

[s7]: https://travis-ci.org/TimonPost/crossterm.svg?branch=master

This crate allows you to style te terminal cross-platform. 
It supports all UNIX and windows terminals down to windows 7 (not all terminals are tested see [Tested Terminals](#tested-terminals) for more info)

This crate is a sub-crate of [crossterm](https://crates.io/crates/crossterm) to style te terminal, and can be use individually.

Other sub-crates are:
- [Crossterm Input](https://crates.io/crates/crossterm_input) 
- [Crossterm Terminal](https://crates.io/crates/crossterm_terminal) 
- [Crossterm Screen](https://crates.io/crates/crossterm_screen)
- [Crossterm Cursor](https://crates.io/crates/crossterm_cursor)
 
When you want to use other modules as well you might want to use crossterm with [feature flags](https://timonpost.github.io/crossterm/docs/feature_flags.html).
 
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

This documentation is only for `crossterm_style` version `0.3` if you have an older version I suggest you check the [Upgrade Manual](https://github.com/TimonPost/crossterm/blob/master/docs/UPGRADE.md). Also, check out the [examples](https://github.com/TimonPost/crossterm/tree/master/crossterm_style/examples) folders with detailed examples for all functionality of this crate.

Add the `crossterm_style` package to your `Cargo.toml` file.

```
[dependencies]
crossterm_style = "0.3"
```

And import the `crossterm_style` modules you want to use.

```rust  
extern crate crossterm_style;

pub use crossterm_style::{color, style, Attribute, Color, ColorType, ObjectStyle, StyledObject, TerminalColor, Colorize, Styler};
```

### Useful Links

- [Documentation](https://docs.rs/crossterm_input/)
- [Crates.io](https://crates.io/crates/crossterm_input)
- [Book](https://timonpost.github.io/crossterm/docs/styling.html)
- [Examples](./examples)

## Features
These are the features of this crate:

- Cross-platform
- Multithreaded (send, sync)
- Detailed Documentation
- Few Dependencies
- Styled output
    - Foreground Color (16 base colors)
    - Background Color (16 base colors)
    - 256 (ANSI) Color Support (Windows 10 and UNIX Only)
    - RGB Color Support (Windows 10 and UNIX only)
    - Text Attributes: bold, italic, underscore and crossed word and [more](https://timonpost.github.io/crossterm/docs/styling.html#attributes) (Windows 10 and UNIX only)
    
## Examples
The [examples](./examples) folder has more complete and verbose examples.

_style text with attributes_
```rust
use crossterm_style::{Colored, Color, Colorize, Styler, Attribute};

// pass any `Attribute` value to the formatting braces.
println!("{} Underlined {} No Underline", Attribute::Underlined, Attribute::NoUnderline);

// you could also call different attribute methods on a `&str` and keep on chaining if needed.
let styled_text = "Bold Underlined".bold().underlined();
println!("{}", styled_text);

// old-way but still usable
let styled_text = style("Bold Underlined").bold().underlined();
```

_style text with colors_
```rust
use crossterm_style::{Colored, Color, Colorize};

println!("{} Red foreground color", Colored::Fg(Color::Red));
println!("{} Blue background color", Colored::Bg(Color::Blue));

// you can also call different coloring methods on a `&str`.
let styled_text = "Bold Underlined".red().on_blue();
println!("{}", styled_text);

// old-way but still usable
let styled_text = style("Bold Underlined").with(Color::Red).on(Color::Blue);
```
_style text with RGB and ANSI Value_
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
