use std::fmt::Display;
use Construct;
use super::base_cursor::ITerminalCursor;
use super::{AnsiCursor, NoCursor, WinApiCursor};

/// Struct on wits cursor realated actions can be performed.
pub struct TerminalCursor {
    terminal_cursor: Option<Box<ITerminalCursor>>,
}

impl TerminalCursor {
    /// Instantiates an platform specific cursor implementation whereon cursor related actions can be performed.
    pub fn init(&mut self) {
        if let None = self.terminal_cursor {
            self.terminal_cursor = get_cursor_options();
        }
    }

    /// Goto some position (x,y) in the terminal.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    ///     cursor::get().goto(10,10);
    /// 
    /// ```
    pub fn goto(&mut self, x: u16, y: u16) -> &mut TerminalCursor {
        &self.init();
        if let Some(ref terminal_cursor) = self.terminal_cursor {
            terminal_cursor.goto(x, y);
        }
        self
    }

    /// Get current cursor position (x,y) in the terminal.
    ///
    /// #Example
    ///
    /// ```rust
    /// 
    ///     let pos = cursor::get().pos();
    ///     println!("{:?}", pos);
    /// 
    /// ```
    pub fn pos(&mut self) -> (i16, i16) {
        &self.init();
        if let Some(ref terminal_cursor) = self.terminal_cursor {
            terminal_cursor.pos()
        } else {
            (0, 0)
        }
    }

    /// Move the current cursor position `n` times up.
    ///
    /// #Example
    ///
    /// ```rust
    /// 
    ///     // move 1 time up
    ///     cursor::get().move_up(1);
    /// 
    ///     // move 2 time up
    ///     cursor::get().move_up(2);    /// 
    /// ```
    pub fn move_up(&mut self, count: u16) -> &mut TerminalCursor {
        &self.init();
        if let Some(ref terminal_cursor) = self.terminal_cursor {
            terminal_cursor.move_up(count);
        }
        self
    }

    /// Move the current cursor position `n` times right.
    ///
    /// #Example
    ///
    /// ```rust
    /// 
    ///     // move 1 time right
    ///     cursor::get().move_right(1);
    /// 
    ///     // move 2 time right
    ///     cursor::get().move_right(2);
    /// 
    /// ```
    pub fn move_right(&mut self, count: u16) -> &mut TerminalCursor {
        &self.init();
        if let Some(ref terminal_cursor) = self.terminal_cursor {
            terminal_cursor.move_right(count);
        }
        self
    }

    /// Move the current cursor position `n` times down.
    ///
    /// #Example
    ///
    /// ```rust
    /// 
    ///     // move 1 time down 
    ///     cursor::get().move_down(1);
    /// 
    ///     // move 2 time down
    ///     cursor::get().move_down(2);
    ///
    /// ```
    pub fn move_down(&mut self, count: u16) -> &mut TerminalCursor {
        &self.init();
        if let Some(ref terminal_cursor) = self.terminal_cursor {
            terminal_cursor.move_down(count);
        }
        self
    }

    /// Move the current cursor position `n` times left.
    ///
    /// #Example
    ///
    /// ```rust
    /// 
    ///     // move 1 time left
    ///     cursor::get().move_left(1);
    /// 
    ///     // move 2 time left
    ///     cursor::get().move_left(2);
    /// 
    /// ```
    pub fn move_left(&mut self, count: u16) -> &mut TerminalCursor {
        &self.init();
        if let Some(ref terminal_cursor) = self.terminal_cursor {
            terminal_cursor.move_left(count);
        }
        self
    }

    /// Print an value at the current cursor position.
    ///
    /// This method prints an value and clears the buffer. 
    /// If you do not clear the buffer the character  will not be printed at the wished position.
    /// #Example
    ///
    /// ```rust
    /// 
    ///     // of course we can just do this.
    ///     cursor::get().goto(10,10);
    ///     print!("@");
    ///     std::io::stdout().flush();
    /// 
    ///     // but now we can chain the methods so it looks cleaner and it automatically flushes the buffer.  
    ///     cursor::get()
    ///     .goto(10,10)
    ///     .print("@");
    /// 
    /// ```
    pub fn print<D: Display>(&mut self, value: D) -> &mut TerminalCursor {
        print!("{}", value);
        use std;
        use std::io::Write;
        // rust is line buffered so we need to flush the buffer in order to print it at the current cursor position. 
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
