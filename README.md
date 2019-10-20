<h1 align="center"><img width="440" src="docs/crossterm_full.png" /></h1>

[![Donate](https://img.shields.io/badge/Donate-PayPal-green.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_s-xclick&hosted_button_id=Z8QK6XU749JB2) ![Travis][s7] [![Latest Version][s1]][l1] [![MIT][s2]][l2] [![docs][s3]][l3] ![Lines of Code][s6] [![Join us on Discord][s5]][l5]

# Cross-platform Terminal Manipulation Library 

Have you ever been disappointed when a terminal library for the Rust language was only written for UNIX systems? 
Crossterm provides clearing, input handling, styling, cursor movement and terminal actions for both
Windows and UNIX systems.

Crossterm aims to be simple and easy to call in code. Through the simplicity of Crossterm, you do not have to
worry about the platform you are working with.

This crate supports all UNIX and Windows terminals down to Windows 7 (not all terminals are tested,
see [Tested Terminals](#tested-terminals) for more info).

## Table of Contents

* [Features](#features)
    * [Tested Terminals](#tested-terminals)
* [Getting Started](#getting-started)
    * [Feature Flags](#feature-flags)
    * [`crossterm` vs `crossterm_*` crates](#crossterm-vs-crossterm_-crates)
* [Other Resources](#other-resources)
* [Used By](#used-by)
* [Contributing](#contributing)    

## Features

- Cross-platform
- Multi-threaded (send, sync)
- Detailed documentation
- Few dependencies
- Cursor (feature `cursor`)
    - Move the cursor N times (up, down, left, right)
    - Set/get the cursor position
    - Store the cursor position and restore to it later
    - Hide/show the cursor
    - Enable/disable cursor blinking (not all terminals do support this feature)
- Styled output (feature `style`)
    - Foreground color (16 base colors)
    - Background color (16 base colors)
    - 256 (ANSI) color support (Windows 10 and UNIX only)
    - RGB color support (Windows 10 and UNIX only)
    - Text attributes like bold, italic, underscore, crossed, etc.
- Terminal (feature `terminal`)
    - Clear (all lines, current line, from cursor down and up, until new line)
    - Scroll up, down
    - Set/get the terminal size
    - Exit current process
- Input (feature `input`)
    - Read character
    - Read line
    - Read key input events (async / sync)
    - Read mouse input events (press, release, position, button)
- Screen (feature `screen`)
    - Alternate screen
    - Raw screen   

<!--
WARNING: Do not change following heading title as it's used in the URL by other crates!
-->

### Tested Terminals

- Windows Powershell
    - Windows 10 (Pro)
- Windows CMD
    - Windows 10 (Pro)
    - Windows 8.1 (N)
- Ubuntu Desktop Terminal
    - Ubuntu 17.10
- (Arch, Manjaro) KDE Konsole
- Linux Mint

This crate supports all UNIX terminals and Windows terminals down to Windows 7; however, not all of the
terminals have been tested. If you have used this library for a terminal other than the above list without
issues, then feel free to add it to the above list - I really would appreciate it!

## Getting Started

<details>
<summary>
Click to show Cargo.toml.
</summary>

```toml
[dependencies]
crossterm = "0.11"
```

</details>
<p></p>

```rust
use std::io::{stdout, Write};

use crossterm::{execute, Attribute, Color, Output, ResetColor, Result, SetBg, SetFg};

fn main() -> Result<()> {
    execute!(
        stdout(),
        // Blue foreground
        SetFg(Color::Blue),
        // Red background
        SetBg(Color::Red),
        Output("Styled text here.".to_string()),
        // Reset to default colors
        ResetColor
    )
}
```

### Feature Flags

All features are enabled by default. You can disable default features and enable some of them only.

```toml
[dependencies.crossterm]
version = "0.11"
default-features = false        # Disable default features
features = ["cursor", "screen"] # Enable required features only
```

| Feature | Description | Links |
| :-- | :-- | :-- |
| `input` | Sync/Async input readers | [API documentation](https://docs.rs/crossterm_input), [crates.io](https://crates.io/crates/crossterm_input), [GitHub](https://github.com/crossterm-rs/crossterm-input) |
| `cursor` | Cursor manipulation | [API documentation](https://docs.rs/crossterm_cursor), [crates.io](https://crates.io/crates/crossterm_cursor), [GitHub](https://github.com/crossterm-rs/crossterm-cursor) |
| `screen` | Alternate screen & raw mode | [API documentation](https://docs.rs/crossterm_screen), [crates.io](https://crates.io/crates/crossterm_screen), [GitHub](https://github.com/crossterm-rs/crossterm-screen) |
| `terminal` | Size, clear, scroll | [API documentation](https://docs.rs/crossterm_terminal), [crates.io](https://crates.io/crates/crossterm_terminal), [GitHub](https://github.com/crossterm-rs/crossterm-terminal) |
| `style` | Colors, text attributes | [API documentation](https://docs.rs/crossterm_style), [crates.io](https://crates.io/crates/crossterm_style), [GitHub](https://github.com/crossterm-rs/crossterm-style) |

### `crossterm` vs `crossterm_*` crates

There're two ways how to use the Crossterm library:

* `crossterm` crate with the `cursor` feature flag,
* `crossterm_cursor` crate.

Both provide same functionality, the only difference is the namespace (`crossterm` vs `crossterm_cursor`).

The first way (`crossterm` crate with feature flags) is preferred. The second way will be
deprecated and no longer supported soon. The `crossterm_*` crates will be marked as deprecated and
repositories archived on the GitHub. See the
[Merge sub-crates to the crossterm crate](https://github.com/crossterm-rs/crossterm/issues/265)
for more details.

#### `crossterm` crate:

```toml
[dependencies.crossterm]
version = "0.11"
default-features = false        # Disable default features
features = ["cursor"]           # Enable cursor feature only
```

```rust
use crossterm::cursor;
```

#### `crossterm_cursor` crate:

```toml
[dependencies]
crossterm_cursor = "0.3"
```

```rust
use crossterm_cursor::cursor;
```

### Other Resources

- [API documentation](https://docs.rs/crossterm/)
- [Examples repository](https://github.com/crossterm-rs/examples)

## Used By

- [Broot](https://dystroy.org/broot/)
- [Cursive](https://github.com/gyscos/Cursive)
- [TUI](https://github.com/fdehau/tui-rs)
- [Rust-sloth](https://github.com/jonathandturner/rust-sloth/tree/crossterm-port)

## Contributing
  
I highly appreciate when anyone contributes to this crate. Before you do, please,
read the [Contributing](docs/CONTRIBUTING.md) guidelines. 

## Authors

* **Timon Post** - *Project Owner & creator*

## Support

Would you like Crossterm to be even more gorgeous and beautiful? You can help with this by donating. 

[![paypal](https://www.paypalobjects.com/en_US/i/btn/btn_donateCC_LG.gif)](https://www.paypal.com/cgi-bin/webscr?cmd=_s-xclick&hosted_button_id=Z8QK6XU749JB2)

## License

This project, `crossterm` and all it's sub-crates: `crossterm_screen`, `crossterm_cursor`, `crossterm_style`,
`crossterm_input`, `crossterm_terminal`, `crossterm_winapi`, `crossterm_utils` are licensed under the MIT
License - see the [LICENSE](https://github.com/crossterm-rs/crossterm/blob/master/LICENSE) file for details.

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

[s6]: https://tokei.rs/b1/github/crossterm-rs/crossterm?category=code
[s7]: https://travis-ci.org/crossterm-rs/crossterm.svg?branch=master
