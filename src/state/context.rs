//! This module contains the code for the context of the terminal.

use {ScreenManager, StateManager};

use std::rc::Rc;
use std::sync::Mutex;

/// This type contains the context of the current terminal. The context surrounds the changed states of the terminal and can be used for managing the output of the terminal.
pub struct Context {
    pub screen_manager: Rc<Mutex<ScreenManager>>,
    pub state_manager: Mutex<StateManager>,
}

impl Context {
    /// Create new Context instance so that you can provide it to other modules like terminal, cursor and color
    ///
    /// This context type is just an wrapper that crossterm uses for managing the state the terminal.
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
    pub fn new() -> Rc<Context> {
        Rc::new(Context {
            screen_manager: Rc::new(Mutex::new(ScreenManager::new())),
            state_manager: Mutex::new(StateManager::new()),
        })
    }
}

use std::io::Write;

impl Drop for Context {
    fn drop(&mut self) {
        panic!();
        let mut changes = self.state_manager.lock().unwrap();
        changes.restore_changes();
    }
}
