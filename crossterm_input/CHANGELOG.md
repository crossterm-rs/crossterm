# Changes crossterm_input 0.4.0
- `TerminalInput::read_line` returns `crossterm::Result` instead of `io::Result`
- `TerminalInput::read_char` returns `crossterm::Result` instead of `io::Result`  
- `Command::get_anis_code()` to `ansi_code()`
- Added KeyEvent::Enter and KeyEvent::Tab: [added-key-event-enter], [added-key-event-tab] 
- `ExecutableCommand::queue` returns `crossterm::Result`
- `QueueableCommand::queue` returns `crossterm::Result`
- Added derives: Serialize/Deserialize for key events [serde]
- Command API takes mutable self instead of self

[added-key-event-tab]: https://github.com/TimonPost/crossterm/pull/239
[added-key-event-enter]: https://github.com/TimonPost/crossterm/pull/236
[serde]: https://github.com/TimonPost/crossterm/pull/190

# Changes crossterm_input 0.3.3
- Removed println from `SyncReader`

# Changes crossterm_input 0.3.2
- Fixed some special key combination detections for UNIX systems
- Windows mouse input event position was 0-based and should be 1-based

# Changes crossterm_input 0.3.1
- Updated crossterm_utils 

# Changes crossterm_input 0.3
- Removed `TerminalInput::from_output()` 

# Changes crossterm_input 0.2.1
- Fixed SyncReade bug.

# Changes crossterm_input 0.2.1
- Introduced SyncReader

# Changes crossterm_input 0.2
- Introduced KeyEvents
- Introduced MouseEvents

# Changes crossterm_input 0.1 
- Moved out of `crossterm` 5.4 crate. 