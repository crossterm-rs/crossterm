//! This module contains the commands that can be used for both unix and windows systems.

use super::ICommand;
use std::io;
use std::io::Write;

/// This command is used for switching to alternate screen and back to main screen.
#[derive(Clone, Copy)]
pub struct ToAlternateScreenBufferCommand;

impl ICommand for ToAlternateScreenBufferCommand
{
    fn new() -> Box<ToAlternateScreenBufferCommand> {
//        println!("create new unix alternate screen");
        Box::from(ToAlternateScreenBufferCommand { })
    }

    fn execute(&mut self) -> bool
    {
//        println!("execute alternate screen");
        let mut some_writer = io::stdout();
        match write!(some_writer, csi!("?1049h"))
        {
            Ok(_) => true,
            Err(_) => false
        }
    }

    fn undo(&mut self) -> bool
    {
//        println!("undo alternate screen");
        let mut some_writer = io::stdout();
        match write!(some_writer, csi!("?1049l"))
        {
            Ok(_) => true,
            Err(_) => false
        }
    }
}