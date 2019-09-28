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
    │   ├── ansi.rs
    │   └── windows.rs
    ├── cursor.rs
    ├── lib.rs
    ├── sys
    │   ├── unix.rs
    │   └── windows.rs
    └── sys.rs
```

* `src/lib.rs` - public interface of the crate (for example `TerminalCursor` struct)
* `src/cursor.rs` - `Cursor` trait, which must be implement by all platform specific cursors
* `src/cursor/ansi.rs` - `AnsiCursor` structure implementing the `Cursor` trait
* `src/cursor/windows.rs` - `WinApiCursor` structure implementing the `Cursor` trait
* `src/sys` - platform specific logic

The above structure is followed by other crates. 

Why I have chosen this design:

* You can easily add new platform by implementing the trait
* You can keep the functionality for different platforms separated
* You have one API the user can call like in the `src/lib.rs`

Try to avoid changing `src/lib.rs` a lot, because it contains API for
the end-user.

## Code style

### Import order

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

use super::Cursor;
```

#### CLion tips

The CLion IDE does this for you (_Menu_ -> _Code_ -> _Optimize Imports_). Be aware that the CLion sorts
imports in a group in a different way when compared to the `rustfmt`. It's effectively two steps operation
to get proper grouping & sorting:

* _Menu_ -> _Code_ -> _Optimize Imports_ - group & semantically order imports
* `cargo fmt` - fix ordering within the group

Second step can be automated via _CLion_ -> _Preferences_ ->
_Languages & Frameworks_ -> _Rust_ -> _Rustfmt_ -> _Run rustfmt on save_.  

### Max line length

| Type | Max line length |
| :--- | ---: |
| Code | 100 |
| Comments in the code | 120 |
| Documentation | 120 |

100 is the [`max_width`](https://github.com/rust-lang/rustfmt/blob/master/Configurations.md#max_width)
default value.

120 is because of the GitHub. The editor & viewer width there is +- 123 characters. 

### 
### Warnings

The code must be warning free. It's quite hard to find an error if the build logs are polluted with warnings.
If you decide to silent a warning with (`#[allow(...)]`), please add a comment why it's required.

Always consult the [Travis CI](https://travis-ci.org/crossterm-rs/crossterm/pull_requests) build logs.

### Disallowed warnings

Search for `#![deny(...)]` in the code:

* `unused_must_use`
* `unused_imports`
