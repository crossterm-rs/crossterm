//! This module is used for registering, storing an restoring the terminal state changes.

use std::collections::HashMap;
use super::commands::IContextCommand;
use Terminal;
/// Struct that stores the changed states of the terminal.
pub struct Context
{
    changed_states: HashMap<u16, Box<IContextCommand>>,
}

impl Context
{
    /// Create new Context where the terminals states can be handled.
    pub fn new() -> Context
    {
        Context {
            changed_states: HashMap::new(),
        }
    }

//    /// Restore all changes that are made to the terminal.
//    pub fn restore_changes(&mut self)
//    {
//        use std::iter::FromIterator;
//
//        let mut buffer = Vec::new();
//
//        for i in 0..self.changed_states.len()
//        {
//            buffer[i] = self.changed_states.iter().nth(i).unwrap();
//        }
//
//        for i in 0..buffer.len()
//        {
//            buffer[i].1.undo(self);
//        }
//    }

    /// Register new changed state with the given key.
    pub fn register_change(&mut self, change: Box<IContextCommand>, key: u16)
    {
        if !self.changed_states.contains_key(&key)
        {
            self.changed_states.insert(key, change);
        }
    }

    /// Undo an specific state by the given state key.
    pub fn undo_state(&mut self, state_key: u16, terminal: &Terminal)
    {
        if self.changed_states.contains_key(&state_key)
        {
            {
                let mut command = self.changed_states.get_mut(&state_key).unwrap();
                command.undo(&terminal);
            }
            &self.changed_states.remove(&state_key);

        }
    }
}



