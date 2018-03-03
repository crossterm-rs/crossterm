use super::commands::{IContextCommand, ICommand};
use std::ops::Drop;
use std::collections::HashMap;

pub struct Context
{
    changed_states:  HashMap<i16, Box<IContextCommand>>,
}

impl Context
{
    pub fn new() -> Context
    {
        Context { changed_states: HashMap::new() }
    }

    /// restore to default terminal state
    pub fn restore_changes(&mut self)
    {
//        println!("restore change");
        for (x, state) in self.changed_states.iter_mut()
        {
            state.undo();
//            println!("State is removed, total state");
        }
    }

    /// Register new changed state
    pub fn register_change(&mut self, change: Box<IContextCommand>, key: i16)
    {
//        println!("register change");
        if !self.changed_states.contains_key(&key)
        {
            self.changed_states.insert(key, change);
//            println!("State is registerd, total states: {}", self.changed_states.len());
        }
    }

    /// Undo state
    pub fn undo_state(&mut self, state_key: i16)
    {
//        println!("undo specific");
        if self.changed_states.contains_key(&state_key)
        {
            self.changed_states.remove(&state_key);
        }
    }
}
//
//fn state_wrapper() -> Context {
//    // Initialize it to a null value
//    static mut SINGLETON: *const StateWrapper = 0 as *const StateWrapper;
//    static ONCE: Once = ONCE_INIT;
//
//    unsafe {
//        ONCE.call_once(|| {
//            // Make it
//            let singleton = StateWrapper {
//                state: Arc::new(Mutex::new(State::new())),
//            };
//
//            // Put it in the heap so it can outlive this call
//            SINGLETON = mem::transmute(Box::new(singleton));
//        });
//
//        // Now we give out a copy of the data that is safe to use concurrently.
//        (*SINGLETON).clone()
//    }
//}



