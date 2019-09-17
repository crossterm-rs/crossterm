This folder contains examples for crossterm and it's the sub-crates.

When using a sub-crate instead of the crossterm crate, make sure to change the namespaces in the examples from `crossterm` to `crossterm_{crate_name}`.

Examples, on the different functionalities
- [crossterm style](https://crates.io/crates/crossterm_style) 
    - [color](https://github.com/TimonPost/crossterm/blob/master/examples/cursor.rs): this is about the styling of the terminal
- [crossterm input](https://crates.io/crates/crossterm_input) 
    - [input](https://github.com/TimonPost/crossterm/blob/master/examples/input.rs): this is about input reading
    - [key_events](https://github.com/TimonPost/crossterm/blob/master/examples/key_events.rs): this is about reading key events
- [crossterm screen](https://crates.io/crates/crossterm_screen)
    - [alternate_screen](https://github.com/TimonPost/crossterm/blob/master/examples/alternate_screen.rs): this is about switching to an alternate screen buffer
    - [raw_screen](https://github.com/TimonPost/crossterm/blob/master/examples/raw_screen.rs): this is about enabling raw screen
- [crossterm cursor](https://crates.io/crates/crossterm_cursor)
    - [cursor](https://github.com/TimonPost/crossterm/blob/master/examples/cursor.rs): this is about the actions you can perform with the cursor
- [crossterm terminal](https://crates.io/crates/crossterm_terminal)
    - [terminal](https://github.com/TimonPost/crossterm/blob/master/examples/terminal.rs): this is about the actions you can perform on the terminal

Other
- [crossterm](https://github.com/TimonPost/crossterm/blob/master/examples/crossterm.rs): this is about the struct `Crossterm`
- [command](https://github.com/TimonPost/crossterm/blob/master/examples/command.rs): this is about to the command api
- [program examples](https://github.com/TimonPost/crossterm/tree/master/examples/program_examples): this folder will contain some real life examples
- [command_bar](https://github.com/TimonPost/crossterm/tree/master/examples/command_bar): this is a terminal application where multiple threads write to the output while you can enter
  commands asynchronously.
