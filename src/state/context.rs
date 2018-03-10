//! This module is used for registering, storing an restoring the terminal state changes.

use std::ops::Drop;
use std::collections::HashMap;
use super::commands::IContextCommand;

/// Struct that stores the changed states of the terminal.
pub struct Context
{
    changed_states:  HashMap<i16, (Box<IContextCommand>) >,
}

impl Context
{
    /// Create new Context where the terminals states can be handled.
    pub fn new() -> Context
    {
        println!("Context has been created");
        Context { changed_states: HashMap::new() }
    }

    /// Restore all changes that are made to the terminal.
    pub fn restore_changes(&mut self)
    {
        for (x, state) in self.changed_states.iter_mut()
        {
            state.undo();
        }
    }

    /// Register new changed state with the given key.
    pub fn register_change(&mut self, change: Box<IContextCommand>, key: i16)
    {
        if !self.changed_states.contains_key(&key)
        {
            self.changed_states.insert(key, change);
        }
    }

    /// Undo an specific state by the given state key.
    pub fn undo_state(&mut self, state_key: i16)
    {
        if self.changed_states.contains_key(&state_key)
        {
            self.changed_states.remove(&state_key);
        }
    }
}



