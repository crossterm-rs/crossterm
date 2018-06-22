//! This module contains the commands that can be used for both unix and windows systems. Or else said terminals that support ansi codes.
use Context;
use super::{IStateCommand};

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
pub struct ToAlternateScreenBufferCommand<'a>
{
    context: &'a Context
}

impl<'a> ToAlternateScreenBufferCommand<'a>
{
    pub fn new(context: & 'a Context) -> Box < ToAlternateScreenBufferCommand > {
        Box::from(ToAlternateScreenBufferCommand {context: context})
    }
}

impl<'context> IStateCommand for ToAlternateScreenBufferCommand<'context>
{
    fn execute(&mut self) -> bool
    {
        let mut screen = self.context.screen_manager.lock().unwrap();
        {
            screen.write_ansi_str(csi!("?1049h"));
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