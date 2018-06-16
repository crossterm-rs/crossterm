//! With this module you can perform actions that are cursor related.
//! Like changing and display the position of the cursor in terminal.
//!
//! Note that positions of the cursor are 0 -based witch means that the coordinates (cells) starts counting from 0

use super::*;
use Context;

use std::fmt::Display;
use std::io::Write;

/// Struct that stores an specific platform implementation for cursor related actions.
pub struct TerminalCursor<'context> {
    context: &'context Context,
    terminal_cursor: Option<Box<ITerminalCursor>>,
}

impl <'context> TerminalCursor<'context>
{
    /// Create new cursor instance whereon cursor related actions can be performed.
    pub fn new(context: &'context Context) -> TerminalCursor<'context> {
        #[cfg(target_os = "windows")]
        let cursor = functions::get_module::<Box<ITerminalCursor>>(WinApiCursor::new(), AnsiCursor::new());

        #[cfg(not(target_os = "windows"))]
        let cursor = Some(AnsiCursor::new() as Box<ITerminalCursor>);

        TerminalCursor { terminal_cursor: cursor , context}
    }

    /// Goto some position (x,y) in the terminal.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    ///  extern crate crossterm;
    ///  use self::crossterm::Context;
    ///  use self::crossterm::cursor;
    ///
    //    pub fn goto()
    //    {
    //        let context = Context::new();
    //
    //        // Get the cursor
    //        let mut cursor = cursor(&context);
    //        // Set the cursor to position X: 10, Y: 5 in the terminal
    //        cursor.goto(10,5);
    //    }
    ///
    /// ```
    pub fn goto(&mut self, x: u16, y: u16) -> &mut TerminalCursor<'context> {
        if let Some(ref terminal_cursor) = self.terminal_cursor {
            terminal_cursor.goto(x, y, &self.context);
        }
        self
    }

    /// Get current cursor position (x,y) in the terminal.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    ///  extern crate crossterm;
    ///  use self::crossterm::Context;
    ///  use self::crossterm::cursor;
    ///
    ///  pub fn pos()
    ///  {
    ///      let context = Context::new();
    ///
    ///      // Get the cursor
    ///      let mut cursor = cursor(&context);
    ///      // get the cursor position.
    ///      let (x,y) = cursor.pos();
    ///  }
    ///
    /// ```
    pub fn pos(&mut self) -> (u16, u16) {
        if let Some(ref terminal_cursor) = self.terminal_cursor {
            terminal_cursor.pos(&self.context)
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
    ///  extern crate crossterm;
    ///  use self::crossterm::Context;
    ///  use self::crossterm::cursor;
    ///
    /// pub fn move_up()
    /// {
    ///     let context = Context::new();
    ///
    ///     // Get the cursor
    ///     let mut cursor = cursor(&context);
    ///     // Move the cursor to position 3 times to the up in the terminal
    ///     cursor.move_up(3);
    /// }
    /// 
    /// ```
    pub fn move_up(&mut self, count: u16) -> &mut TerminalCursor<'context> {
        if let Some(ref terminal_cursor) = self.terminal_cursor {
            terminal_cursor.move_up(count, &self.context);
        }
        self
    }

    /// Move the current cursor position `n` times right.
    ///
    /// #Example
    ///
    /// ```rust
    ///  extern crate crossterm;
    ///  use self::crossterm::Context;
    ///  use self::crossterm::cursor;
    ///
    //  pub fn move_right()
    //  {
    //      let context = Context::new();
    //
    //      // Get the cursor
    //      let mut cursor = cursor(&context);
    //      // Move the cursor to position 3 times to the right in the terminal
    //      cursor.move_right(3);
    //  }
    /// ```
    pub fn move_right(&mut self, count: u16) -> &mut TerminalCursor<'context> {
        if let Some(ref terminal_cursor) = self.terminal_cursor {
            terminal_cursor.move_right(count, &self.context);
        }
        self
    }

    /// Move the current cursor position `n` times down.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    ///  extern crate crossterm;
    ///  use self::crossterm::Context;
    ///  use self::crossterm::cursor;
    ///
    /// pub fn move_down()
    /// {
    ///    let context = Context::new();
    ///
    ///    // Get the cursor
    ///    let mut cursor = cursor(&context);
    ///    // Move the cursor to position 3 times to the down in the terminal
    ///    cursor.move_down(3);
    /// }
    ///
    /// ```
    pub fn move_down(&mut self, count: u16) -> &mut TerminalCursor<'context> {
        if let Some(ref terminal_cursor) = self.terminal_cursor {
            terminal_cursor.move_down(count, &self.context);
        }
        self
    }

    /// Move the current cursor position `n` times left.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    ///  extern crate crossterm;
    ///  use self::crossterm::Context;
    ///  use self::crossterm::cursor;
    ///
    ///  pub fn move_left()
    ///  {
    ///      let context = Context::new();
    ///
    ///      // Get the cursor
    ///      let mut cursor = cursor(&context);
    ///      // Move the cursor to position 3 times to the left in the terminal
    ///      cursor.move_left(3);
    ///  }
    ///
    /// ```
    pub fn move_left(&mut self, count: u16) -> &mut TerminalCursor<'context> {
        if let Some(ref terminal_cursor) = self.terminal_cursor {
            terminal_cursor.move_left(count, &self.context);
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
    /// use self::crossterm::Context;
    /// use self::crossterm::cursor;
    ///
    /// use std;
    /// use std::io::Write;
    ///
    /// let context = Context::new();
    ///
    /// // of course we can just do this.
    /// cursor::cursor(&context).goto(10,10);
    /// print!("@");
    /// std::io::stdout().flush();
    /// 
    /// // but now we can chain the methods so it looks cleaner and it automatically flushes the buffer.  
    /// cursor::cursor(&context)
    /// .goto(10,10)
    /// .print("@");
    /// 
    /// ```
    pub fn print<D: Display>(&mut self, value: D) -> &mut TerminalCursor<'context> {
        let mut screen = self.context.screen_manager.lock().unwrap();
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
    /// use self::crossterm::Context;
    /// use self::crossterm::cursor;
    ///
    /// let context = Context::new();
    /// cursor::cursor(&context).safe_position();
    ///
    /// ```
    pub fn save_position(&mut self)
    {
        if let Some(ref mut terminal_cursor) = self.terminal_cursor {
            terminal_cursor.save_position(&self.context);
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
    /// use self::crossterm::cursor::cursor;
    /// use self::crossterm::Context;
    ///
    /// let context = Context::new();
    /// cursor(&context).reset_position();
    ///
    /// ```
    pub fn reset_position(&mut self)
    {
        if let Some(ref terminal_cursor) = self.terminal_cursor {
            terminal_cursor.reset_position(&self.context);
        }
    }
}

/// Get an TerminalCursor implementation whereon cursor related actions can be performed.
///
/// Check `/examples/version/cursor` in the libary for more spesific examples.
/// 
/// #Example
///
/// ```rust
///
///  extern crate crossterm;
///  use self::crossterm::Context;
///  use self::crossterm::cursor;
///
/// let context = Context::new();
///
/// // Get cursor and goto pos X: 5, Y: 10
/// let mut cursor = cursor::cursor(&context);
/// cursor.goto(5,10);
///     
/// //Or you can do it in one line.
/// cursor::cursor(&context).goto(5,10);
///
/// ```
pub fn cursor<'context>(context: &'context Context) -> Box<TerminalCursor<'context>> {
    Box::from(TerminalCursor::new(&context))
}
