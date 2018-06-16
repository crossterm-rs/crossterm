use std::rc::Rc;
use std::sync::Mutex;
use Context;
use super::commands::IStateCommand;

/// Simple wrapper for executing an command.
pub struct CommandManager;

impl CommandManager
{
    /// execute an certain command by id.
    pub fn execute(terminal: &Context, command_id: u16) -> bool
    {
        let mut mutex: Rc<Mutex<Box<IStateCommand>>>;

        let mut state = terminal.state_manager.lock().unwrap();
        {
            mutex = state.get(command_id);
        }

        let mut command = mutex.lock().unwrap();
        let has_succeeded = command.execute(&terminal);
        return has_succeeded;
    }

    /// undo an certain command by id.
    pub fn undo(terminal: &Context, command_id: u16) -> bool
    {
        let mut mutex: Rc<Mutex<Box<IStateCommand>>>;

        let mut state = terminal.state_manager.lock().unwrap();
        {
            mutex = state.get(command_id);
        }

        let mut command = mutex.lock().unwrap();
        let has_succeeded = command.undo(&terminal);
        return has_succeeded;
    }
}