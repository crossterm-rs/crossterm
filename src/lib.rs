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
//! ## Important
//!
//! This crate re-exports all other `crossterm_*` crates types only. Please, consult the
//! `crossterm` crate repository [README](https://github.com/crossterm-rs/crossterm/blob/master/README.md) to
//! learn how to use features to enable/disable functionality, what's planned, etc. There will be
//! new code organization, breaking API changes, etc.

#[cfg(feature = "cursor")]
pub use crossterm_cursor::{
    cursor, BlinkOff, BlinkOn, Down, Goto, Hide, Left, ResetPos, Right, SavePos, Show,
    TerminalCursor, Up,
};
#[cfg(feature = "input")]
pub use crossterm_input::{
    input, AsyncReader, InputEvent, KeyEvent, MouseButton, MouseEvent, SyncReader, TerminalInput,
};
#[cfg(feature = "screen")]
pub use crossterm_screen::{AlternateScreen, IntoRawMode, RawScreen};
#[cfg(feature = "style")]
pub use crossterm_style::{
    color, style, Attribute, Color, Colored, Colorize, ObjectStyle, PrintStyledFont, ResetColor,
    SetAttr, SetBg, SetFg, StyledObject, Styler, TerminalColor,
};
#[cfg(feature = "terminal")]
pub use crossterm_terminal::{terminal, Clear, ClearType, ScrollDown, ScrollUp, SetSize, Terminal};
pub use crossterm_utils::{
    execute, queue, Command, ErrorKind, ExecutableCommand, Output, QueueableCommand, Result,
};

pub use self::crossterm::Crossterm;

mod crossterm;
