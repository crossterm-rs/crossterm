use std::fmt::Display;
use Construct;
use super::base_cursor::ITerminalCursor;
use super::{AnsiCursor, NoCursor, WinApiCursor};

/// Struct with the cursor on wits cursor realated actions can be performed.
pub struct TerminalCursor {
    terminal_cursor: Option<Box<ITerminalCursor>>,
}

// impl Clone for TerminalCursor {
//     fn clone(&self) -> TerminalCursor { *self }
// }

impl TerminalCursor {
    /// Instantiate an cursor implementation whereon cursor related actions can be performed.
    pub fn init(&mut self) {
        if let None = self.terminal_cursor {
            self.terminal_cursor = get_cursor_options();
        }
    }

    /// Goto some location (x,y) in the terminal.
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate crossterm;
    ///
    /// use self::crossterm::terminal_cursor::{cursor,TerminalCursor};

    /// fn main()
    /// {
    ///     cursor::get().goto(10,10);
    /// }
    /// ```
    pub fn goto(&mut self, x: u16, y: u16) -> &mut TerminalCursor {
        &self.init();
        if let Some(ref terminal_cursor) = self.terminal_cursor {
            terminal_cursor.goto(x, y);
        }
        self
    }

    pub fn pos(mut self) -> (u16, u16) {
        &self.init();
        if let Some(ref terminal_cursor) = self.terminal_cursor {
            terminal_cursor.pos()
        } else {
            (0, 0)
        }
    }

    /// Print an value at the current cursor location.
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate crossterm;
    ///
    /// use self::crossterm::terminal_cursor::{cursor,TerminalCursor};

    /// fn main()
    /// {
    ///     // of course we can just do this.
    ///     print!("@").
    ///     // but now we can chain the methods so it looks cleaner.
    ///     cursor::get()
    ///     .goto(10,10)
    ///     .print("@");
    /// }
    /// ```
    pub fn move_up(&mut self, count: u16) -> &mut TerminalCursor {
        &self.init();
        if let Some(ref terminal_cursor) = self.terminal_cursor {
            terminal_cursor.move_up(count);
        }
        self
    }

    pub fn move_right(&mut self, count: u16) -> &mut TerminalCursor {
        &self.init();
        if let Some(ref terminal_cursor) = self.terminal_cursor {
            terminal_cursor.move_right(count);
        }
        self
    }

    pub fn move_down(&mut self, count: u16) -> &mut TerminalCursor {
        &self.init();
        if let Some(ref terminal_cursor) = self.terminal_cursor {
            terminal_cursor.move_down(count);
        }
        self
    }

    pub fn move_left(&mut self, count: u16) -> &mut TerminalCursor {
        &self.init();
        if let Some(ref terminal_cursor) = self.terminal_cursor {
            terminal_cursor.move_left(count);
        }
        self
    }

    pub fn print<D: Display>(&mut self, value: D) -> &mut TerminalCursor {
        print!("{}", value);
        use std;
        use std::io::Write;
        std::io::stdout().flush();
        self
    }
}

/// Get the concrete ITerminalCursor implementation based on the current operating system.
fn get_cursor_options() -> Option<Box<ITerminalCursor>> {
    if cfg!(target_os = "linux") {
        Some(AnsiCursor::new())
    } else if cfg!(target_os = "windows") {
        Some(WinApiCursor::new())
    } else {
        Some(NoCursor::new())
    }
}

/// Get terminal cursor options whereon cursor related actions can be performed.
pub fn get() -> Box<TerminalCursor> {
    Box::from(TerminalCursor {
        terminal_cursor: get_cursor_options(),
    })
}
