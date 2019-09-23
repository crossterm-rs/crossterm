# Changes crossterm_style 0.4
- `get_available_color_count` returns no result
- `ExecutableCommand::queue` returns `crossterm::Result`
- `QueueableCommand::queue` returns `crossterm::Result`
- `available_color_count` to `available_color_count()`
- Added derives: `Debug` for `ObjectStyle`  [debug-derive]
- Command API takes mutable self instead of self

# Changes crossterm_style 0.3
- Removed `TerminalColor::from_output()` 
- Added `NoItalic` attribute

# Changes crossterm_style 0.2
- Introduced more `Attributes`
- Introduced easier ways to style text [issue 87](https://github.com/TimonPost/crossterm/issues/87).
- Removed `ColorType` since it was unnecessary.

# Changes crossterm_style 0.1 
- Moved out of `crossterm` 5.4 crate. 