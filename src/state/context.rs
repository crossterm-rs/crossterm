//! This module contains the code for the context of the terminal.

use std::sync::Mutex;
use std::rc::Rc;

use super::super::style;
use { StateManager, ScreenManager };

use std::fmt;

/// This type contains the context of the current terminal. The context surrounds the changed states of the terminal and can be used for managing the output of the terminal.
pub struct Context
{
    pub screen_manager: Rc<Mutex<ScreenManager>>,
    pub state_manager: Mutex<StateManager>
}

impl Context
{
    /// Create new Context instance so that you can provide it to other modules like terminal, cursor and color
    ///
    /// This context type is just an wrapper that crossterm uses for managin the state the terminal.
    ///
    /// You must provide this context otherwise crossterm would not be able to restore to the original state of the terminal.
    /// Also futures like rawscreen and ansi codes can not be used.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// use crossterm::Context;
    ///
    /// use crossterm::cursor;
    /// use crossterm::color;
    /// use crossterm::terminal;
    ///
    /// let cursor = cursor::cursor(&context)
    /// let terminal = terminal::terminal(&context);
    /// let color = terminal::color(&context);
    ///
    /// ```
    pub fn new() -> Context
    {
        Context {
            screen_manager: Rc::new(Mutex::new(ScreenManager::new())),
            state_manager: Mutex::new(StateManager::new())
        }
    }

    /// Wraps an displayable object so it can be formatted with colors and attributes.
    ///
    /// Check `/examples/color` in the libary for more spesific examples.
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate crossterm;
    ///
    /// use self::crossterm::style::{paint,Color};
    ///
    /// fn main()
    /// {
    ///     // Create an styledobject object from the text 'Unstyled font'
    ///     // Currently it has the default foregroundcolor and backgroundcolor.
    ///     println!("{}",paint("Unstyled font"));
    ///
    ///     // Create an displayable object from the text 'Colored font',
    ///     // Paint this with the `Red` foreground color and `Blue` backgroundcolor.
    ///     // Print the result.
    ///     let styledobject = paint("Colored font").with(Color::Red).on(Color::Blue);
    ///     println!("{}", styledobject);
    ///
    ///     // Or all in one line
    ///     println!("{}", paint("Colored font").with(Color::Red).on(Color::Blue));
    /// }
    /// ```
    pub fn paint<D>(&self, val: D) -> style::StyledObject<D>
        where
            D: fmt::Display,
    {
        style::ObjectStyle::new().apply_to(val, &self)
    }
}

impl Drop for Context
{
    fn drop(&mut self)
    {
        let mut changes = self.state_manager.lock().unwrap();
        changes.restore_changes(&self);
    }
}
