#![deny(unused_imports, unused_must_use)]

//! # Crossterm
//!
//! Have you ever been disappointed when a terminal library for rust was only written for UNIX systems?
//! Crossterm provides clearing, input handling, styling, cursor movement, and terminal actions for both
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
//! command such as moving the cursor is executed. The command API offers:
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
//! Before `crossterm` 10.0 was released, `crossterm` had some performance issues. It did a `flush` after each command
//! (cursor movement). A `flush` is heavy action on the terminal, and if it is done more often the performance
//! will go down quickly.
//!
//! Linux and Windows 10 systems support ANSI escape codes. Those ANSI escape codes are strings or rather a
//! byte sequence. When we `write` and `flush` those to the terminal we can perform some action.
//!
//! ## Lazy Execution
//!
//! Because `flush` is a heavy system call we can instead `write` the commands to the `stdout` without flushing.
//! When can do a `flush` when we do want to execute the commands.
//!
//! If you create a terminal editor or TUI, it is wise to use this option. For example, you can `write` commands
//! to the terminal `stdout` and `flush` the `stdout` at every frame. By doing this you can make efficient use of the
//! terminal buffer and get better performance because you are not calling `flush` after every command.
//!
//! ### Examples
//!
//! Functions:
//!
//! ```no_run
//! use std::io::Write;
//! use crossterm::{QueueableCommand, cursor};
//!
//! let mut stdout = std::io::stdout();
//! stdout.queue(cursor::MoveTo(5,5));
//!
//! // some other code ...
//!
//! stdout.flush();
//! ```
//!
//! The `queue` function returns itself, therefore you can use this to queue another command. Like
//! `stdout.queue(Goto(5,5)).queue(Clear(ClearType::All))`.
//!
//! Macros:
//!
//! ```no_run
//! use std::io::Write;
//! use crossterm::{queue, QueueableCommand, cursor};
//!
//! let mut stdout = std::io::stdout();
//! queue!(stdout,  cursor::MoveTo(5, 5));
//!
//! // some other code ...
//!
//! stdout.flush();
//! ```
//!
//! You can pass more than one command into the macro like `queue!(stdout, Goto(5, 5), Clear(ClearType::All))` and
//! they will be executed in the given order from left to right.
//!
//! ## Direct Execution
//!
//! If you want to execute commands directly, this is also possible. You don't have to flush the 'stdout',
//! as described above. This is fine if you are not executing lots of commands.
//!
//! ### Examples
//!
//! Functions:
//!
//! ```no_run
//! use std::io::Write;
//! use crossterm::{ExecutableCommand, cursor};
//!
//! let mut stdout = std::io::stdout();
//! stdout.execute(cursor::MoveTo(5,5));
//! ```
//!
//! Macros:
//!
//! ```no_run
//! use std::io::Write;
//! use crossterm::{execute, ExecutableCommand, cursor};
//!
//! let mut stdout = std::io::stdout();
//! execute!(stdout, cursor::MoveTo(5, 5));
//! ```
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
//!     ExecutableCommand, QueueableCommand, Color,
//!     Colorize, terminal, cursor, style, Result
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
//!         stdout
//!           .queue(cursor::MoveTo(x,y))?
//!           .queue(style::PrintStyledFont( "█".magenta()))?;
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
//!     execute, queue, Color, PrintStyledFont,
//!     Colorize, cursor, terminal, style, Result
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
//!         queue!(stdout, cursor::MoveTo(x,y), style::PrintStyledFont( "█".magenta()))?;
//!       }
//!     }
//!   }
//!   stdout.flush()?;
//!   Ok(())
//! }
//!```

#[cfg(feature = "input")]
pub use input::{
    input, AsyncReader, InputEvent, KeyEvent, MouseButton, MouseEvent, SyncReader, TerminalInput,
};
#[cfg(feature = "screen")]
pub use screen::{
    AlternateScreen, EnterAlternateScreen, IntoRawMode, LeaveAlternateScreen, RawScreen,
};
#[cfg(feature = "style")]
pub use style::{
    color, style, Attribute, Color, Colored, Colorize, ContentStyle, PrintStyledFont, ResetColor,
    SetAttr, SetBg, SetFg, StyledContent, Styler, TerminalColor,
};
pub use utils::{Command, ErrorKind, ExecutableCommand, Output, QueueableCommand, Result};

pub use self::crossterm::Crossterm;

mod crossterm;

/// A functionality to work with the terminal cursor
#[cfg(feature = "cursor")]
pub mod cursor;
/// A functionality to read the input events.
#[cfg(feature = "input")]
pub mod input;
/// A functionality to work with the terminal screen.
#[cfg(feature = "screen")]
pub mod screen;
/// A functionality to apply attributes and colors on your text.
#[cfg(feature = "style")]
pub mod style;
/// A functionality to work with the terminal.
#[cfg(feature = "terminal")]
pub mod terminal;
/// Shared utilities.
pub mod utils;
