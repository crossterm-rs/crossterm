extern crate crossterm_utils;

#[cfg(feature = "cursor")]
extern crate crossterm_cursor;
#[cfg(feature = "input")]
extern crate crossterm_input;
#[cfg(feature = "screen")]
extern crate crossterm_screen;
#[cfg(feature = "style")]
extern crate crossterm_style;
#[cfg(feature = "terminal")]
extern crate crossterm_terminal;

mod crossterm;

#[cfg(feature = "cursor")]
pub use self::crossterm_cursor::{cursor, TerminalCursor};
#[cfg(feature = "input")]
pub use self::crossterm_input::{
    input, AsyncReader, InputEvent, KeyEvent, MouseButton, MouseEvent, SyncReader, TerminalInput,
};
#[cfg(feature = "screen")]
pub use self::crossterm_screen::{AlternateScreen, IntoRawMode, RawScreen};
#[cfg(feature = "style")]
pub use self::crossterm_style::{
    color, style, Attribute, Color, Colored, Colorize, ObjectStyle, StyledObject, Styler,
    TerminalColor,
};
#[cfg(feature = "terminal")]
pub use self::crossterm_terminal::*;

pub use self::crossterm::Crossterm;
