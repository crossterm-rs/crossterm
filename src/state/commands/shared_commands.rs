//! This module contains the commands that can be used for both unix and windows systems. Or else said terminals that support ansi codes.
use Context;
use super::{ICommand, IStateCommand};

pub struct EmptyCommand;

impl IStateCommand for EmptyCommand
{
    fn execute(&mut self, terminal: &Context) -> bool
    {
       return false
    }

    fn undo(&mut self, terminal: &Context) -> bool
    {
       return false;
    }
}

/// This command is used for switching to alternate screen and back to main screen.
#[derive(Clone, Copy)]
pub struct ToAlternateScreenBufferCommand;

impl ICommand for ToAlternateScreenBufferCommand
{
    fn new() -> Box<ToAlternateScreenBufferCommand> {
        Box::from(ToAlternateScreenBufferCommand {})
    }

    fn execute(&mut self, terminal: &Context) -> bool
    {
        let mut screen = terminal.screen_manager.lock().unwrap();
        {
            screen.write_ansi_str(csi!("?1049h"));
            screen.toggle_is_alternate_screen(true);
            return true;
        }
    }

    fn undo(&mut self, terminal: &Context) -> bool
    {
        let mut screen = terminal.screen_manager.lock().unwrap();
        {
            screen.write_ansi_str(csi!("?1049l"));
            screen.toggle_is_alternate_screen(false);
            return true;
        }
    }
}