//! This module is used for registering, storing an restoring the terminal state changes.

use super::commands::shared_commands::EmptyCommand;
use super::commands::IStateCommand;

use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;

/// Struct that stores the changed states of the terminal.
pub struct StateManager {
    changed_states: HashMap<u16, Rc<Mutex<Box<IStateCommand>>>>,
}

impl StateManager {
    /// Create new Context where the terminals states can be handled.
    pub fn new() -> StateManager {
        StateManager {
            changed_states: HashMap::new(),
        }
    }

    /// Restore all changes that are made to the terminal.
    pub fn restore_changes(&mut self) {
        for (id, item) in self.changed_states.iter_mut() {
//            let mut item = item.lock().unwrap();
//            item.undo();
//
//            println!("undo!");
        }
    }

    /// Register new changed state with the given key.
    pub fn register_change(&mut self, change: Box<IStateCommand>, key: u16) {
        self.changed_states.insert(key, Rc::new(Mutex::new(change)));
    }

    /// Get an state command from storage by id.
    pub fn get(&mut self, state_key: u16) -> Rc<Mutex<Box<IStateCommand>>> {
        if self.changed_states.contains_key(&state_key) {
            return self.changed_states[&state_key].clone();
        }

        return Rc::new(Mutex::new(Box::new(EmptyCommand)));
    }

    pub fn get_changes_count(&self) -> u16 {
        return self.changed_states.len() as u16;
    }
}
