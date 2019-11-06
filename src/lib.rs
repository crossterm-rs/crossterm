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
//! see [Tested Terminals](https://github.com/crossterm-rs/crossterm/tree/zrzka/docs-update#tested-terminals)
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
//! * Macros are generally seen as more difficult but offer an API with less boilerplate code. If you are
//!   not afraid of macros, this is a recommendation.
//!
//! Linux and Windows 10 systems support ANSI escape codes. Those ANSI escape codes are strings or rather a
//! byte sequence. When we `write` and `flush` those to the terminal we can perform some action.
//! For older windows systems a WinApi call is made.
//!
//! ### Supported Commands
//!
//!| *Command Name*                  |  *Description*                                                     |
//!| :------------------------------ | :----------------------------                                      |
//!|   **crossterm::cursor module**   |                                                                   |
//!|  `cursor::DisableBlinking`	      | disables blinking of the terminal cursor.                         |
//!|  `cursor::EnableBlinking`	      | enables blinking of the terminal cursor.                          |
//!|  `cursor::Hide`	              | hides the terminal cursor.                                        |
//!|  `cursor::MoveDown`	          | moves the terminal cursor a given number of rows down.            |
//!|  `cursor::MoveLeft`	          | moves the terminal cursor a given number of columns to the left.  |
//!|  `cursor::MoveRight`	          | moves the terminal cursor a given number of columns to the right. |
//!|  `cursor::MoveTo`	              | moves the terminal cursor to the given position (column, row).    |
//!|  `cursor::MoveUp`	              | moves the terminal cursor a given number of rows up.              |
//!|  `cursor::RestorePosition`	      | restores the saved terminal cursor position.                      |
//!|  `cursor::SavePosition`	      | saves the current terminal cursor position.                       |
//!|  `cursor::Show`	              | shows the terminal cursor.                                        |
//!|   **crossterm::event module**    |                                                                   |
//!|  `event::DisableMouseCapture`    | disables mouse event monitoring.                                  |
//!|  `event::EnableMouseCapture`	  | enables mouse mode                                                |
//!|                                  |                                                                   |
//!|  `screen::EnterAlternateScreen`  | switches to the alternate screen.                                 |
//!|  `screen::LeaveAlternateScreen`  |	switches back to the main screen.                                 |
//!|   **crossterm::style module**    |                                                                   |
//!|  `style::PrintStyledContent`     | prints styled content.                                            |
//!|  `style::ResetColor`	          | resets the colors back to default.                                |
//!|  `style::SetAttribute`	          | sets an attribute.                                                |
//!|  `style::SetBackgroundColor`     | sets the the background color.                                    |
//!|  `style::SetForegroundColor`     | sets the the foreground color.                                    |
//!|   **crossterm::terminal module** |                                                                   |
//!|  `terminal::Clear`	              | clears the terminal screen buffer.                                |
//!|  `terminal::ScrollDown`	      | scrolls the terminal screen a given number of rows down.          |
//!|  `terminal::ScrollUp`	          | scrolls the terminal screen a given number of rows up.            |
//!|  `terminal::SetSize`             | sets the terminal size (columns, rows).                           |
//!
//! There are two different way's to execute commands.
//! * [Lazy Execution](#lazy-execution)
//! * [Direct Execution](#direct-execution)
//!
//! ## Lazy Execution
//!
//! Flushing bytes to the terminal buffer is a heavy system call. If we perform a lot of actions with the terminal,
//! we want to do this periodically - like with a TUI editor - so that we can flush more data to the terminal buffer at the same time.
//!
//! Crossterm offers the possibility to do this with `queue`.
//! With `queue` you can queue commands, and when you call [Write::flush][flush] these commands will be executed.
//!
//! You can pass a custom buffer implementing [std::io::Write][write] to this `queue` operation.
//! The commands will be executed on that buffer.
//! The most common buffer is [std::io::stdout][stdout] however, [std::io::stderr][stderr] is used sometimes as well.
//!
//! ### Examples
//! A simple demonstration that shows the command API in action with cursor commands.
//!
//! **Functions**
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
//! The [queue](./trait.QueueableCommand.html) function returns itself, therefore you can use this to queue another command. Like
//! `stdout.queue(Goto(5,5)).queue(Clear(ClearType::All))`.
//!
//! **Macros**
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
//! You can pass more than one command into the [queue](./macro.queue.html) macro like `queue!(stdout, MoveTo(5, 5), Clear(ClearType::All))` and
//! they will be executed in the given order from left to right.
//!
//! ## Direct Execution
//!
//! For many applications it is not at all important to be efficient with 'flush' operations.
//! For this use case there is the `execute` operation.
//! This operation executes the command immediately, and calls the `flush` under water.
//!
//! You can pass a custom buffer implementing [std::io::Write][write] to this `execute` operation.
//! The commands will be executed on that buffer.
//! The most common buffer is [std::io::stdout][stdout] however, [std::io::stderr][stderr] is used sometimes as well.
//!
//! ### Examples
//!
//! **Functions**
//!
//! ```no_run
//! use std::io::{Write, stdout};
//! use crossterm::{ExecutableCommand, cursor};
//!
//! let mut stdout = stdout();
//! stdout.execute(cursor::MoveTo(5,5));
//! ```
//! The [execute](./trait.ExecutableCommand.html) function returns itself, therefore you can use this to queue another command. Like
//! `stdout.queue(Goto(5,5)).queue(Clear(ClearType::All))`.
//!
//! **Macros**
//!
//! ```no_run
//! use std::io::{Write, stdout};
//! use crossterm::{execute, ExecutableCommand, cursor};
//!
//! let mut stdout = stdout();
//! execute!(stdout, cursor::MoveTo(5, 5));
//! ```
//! You can pass more than one command into the [execute](./macro.execute.html) macro like `execute!(stdout, MoveTo(5, 5), Clear(ClearType::All))` and
//! they will be executed in the given order from left to right.
//!
//! ## Examples
//!
//! Print a rectangle colored with magenta and use both direct execution and lazy execution.
//!
//! **Functions**
//!
//! ```no_run
//! use std::io::{stdout, Write};
//! use crossterm::{
//!     ExecutableCommand, QueueableCommand,
//!     terminal, cursor, style::{self, Colorize}, Result
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
//! **Macros:**
//!
//! ```no_run
//! use std::io::{stdout, Write};
//! use crossterm::{
//!     execute, queue,
//!     style::{self, Colorize}, cursor, terminal, Result
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

#[cfg(windows)]
pub use utils::functions::supports_ansi;
pub use utils::{Command, ErrorKind, ExecutableCommand, Output, QueueableCommand, Result};

/// A module to work with the terminal cursor
#[cfg(feature = "cursor")]
pub mod cursor;
/// A module to read events.
#[cfg(feature = "event")]
pub mod event;
/// A module to work with the terminal screen.
#[cfg(feature = "screen")]
pub mod screen;
/// A module to apply attributes and colors on your text.
#[cfg(feature = "style")]
pub mod style;
/// A module to work with the terminal.
#[cfg(feature = "terminal")]
pub mod terminal;
/// Shared utilities.
pub mod utils;
