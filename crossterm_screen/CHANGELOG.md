# Changes crossterm_screen 0.3.0

- `RawScreen::into_raw_mode` returns `crossterm::Result` instead of `io::Result`
- `RawScreen::disable_raw_mode` returns `crossterm::Result` instead of `io::Result`
- `AlternateScreen::to_alternate` returns `crossterm::Result` instead of `io::Result`
- `AsyncReader::stop_reading()` to `stop()`
- `RawScreen::disable_raw_mode_on_drop` to `keep_raw_mode_on_drop`