#![deny(unused_imports, unused_must_use)]

//! # Cross-platform Terminal Manipulation Library
//!
//! Crossterm is a pure-rust, terminal manipulation library that makes it possible to write cross-platform text-based interfaces.
//!
//! This crate supports all UNIX and Windows terminals down to Windows 7 (not all terminals are tested
//! see [Tested Terminals](https://github.com/crossterm-rs/crossterm#tested-terminals)
//! for more info).
//!
//! ## Command API
//!
//! The command API makes the use of `crossterm` much easier and offers more control over when and how a
//! command is executed. A command is just an action you can perform on the terminal e.g. cursor movement.
//!
//! The command API offers:
//!
//! * Better Performance.
//! * Complete control over when to flush.
//! * Complete control over where the ANSI escape commands are executed to.
//! * Way easier and nicer API.
//!
//! There are two ways to use the API command:
//!
//! * Functions can execute commands on types that implement Write. Functions are easier to use and debug.
//!   There is a disadvantage, and that is that there is a boilerplate code involved.
//! * Macros are generally seen as more difficult and aren't always well supported by editors but offer an API with less boilerplate code. If you are
//!   not afraid of macros, this is a recommendation.
//!
//! Linux and Windows 10 systems support ANSI escape codes. Those ANSI escape codes are strings or rather a
//! byte sequence. When we `write` and `flush` those to the terminal we can perform some action.
//! For older windows systems a WinAPI call is made.
//!
//! ### Supported Commands
//!
//! - Module [`cursor`](cursor/index.html)
//!   - Visibility - [`Show`](cursor/struct.Show.html), [`Hide`](cursor/struct.Hide.html)
//!   - Appearance - [`EnableBlinking`](cursor/struct.EnableBlinking.html),
//!     [`DisableBlinking`](cursor/struct.DisableBlinking.html),
//!     [`SetCursorStyle`](cursor/enum.SetCursorStyle.html)
//!   - Position -
//!     [`SavePosition`](cursor/struct.SavePosition.html), [`RestorePosition`](cursor/struct.RestorePosition.html),
//!     [`MoveUp`](cursor/struct.MoveUp.html), [`MoveDown`](cursor/struct.MoveDown.html),
//!     [`MoveLeft`](cursor/struct.MoveLeft.html), [`MoveRight`](cursor/struct.MoveRight.html),
//!     [`MoveTo`](cursor/struct.MoveTo.html), [`MoveToColumn`](cursor/struct.MoveToColumn.html),[`MoveToRow`](cursor/struct.MoveToRow.html),
//!     [`MoveToNextLine`](cursor/struct.MoveToNextLine.html), [`MoveToPreviousLine`](cursor/struct.MoveToPreviousLine.html)
//! - Module [`event`](event/index.html)
//!   - Keyboard events -
//!     [`PushKeyboardEnhancementFlags`](event/struct.PushKeyboardEnhancementFlags.html),
//!     [`PopKeyboardEnhancementFlags`](event/struct.PopKeyboardEnhancementFlags.html)
//!   - Mouse events - [`EnableMouseCapture`](event/struct.EnableMouseCapture.html),
//!     [`DisableMouseCapture`](event/struct.DisableMouseCapture.html)
//! - Module [`style`](style/index.html)
//!   - Colors - [`SetForegroundColor`](style/struct.SetForegroundColor.html),
//!     [`SetBackgroundColor`](style/struct.SetBackgroundColor.html),
//!     [`ResetColor`](style/struct.ResetColor.html), [`SetColors`](style/struct.SetColors.html)
//!   - Attributes - [`SetAttribute`](style/struct.SetAttribute.html), [`SetAttributes`](style/struct.SetAttributes.html),
//!     [`PrintStyledContent`](style/struct.PrintStyledContent.html)
//!   - Hyperlinks - [`StartHyperlink`](style/struct.StartHyperlink.html),
//!     [`EndHyperlink`](style/struct.EndHyperlink.html)
//! - Module [`terminal`](terminal/index.html)
//!   - Scrolling - [`ScrollUp`](terminal/struct.ScrollUp.html),
//!     [`ScrollDown`](terminal/struct.ScrollDown.html)
//!   - Miscellaneous - [`Clear`](terminal/struct.Clear.html),
//!     [`SetSize`](terminal/struct.SetSize.html),
//!     [`SetTitle`](terminal/struct.SetTitle.html),
//!     [`DisableLineWrap`](terminal/struct.DisableLineWrap.html),
//!     [`EnableLineWrap`](terminal/struct.EnableLineWrap.html)
//!   - Alternate screen - [`EnterAlternateScreen`](terminal/struct.EnterAlternateScreen.html),
//!     [`LeaveAlternateScreen`](terminal/struct.LeaveAlternateScreen.html),
//!     [`EnableAlternateScrollMode`](terminal/struct.EnableAlternateScrollMode.html),
//!     [`DisableAlternateScrollMode`](terminal/struct.DisableAlternateScrollMode.html)
//! - Module [`clipboard`](clipboard/index.html) (requires
//!   [`feature = "osc52"`](#optional-features))
//!   - Clipboard - [`CopyToClipboard`](clipboard/struct.CopyToClipboard.html)
//!
//! ### Command Execution
//!
//! There are two different ways to execute commands:
//!
//! * [Lazy Execution](#lazy-execution)
//! * [Direct Execution](#direct-execution)
//!
//! #### Lazy Execution
//!
//! Flushing output after every command can be costly when an application updates the terminal
//! frequently, such as a TUI editor. Use `queue` to batch commands, then call
//! [`Write::flush`][flush] when you are ready to send the output.
//!
//! You can use any writer implementing [`std::io::Write`][write], including
//! [`std::io::stdout`][stdout], [`std::io::stderr`][stderr], or a custom buffer.
//!
//! ##### Methods
//!
//! Queue a cursor movement, then flush the output:
//!
//! ```no_run
//! use std::io::{stdout, Write};
//! use crossterm::{
//!     cursor::MoveTo,
//!     terminal::{Clear, ClearType},
//!     QueueableCommand,
//! };
//!
//! # fn main() -> std::io::Result<()> {
//! let mut stdout = stdout();
//! stdout.queue(MoveTo(5, 5))?;
//!
//! // Queue more commands here before flushing.
//!
//! stdout.flush()?;
//! # Ok(())
//! # }
//! ```
//!
//! The [`queue`](QueueableCommand::queue) method returns `io::Result<&mut Self>`, so you can use
//! `?` to propagate errors and chain another command on the same writer:
//!
//! ```no_run
//! # use std::io::stdout;
//! # use crossterm::{
//! #     cursor::MoveTo,
//! #     terminal::{Clear, ClearType},
//! #     QueueableCommand,
//! # };
//! # fn main() -> std::io::Result<()> {
//! # let mut stdout = stdout();
//! stdout
//!     .queue(MoveTo(5, 5))?
//!     .queue(Clear(ClearType::All))?;
//! # Ok(())
//! # }
//! ```
//!
//! ##### Macros
//!
//! The [`queue!`] macro accepts multiple commands and queues them in the order provided:
//!
//! ```no_run
//! # use std::io::{stdout, Write};
//! # use crossterm::{cursor::MoveTo, terminal::{Clear, ClearType}};
//! use crossterm::queue;
//!
//! # fn main() -> std::io::Result<()> {
//! let mut stdout = stdout();
//! queue!(stdout, MoveTo(5, 5), Clear(ClearType::All))?;
//!
//! // Queue more commands here before flushing.
//!
//! // Flush the queued output.
//! stdout.flush()?;
//! # Ok(())
//! # }
//! ```
//!
//! #### Direct Execution
//!
//! For applications that send only a few commands at a time, the cost of flushing after each
//! command is often negligible, so batching commands may offer little performance benefit.
//! Use `execute` when you want to send a command immediately rather than batch commands.
//! It writes the command to the output and calls [`Write::flush`][flush].
//!
//! You can use any writer implementing [`std::io::Write`][write], including
//! [`std::io::stdout`][stdout], [`std::io::stderr`][stderr], or a custom buffer.
//!
//! ##### Methods
//!
//! Execute a cursor movement and flush the output immediately:
//!
//! ```no_run
//! use std::io::stdout;
//! use crossterm::{
//!     cursor::MoveTo,
//!     terminal::{Clear, ClearType},
//!     ExecutableCommand,
//! };
//!
//! # fn main() -> std::io::Result<()> {
//! let mut stdout = stdout();
//! stdout.execute(MoveTo(5, 5))?;
//! # Ok(())
//! # }
//! ```
//!
//! The [`execute`](ExecutableCommand::execute) method returns `io::Result<&mut Self>`, so you can use
//! `?` to propagate errors and chain another command on the same writer:
//!
//! ```no_run
//! # use std::io::stdout;
//! # use crossterm::{
//! #     cursor::MoveTo,
//! #     terminal::{Clear, ClearType},
//! #     ExecutableCommand,
//! # };
//! # fn main() -> std::io::Result<()> {
//! # let mut stdout = stdout();
//! stdout
//!     .execute(MoveTo(5, 5))?
//!     .execute(Clear(ClearType::All))?;
//! # Ok(())
//! # }
//! ```
//!
//! ##### Macros
//!
//! The [`execute!`] macro accepts multiple commands, writes them in the order provided,
//! and flushes the output:
//!
//! ```no_run
//! # use std::io::stdout;
//! # use crossterm::{cursor::MoveTo, terminal::{Clear, ClearType}};
//! use crossterm::execute;
//!
//! # fn main() -> std::io::Result<()> {
//! let mut stdout = stdout();
//! execute!(stdout, MoveTo(5, 5), Clear(ClearType::All))?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Examples
//!
//! Print a rectangle colored with magenta and use both direct execution and lazy execution.
//!
//! Functions:
//!
//! ```no_run
//! use std::io::{self, Write};
//! use crossterm::{
//!     ExecutableCommand, QueueableCommand,
//!     terminal, cursor, style::{self, Stylize}
//! };
//!
//! fn main() -> io::Result<()> {
//!   let mut stdout = io::stdout();
//!
//!   stdout.execute(terminal::Clear(terminal::ClearType::All))?;
//!
//!   for y in 0..40 {
//!     for x in 0..150 {
//!       if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
//!         // in this loop we are more efficient by not flushing the buffer.
//!         stdout
//!           .queue(cursor::MoveTo(x,y))?
//!           .queue(style::PrintStyledContent( "█".magenta()))?;
//!       }
//!     }
//!   }
//!   stdout.flush()?;
//!   Ok(())
//! }
//! ```
//!
//! Macros:
//!
//! ```no_run
//! use std::io::{self, Write};
//! use crossterm::{
//!     execute, queue,
//!     style::{self, Stylize}, cursor, terminal
//! };
//!
//! fn main() -> io::Result<()> {
//!   let mut stdout = io::stdout();
//!
//!   execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
//!
//!   for y in 0..40 {
//!     for x in 0..150 {
//!       if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
//!         // in this loop we are more efficient by not flushing the buffer.
//!         queue!(stdout, cursor::MoveTo(x,y), style::PrintStyledContent( "█".magenta()))?;
//!       }
//!     }
//!   }
//!   stdout.flush()?;
//!   Ok(())
//! }
//!```
//!
#![cfg_attr(feature = "document-features", doc = "## Feature Flags")]
#![cfg_attr(feature = "document-features", doc = document_features::document_features!())]
//!
//! [write]: https://doc.rust-lang.org/std/io/trait.Write.html
//! [stdout]: https://doc.rust-lang.org/std/io/fn.stdout.html
//! [stderr]: https://doc.rust-lang.org/std/io/fn.stderr.html
//! [flush]: https://doc.rust-lang.org/std/io/trait.Write.html#tymethod.flush

pub use crate::command::{Command, ExecutableCommand, QueueableCommand, SynchronizedUpdate};

/// A module to work with the terminal cursor
pub mod cursor;
/// A module to read events.
#[cfg(feature = "events")]
pub mod event;
/// A module to apply attributes and colors on your text.
pub mod style;
/// A module to work with the terminal.
pub mod terminal;

/// A module for clipboard interaction
#[cfg(feature = "osc52")]
pub mod clipboard;

#[cfg(windows)]
/// A module that exposes one function to check if the current terminal supports ANSI sequences.
pub mod ansi_support;
mod command;
pub(crate) mod macros;
