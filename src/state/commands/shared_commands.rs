//! This module contains the commands that can be used for both unix and windows systems. Or else said terminals that support ansi codes.
use Context;
use super::{IStateCommand};

use std::rc::Rc;

pub struct EmptyCommand;

impl IStateCommand for EmptyCommand
{
    fn execute(&mut self) -> bool
    {
       return false
    }

    fn undo(&mut self) -> bool
    {
       return false;
    }
}

/// This command is used for switching to alternate screen and back to main screen.
pub struct ToAlternateScreenBufferCommand
{
    context: Rc<Context>
}

impl ToAlternateScreenBufferCommand
{
    pub fn new(context: Rc<Context>) -> u16 {

        let mut state = context.state_manager.lock().unwrap();
        {
            let key = state.get_changes_count();
            let command = ToAlternateScreenBufferCommand {context: context.clone()};

            state.register_change(Box::from(command), key);
            key
        }
    }
}

impl IStateCommand for ToAlternateScreenBufferCommand
{
    fn execute(&mut self) -> bool
    {

        println!("asdfasdf");
        let mut screen = self.context.screen_manager.lock().unwrap();
        {
            screen.toggle_is_alternate_screen(true);
            return true;
        }
    }

    fn undo(&mut self) -> bool
    {
        let mut screen = self.context.screen_manager.lock().unwrap();
        {
            screen.write_ansi_str(csi!("?1049l"));
            screen.toggle_is_alternate_screen(false);
            return true;
        }
    }
}