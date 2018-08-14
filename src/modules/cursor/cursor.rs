//! With this module you can perform actions that are cursor related.
//! Like changing and display the position of the cursor in terminal.
//!
//! Note that positions of the cursor are 0 -based witch means that the coordinates (cells) starts counting from 0

use super::*;

use std::fmt::Display;
use std::io::Write;

use std::sync::Arc;

/// Struct that stores an specific platform implementation for cursor related actions.
///
/// Check `/examples/version/cursor` in the library for more specific examples.
///
/// #Example
///
/// ```rust
///
/// extern crate crossterm;
/// use self::crossterm::cursor::cursor;
/// use self::crossterm::Screen;
///
/// let screen = Screen::default();
/// let mut cursor = cursor(&screen);
///
/// // Get cursor and goto pos X: 5, Y: 10
/// cursor.goto(5,10);
/// 
/// cursor.show();
/// cursor.hide();
/// cursor.blink(true);
/// cursor.move_left(2);
/// 
/// ```
pub struct TerminalCursor<'stdout> {
    screen: &'stdout Arc<Stdout>,
    terminal_cursor: Box<ITerminalCursor>,
}

impl<'stdout> TerminalCursor<'stdout> {
    /// Create new cursor instance whereon cursor related actions can be performed.
    pub fn new(screen: &'stdout Arc<Stdout>) -> TerminalCursor<'stdout> {
        #[cfg(target_os = "windows")]
        let cursor =
            functions::get_module::<Box<ITerminalCursor>>(WinApiCursor::new(), AnsiCursor::new())
                .unwrap();

        #[cfg(not(target_os = "windows"))]
        let cursor = AnsiCursor::new() as Box<ITerminalCursor>;

        TerminalCursor {
            terminal_cursor: cursor,
            screen: screen,
        }
    }

    /// Goto some position (x,y) in the terminal.
    ///
    /// #Example
    ///
    /// ```rust
    /// let screen = Screen::default();
    /// let cursor = cursor(&screen);
    ///
    /// // change the cursor to position, x: 4 and y: 5
    /// cursor.goto(4,5);
    ///
    /// ```
    pub fn goto(&self, x: u16, y: u16) {
        self.terminal_cursor.goto(x, y, &self.screen);
    }

    /// Get current cursor position (x,y) in the terminal.
    ///
    /// #Example
    ///
    /// ```rust
    /// 
    /// let screen = Screen::default();
    /// let cursor = cursor(&screen);
    ///
    /// // get the current cursor pos
    /// let (x,y) = cursor.pos();
    ///
    /// ```
    pub fn pos(&self) -> (u16, u16) {
        self.terminal_cursor.pos(&self.screen)
    }

    /// Move the current cursor position `n` times up.
    /// #Example
    ///
    /// ```rust
    ///
    /// let screen = Screen::default();
    /// let cursor = cursor(&screen);
    ///
    /// // Move the cursor to position 3 times to the up in the terminal
    /// cursor.move_up(3);
    ///
    /// ```
    pub fn move_up(&mut self, count: u16) -> &mut TerminalCursor<'stdout> {
        self.terminal_cursor.move_up(count, &self.screen);
        self
    }

    /// Move the current cursor position `n` times right.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// let screen = Screen::default();
    /// let cursor = cursor(&screen);
    ///
    /// // Move the cursor to position 3 times to the right in the terminal
    /// cursor.move_right(3);
    ///
    /// ```
    pub fn move_right(&mut self, count: u16) -> &mut TerminalCursor<'stdout> {
        self.terminal_cursor.move_right(count, &self.screen);
        self
    }

    /// Move the current cursor position `n` times down.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// let screen = Screen::default();
    /// let cursor = cursor(&screen);
    ///
    /// // Move the cursor to position 3 times to the down in the terminal
    /// cursor.move_down(3);
    ///
    /// ```
    pub fn move_down(&mut self, count: u16) -> &mut TerminalCursor<'stdout> {
        self.terminal_cursor.move_down(count, &self.screen);
        self
    }

    /// Move the current cursor position `n` times left.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// let screen = Screen::default();
    /// let cursor = cursor(&screen);
    ///
    ///  // Move the cursor to position 3 times to the left in the terminal
    ///  cursor.move_left(3);
    ///
    /// ```
    pub fn move_left(&mut self, count: u16) -> &mut TerminalCursor<'stdout> {
        self.terminal_cursor.move_left(count, &self.screen);
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
    /// let screen = Screen::default();
    /// let cursor = cursor(&screen);
    ///
    /// cursor.safe_position();
    ///
    /// ```
    pub fn save_position(&self) {
        self.terminal_cursor.save_position(&self.screen);
    }

    /// Return to saved cursor position
    ///
    /// Note that this method reset to the position set by `save_position()` and that this position is stored program based not per instance of the `Cursor` struct.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// let screen = Screen::default();
    /// let cursor = cursor(&screen);
    ///
    /// cursor.reset_position();
    ///
    /// ```
    pub fn reset_position(&self) {
        self.terminal_cursor.reset_position(&self.screen);
    }

    /// Hide de cursor in the console.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// let cursor = cursor(&Screen::default());
    /// cursor.hide();
    ///
    /// ```
    pub fn hide(&self) {
        self.terminal_cursor.hide(&self.screen);
    }

    /// Show the cursor in the console.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// let screen = Screen::default();
    /// let cursor = cursor(&screen);
    /// cursor.show();
    ///
    /// ```
    pub fn show(&self) {
        self.terminal_cursor.show(&self.screen);
    }

    /// Enable or disable blinking of the terminal.
    ///
    /// Not all terminals are supporting this functionality. Windows versions lower than windows 10 also are not supporting this version.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// let screen = Screen::default();
    /// let cursor = cursor(&screen);
    /// cursor.blink(true);
    /// cursor.blink(false);
    ///
    /// ```
    pub fn blink(&self, blink: bool) {
        self.terminal_cursor.blink(blink, &self.screen);
    }
}

/// Get an TerminalCursor implementation whereon cursor related actions can be performed.
/// Pass the reference to any screen you want this type to perform actions on.
pub fn cursor<'stdout>(screen_manager: &'stdout Screen) -> TerminalCursor<'stdout> {
    TerminalCursor::new(&screen_manager.stdout)
}
