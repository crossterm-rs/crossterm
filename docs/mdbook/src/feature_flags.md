From `crossterm 0.6` you are allowed to use feature flags. 

With feature flags you can pick the features you want which reduces the size of the library and could prevent you from having unnecessary dependencies.

Crossterm provides the following feature flags:
- input     ; reading input
- terminal  ; terminal actions like resizing
- style     ; styling of the terminal
- cursor    ; moving the terminal cursor
- screen    ; alternate and raw screen

By default all of those will be enabled. 

_Cargo.toml_ 

```
[dependencies]
crossterm = { version="0.6", default-features = false, features = ["screen", "terminal", "cursor", "style", "input"] }
```

You can also use all the crossterm modules individually by directly referencing the crate.

- [Crossterm Style](https://crates.io/crates/crossterm_style) 
- [Crossterm Input](https://crates.io/crates/crossterm_input) 
- [Crossterm Screen](https://crates.io/crates/crossterm_screen)
- [Crossterm Cursor](https://crates.io/crates/crossterm_cursor)
- [Crossterm Terminal](https://crates.io/crates/crossterm_terminal)


