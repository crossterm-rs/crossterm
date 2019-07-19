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
pub use self::crossterm_cursor::{
    cursor, BlinkOff, BlinkOn, Down, Goto, Hide, Left, ResetPos, Right, SavePos, Show,
    TerminalCursor, Up,
};
#[cfg(feature = "input")]
pub use self::crossterm_input::{
    input, AsyncReader, InputEvent, KeyEvent, MouseButton, MouseEvent, SyncReader, TerminalInput,
};
#[cfg(feature = "screen")]
pub use self::crossterm_screen::{AlternateScreen, IntoRawMode, RawScreen};
#[cfg(feature = "style")]
pub use self::crossterm_style::{
    color, style, Attribute, Color, Colored, Colorize, ObjectStyle, PrintStyledFont, SetAttr,
    SetBg, SetFg, StyledObject, Styler, TerminalColor,
};
#[cfg(feature = "terminal")]
pub use self::crossterm_terminal::{
    terminal, Clear, ClearType, ScrollDown, ScrollUp, SetSize, Terminal,
};

pub use self::crossterm::Crossterm;
pub use self::crossterm_utils::{
    execute, queue, Command, ErrorKind, ExecutableCommand, Output, QueueableCommand, Result,
};
