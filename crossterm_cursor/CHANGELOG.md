# Changes crossterm_cursor 0.3
- `TerminalCursor::pos()` returns `crossterm::Result<(u16, u16)>`
- `TerminalCursor::move_*` returns `crossterm::Result`
- `TerminalCursor::reset_position()` to `restore_position()`
- All `i16` values for indexing: set/get cursor pos synced to `u16` values
- `Command::get_anis_code()` to `ansi_code()`
- `ExecutableCommand::queue` returns `crossterm::Result`
- `QueueableCommand::queue` returns `crossterm::Result`
- Command API takes mutable self instead of self

# Changes crossterm_cursor 0.2
- Removed `TerminalCursor::from_output()` 

# Changes crossterm_cursor 0.1 
- Moved out of `crossterm` 5.4 crate. 