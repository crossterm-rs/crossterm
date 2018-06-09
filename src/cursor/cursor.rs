//! With this module you can perform actions that are cursor related.
//! Like changing and displaying the position of the cursor in terminal.
//!
//! Note that positions of the cursor are 0 -based witch means that the coordinates starts counting from 0

use super::*;
use Terminal;

use std::fmt::Display;
use std::io::Write;

/// Struct that stores an specific platform implementation for cursor related actions.
pub struct TerminalCursor<'term> {
    terminal: &'term Terminal,
    terminal_cursor: Option<Box<ITerminalCursor>>,
}

impl <'term> TerminalCursor<'term>
{
    /// Create new cursor instance whereon cursor related actions can be performed.
    pub fn new(terminal: &'term Terminal) -> TerminalCursor<'term> {
        #[cfg(target_os = "windows")]
        let cursor = functions::get_module::<Box<ITerminalCursor>>(WinApiCursor::new(), AnsiCursor::new());

        #[cfg(not(target_os = "windows"))]
        let cursor = Some(AnsiCursor::new() as Box<ITerminalCursor>);

        TerminalCursor { terminal_cursor: cursor , terminal: terminal}
    }

    /// Goto some position (x,y) in the terminal.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    ///
    /// use self::crossterm::cursor;
    ///     
    /// cursor::cursor().goto(10,10);
    /// 
    /// ```
    pub fn goto(&mut self, x: u16, y: u16) -> &mut TerminalCursor<'term> {
        if let Some(ref terminal_cursor) = self.terminal_cursor {
            terminal_cursor.goto(x, y, &self.terminal);
        }
        self
    }

    /// Get current cursor position (x,y) in the terminal.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    ///
    /// use self::crossterm::cursor;
    ///          
    /// let pos = cursor::cursor().pos();
    /// println!("{:?}", pos);
    /// 
    /// ```
    pub fn pos(&mut self) -> (u16, u16) {
        if let Some(ref terminal_cursor) = self.terminal_cursor {
            terminal_cursor.pos(&self.terminal)
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
    /// extern crate crossterm;
    ///
    /// use self::crossterm::cursor;
    ///      
    /// // Move 1 time up
    /// cursor::cursor().move_up(1);
    /// 
    /// // Move 2 times up
    /// cursor::cursor().move_up(2);
    /// 
    /// ```
    pub fn move_up(&mut self, count: u16) -> &mut TerminalCursor<'term> {
        if let Some(ref terminal_cursor) = self.terminal_cursor {
            terminal_cursor.move_up(count, &self.terminal);
        }
        self
    }

    /// Move the current cursor position `n` times right.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    ///
    /// use self::crossterm::cursor;
    ///
    ///
    /// // move 1 time right
    /// cursor::cursor().move_right(1);
    /// 
    /// // move 2 times right
    /// cursor::cursor().move_right(2);
    /// 
    /// ```
    pub fn move_right(&mut self, count: u16) -> &mut TerminalCursor<'term> {
        if let Some(ref terminal_cursor) = self.terminal_cursor {
            terminal_cursor.move_right(count, &self.terminal);
        }
        self
    }

    /// Move the current cursor position `n` times down.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    ///
    /// use self::crossterm::cursor;
    /// 
    /// // move 1 time down 
    /// cursor::cursor().move_down(1);
    /// 
    /// // move 2 times down
    /// cursor::cursor().move_down(2);
    ///
    /// ```
    pub fn move_down(&mut self, count: u16) -> &mut TerminalCursor<'term> {
        if let Some(ref terminal_cursor) = self.terminal_cursor {
            terminal_cursor.move_down(count, &self.terminal);
        }
        self
    }

    /// Move the current cursor position `n` times left.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    ///
    /// use self::crossterm::cursor;
    ///
    /// // move 1 time left
    /// cursor::cursor().move_left(1);
    /// 
    /// // move 2 time left
    /// cursor::cursor().move_left(2);
    /// 
    /// ```
    pub fn move_left(&mut self, count: u16) -> &mut TerminalCursor<'term> {
        if let Some(ref terminal_cursor) = self.terminal_cursor {
            terminal_cursor.move_left(count, &self.terminal);
        }
        self
    }

    /// Print an value at the current cursor position.
    ///
    /// This method prints an value with `print!()` and clears the buffer afterwards. 
    /// Rust's standard output is line-buffered. So your text gets sent to the console one line at a time.
    /// If you set the curosr position and try to `print!()` at that position and do not clear the buffer, than the character will not be printed at that position. 
    /// But will be printed when the next `println()` will be done.
    ///
    /// With this method you can print any displayable value at a certain position and the output buffer will be cleared afterwards.
    ///
    /// For more information see the cursor example in /examples/cursor
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    ///
    /// use self::crossterm::cursor;
    /// use std;
    /// use std::io::Write;
    ///
    /// // of course we can just do this.
    /// cursor::cursor().goto(10,10);
    /// print!("@");
    /// std::io::stdout().flush();
    /// 
    /// // but now we can chain the methods so it looks cleaner and it automatically flushes the buffer.  
    /// cursor::cursor()
    /// .goto(10,10)
    /// .print("@");
    /// 
    /// ```
    pub fn print<D: Display>(&mut self, value: D) -> &mut TerminalCursor<'term> {
        let mut screen = self.terminal.screen_manager.lock().unwrap();
        {
            write!(screen.stdout(), "{}", value);
            // rust is line buffered so we need to flush the buffer in order to print it at the current cursor position.
            screen.stdout().flush();
        }
        self
    }

    /// Save cursor position for recall later.
    ///
    /// Note that this position is stored program based not per instance of the `Cursor` struct.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    ///
    /// use self::crossterm::cursor;
    ///
    /// cursor::cursor().safe_position();
    ///
    /// ```
    pub fn save_position(&mut self)
    {
        if let Some(ref mut terminal_cursor) = self.terminal_cursor {
            terminal_cursor.save_position(&self.terminal);
        }
    }

    /// Return to saved cursor position
    ///
    /// Note that this method reset to the position set by `save_position()` and that this position is stored program based not per instance of the `Cursor` struct.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    ///
    /// use self::crossterm::cursor::cursor;
    ///
    /// cursor().reset_position();
    ///
    /// ```
    pub fn reset_position(&mut self)
    {
        if let Some(ref terminal_cursor) = self.terminal_cursor {
            terminal_cursor.reset_position(&self.terminal);
        }
    }
}

/// Get an TerminalCursor implementation whereon cursor related actions can be performed.
///
/// Check `/examples/cursor` in the libary for more spesific examples.
/// 
/// #Example
///
/// ```rust
///
/// extern crate crossterm;
///
/// use self::crossterm::cursor;
/// 
/// // Get cursor and goto pos X: 5, Y: 10
/// let mut cursor = cursor::cursor();
/// cursor.goto(5,10);
///     
/// //Or you can do it in one line.
/// cursor::cursor().goto(5,10);
///
/// ```
pub fn cursor<'term>(terminal: &'term Terminal) -> Box<TerminalCursor<'term>> {
    Box::from(TerminalCursor::new(&terminal))
}
