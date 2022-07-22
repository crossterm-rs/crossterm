#![deny(unused_imports, unused_must_use)]

//! # Crossterm
//!
//! Have you ever been disappointed when a terminal library for rust was only written for UNIX systems?
//! Crossterm provides clearing, event (input) handling, styling, cursor movement, and terminal actions for both
//! Windows and UNIX systems.
//!
//! Crossterm aims to be simple and easy to call in code. Through the simplicity of Crossterm, you do not
//! have to worry about the platform you are working with.
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
//!     [`DisableBlinking`](cursor/struct.DisableBlinking.html)
//!   - Position -
//!     [`SavePosition`](cursor/struct.SavePosition.html), [`RestorePosition`](cursor/struct.RestorePosition.html),
//!     [`MoveUp`](cursor/struct.MoveUp.html), [`MoveDown`](cursor/struct.MoveDown.html),
//!     [`MoveLeft`](cursor/struct.MoveLeft.html), [`MoveRight`](cursor/struct.MoveRight.html),
//!     [`MoveTo`](cursor/struct.MoveTo.html), [`MoveToColumn`](cursor/struct.MoveToColumn.html),[`MoveToRow`](cursor/struct.MoveToRow.html),
//!     [`MoveToNextLine`](cursor/struct.MoveToNextLine.html), [`MoveToPreviousLine`](cursor/struct.MoveToPreviousLine.html),
//!    - Shape -
//!      [`SetCursorShape`](cursor/struct.SetCursorShape.html)
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
//! - Module [`terminal`](terminal/index.html)
//!   - Scrolling - [`ScrollUp`](terminal/struct.ScrollUp.html),
//!     [`ScrollDown`](terminal/struct.ScrollDown.html)
//!   - Miscellaneous - [`Clear`](terminal/struct.Clear.html),
//!     [`SetSize`](terminal/struct.SetSize.html)
//!     [`SetTitle`](terminal/struct.SetTitle.html)
//!     [`DisableLineWrap`](terminal/struct.DisableLineWrap.html)
//!     [`EnableLineWrap`](terminal/struct.EnableLineWrap.html)
//!   - Alternate screen - [`EnterAlternateScreen`](terminal/struct.EnterAlternateScreen.html),
//!     [`LeaveAlternateScreen`](terminal/struct.LeaveAlternateScreen.html)
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
//! Flushing bytes to the terminal buffer is a heavy system call. If we perform a lot of actions with the terminal,
//! we want to do this periodically - like with a TUI editor - so that we can flush more data to the terminal buffer
//! at the same time.
//!
//! Crossterm offers the possibility to do this with `queue`.
//! With `queue` you can queue commands, and when you call [Write::flush][flush] these commands will be executed.
//!
//! You can pass a custom buffer implementing [std::io::Write][write] to this `queue` operation.
//! The commands will be executed on that buffer.
//! The most common buffer is [std::io::stdout][stdout] however, [std::io::stderr][stderr] is used sometimes as well.
//!
//! ##### Examples
//!
//! A simple demonstration that shows the command API in action with cursor commands.
//!
//! Functions:
//!
//! ```no_run
//! use std::io::{Write, stdout};
//! use crossterm::{QueueableCommand, cursor};
//!
//! let mut stdout = stdout();
//! stdout.queue(cursor::MoveTo(5,5));
//!
//! // some other code ...
//!
//! stdout.flush();
//! ```
//!
//! The [queue](./trait.QueueableCommand.html) function returns itself, therefore you can use this to queue another
//! command. Like `stdout.queue(Goto(5,5)).queue(Clear(ClearType::All))`.
//!
//! Macros:
//!
//! ```no_run
//! use std::io::{Write, stdout};
//! use crossterm::{queue, QueueableCommand, cursor};
//!
//! let mut stdout = stdout();
//! queue!(stdout,  cursor::MoveTo(5, 5));
//!
//! // some other code ...
//!
//! // move operation is performed only if we flush the buffer.
//! stdout.flush();
//! ```
//!
//! You can pass more than one command into the [queue](./macro.queue.html) macro like
//! `queue!(stdout, MoveTo(5, 5), Clear(ClearType::All))` and
//! they will be executed in the given order from left to right.
//!
//! #### Direct Execution
//!
//! For many applications it is not at all important to be efficient with 'flush' operations.
//! For this use case there is the `execute` operation.
//! This operation executes the command immediately, and calls the `flush` under water.
//!
//! You can pass a custom buffer implementing [std::io::Write][write] to this `execute` operation.
//! The commands will be executed on that buffer.
//! The most common buffer is [std::io::stdout][stdout] however, [std::io::stderr][stderr] is used sometimes as well.
//!
//! ##### Examples
//!
//! Functions:
//!
//! ```no_run
//! use std::io::{Write, stdout};
//! use crossterm::{ExecutableCommand, cursor};
//!
//! let mut stdout = stdout();
//! stdout.execute(cursor::MoveTo(5,5));
//! ```
//! The [execute](./trait.ExecutableCommand.html) function returns itself, therefore you can use this to queue
//! another command. Like `stdout.execute(Goto(5,5))?.execute(Clear(ClearType::All))`.
//!
//! Macros:
//!
//! ```no_run
//! use std::io::{Write, stdout};
//! use crossterm::{execute, ExecutableCommand, cursor};
//!
//! let mut stdout = stdout();
//! execute!(stdout, cursor::MoveTo(5, 5));
//! ```
//! You can pass more than one command into the [execute](./macro.execute.html) macro like
//! `execute!(stdout, MoveTo(5, 5), Clear(ClearType::All))` and they will be executed in the given order from
//! left to right.
//!
//! ## Examples
//!
//! Print a rectangle colored with magenta and use both direct execution and lazy execution.
//!
//! Functions:
//!
//! ```no_run
//! use std::io::{stdout, Write};
//! use crossterm::{
//!     ExecutableCommand, QueueableCommand,
//!     terminal, cursor, style::{self, Stylize}, Result
//! };
//!
//! fn main() -> Result<()> {
//!   let mut stdout = stdout();
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
//! use std::io::{stdout, Write};
//! use crossterm::{
//!     execute, queue,
//!     style::{self, Stylize}, cursor, terminal, Result
//! };
//!
//! fn main() -> Result<()> {
//!   let mut stdout = stdout();
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
//! [write]: https://doc.rust-lang.org/std/io/trait.Write.html
//! [stdout]: https://doc.rust-lang.org/std/io/fn.stdout.html
//! [stderr]: https://doc.rust-lang.org/std/io/fn.stderr.html
//! [flush]: https://doc.rust-lang.org/std/io/trait.Write.html#tymethod.flush

pub use crate::{
    command::{Command, ExecutableCommand, QueueableCommand},
    error::{ErrorKind, Result},
};

/// A module to work with the terminal cursor
pub mod cursor;
/// A module to read events.
pub mod event;
/// A module to apply attributes and colors on your text.
pub mod style;
/// A module to work with the terminal.
pub mod terminal;

/// A module to query if the current instance is a tty.
pub mod tty;

#[cfg(windows)]
/// A module that exposes one function to check if the current terminal supports ANSI sequences.
pub mod ansi_support;
mod command;
mod error;
pub(crate) mod macros;
