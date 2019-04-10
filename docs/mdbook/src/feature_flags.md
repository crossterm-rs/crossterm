From `crossterm 0.6` you are able to use feature flags. 

With feature flags you can pick the features you want which reduces the size of the library and could prevent you from having unnecessary dependencies.

Crossterm provides the following feature flags:
- input     ; reading input events
- terminal  ; terminal actions like resizing
- style     ; styling of the terminal
- cursor    ; moving the terminal cursor
- screen    ; alternate and raw screen

By default all of those will be enabled. 

_Cargo.toml_ 

```
[dependencies]
crossterm = { version="0.9", default-features = false, features = ["screen", "terminal", "cursor", "style", "input"] }
```

By default all flags are enabled, the types and functions available to use depend on the specified flags.

```rust
"cursor" => cursor, TerminalCursor
"input" => input, AsyncReader, InputEvent, KeyEvent, MouseButton, MouseEvent, SyncReader, TerminalInput
"screen" => AlternateScreen, IntoRawMode, RawScreen
"style" => color, style, Attribute, Color, Colored, Colorize, ObjectStyle, StyledObject, Styler, TerminalColor,
"terminal" => terminal, ClearType, Terminal
```

You can also use all the crossterm modules individually by directly referencing the crate.

- [Crossterm Style](https://crates.io/crates/crossterm_style) 
- [Crossterm Input](https://crates.io/crates/crossterm_input) 
- [Crossterm Screen](https://crates.io/crates/crossterm_screen)
- [Crossterm Cursor](https://crates.io/crates/crossterm_cursor)
- [Crossterm Terminal](https://crates.io/crates/crossterm_terminal)


