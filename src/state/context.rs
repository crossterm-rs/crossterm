//! This module is used for registering, storing an restoring the terminal state changes.

use std::ops::Drop;
use std::collections::HashMap;
use std::io::Write;
use std::sync::Mutex;

use super::commands::{ ICommand, IContextCommand};
use super::super::manager::ScreenManager;

/// Struct that stores the changed states of the terminal.
pub struct Context
{
    changed_states: HashMap<i16, Box<IContextCommand>>,
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

    /// Restore all changes that are made to the terminal.
    pub fn restore_changes(&mut self)
    {
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



