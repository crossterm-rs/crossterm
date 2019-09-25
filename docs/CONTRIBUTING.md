# Contributing

I would appreciate any contributions to this crate. However, some things are handy to know.

## Architecture

Crossterm is using ANSI escape codes by default for both Unix and for Windows systems except
for Windows versions lower than 10. Crossterm uses WinAPI in this case.

### Crates

The `crossterm` crate consists of 7 crates:

* [cursor](https://github.com/crossterm-rs/crossterm-cursor)
* [input](https://github.com/crossterm-rs/crossterm-input)
* [style](https://github.com/crossterm-rs/crossterm-style)
* [terminal](https://github.com/crossterm-rs/crossterm-terminal)
* [screen](https://github.com/crossterm-rs/crossterm-screen)
* [utils](https://github.com/crossterm-rs/crossterm-utils)
* [winapi](https://github.com/crossterm-rs/crossterm-winapi)

### Module structure

If you would like to contribute, then please follow the existing structure. For
example, the cursor crate has the following file structure:

```text
└── src
    ├── cursor
    │   ├── ansi_cursor.rs
    │   ├── cursor.rs
    │   └── winapi_cursor.rs
    ├── cursor.rs
    ├── lib.rs
    ├── sys
    │   ├── unix.rs
    │   └── winapi.rs
    └── sys.rs
```

* `src/cursor.rs` - `ITerminalCursor` trait for other modules to implement
* `src/cursor/cursor.rs` - cursor functionality for the end user
* `src/cursor/winapi_cursor.rs` - WinAPI based implementation
* `src/cursor/ansi_cursor.rs` - ANSI escape codes based implementation
* `src/sys` - platform specific logic

The above structure is the same for other modules. 

Why I have chosen this design:

* You can easily add new platform by implementing the trait
* You can keep the functionality for different platforms separated
* You have one API the user can call like in the `src/cursor/cursor.rs`

Try to avoid changing `src/cursor/cursor.rs` a lot, because it contains API for
the end-user.

## Code style

### Import Order

All imports are semantically grouped and ordered. The order is:

- standard library (`use std::...`)
- external crates (`use rand::...`)
- current crate (`use crate::...`)
- parent module (`use super::..`)
- current module (`use self::...`)
- module declaration (`mod ...`)

There must be an empty line between groups.

An example:

```rust
use crossterm_utils::{csi, write_cout, Result};

use crate::sys::{get_cursor_position, show_cursor};

use super::ITerminalCursor;
```

### Warnings

The code must be warning free. It's quite hard to find an error if the build logs are polluted with warnings.
If you decide to silent a warning with (`#[allow(...)]`), please add a comment why it's required.

Always consult the [Travis CI](https://travis-ci.org/crossterm-rs/crossterm/pull_requests) build logs.
