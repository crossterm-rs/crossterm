# Changes crossterm_style 0.3.0
- `Terminal::terminal_size` to `Terminal::size`
- `Terminal::size()` returns `Result<(u16, u16)>`
- Return written bytes: [return-written-bytes]
- Synced all `i16` values for indexing: set size, get size, scrolling to `u16` values
- Synced set/get terminal size behaviour: [fixed-get-set-terminal-size]
- `ExecutableCommand::queue` returns `crossterm::Result`
- `QueueableCommand::queue` returns `crossterm::Result`
- Command API takes mutable self instead of self

[return-written-bytes]: https://github.com/crossterm-rs/crossterm/pull/212
[fixed-get-set-terminal-size]: https://github.com/crossterm-rs/crossterm/pull/242

# Changes crossterm_terminal 0.2.2
- Terminal size Linux was not 0-based.
- Made FreeBSD compile

# Changes crossterm_terminal 0.2
- Removed `Terminal:from_output()` 

# Changes crossterm_terminal 0.1 
- Moved out of `crossterm` 5.4 crate. 