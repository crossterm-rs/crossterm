mod sys;
mod terminal;

pub use self::terminal::{terminal, Clear, ClearType, ScrollDown, ScrollUp, SetSize, Terminal};

pub use crossterm_utils::{execute, queue, Command, ExecutableCommand, QueueableCommand, Result};
