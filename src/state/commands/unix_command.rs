//! This module contains the commands that can be used for unix systems.

use {Terminal, Context};
use super::IContextCommand;
use kernel::unix_kernel::terminal;
use termios::{Termios, tcsetattr, TCSAFLUSH, ICANON, ECHO, CREAD};

const FD_STDIN: ::std::os::unix::io::RawFd = 1;

use std::sync::Mutex;

/// This command is used for switching to NoncanonicalMode.
#[derive(Copy, Clone)]
pub struct NoncanonicalModeCommand
{
    key: u16
}

impl IContextCommand for NoncanonicalModeCommand
{
    fn new(context: &Mutex<Context>) -> (Box<NoncanonicalModeCommand>, u16) {
        let key = 1;

        let mut context = context.lock().unwrap();
        {
            let command = NoncanonicalModeCommand { key: key };
            context.register_change(Box::from(command), key);
            (Box::from(command),key)
        }
    }

    fn execute(&mut self, terminal: &Terminal) -> bool
    {
        // Set noncanonical mode
        if let Ok(orig) = Termios::from_fd(FD_STDIN)
        {
            let mut noncan = orig.clone();
            noncan.c_lflag &= !ICANON;
            noncan.c_lflag &= !ECHO;
            noncan.c_lflag &= !CREAD;
            match tcsetattr(FD_STDIN, TCSAFLUSH, &noncan)
            {
                Ok(_) => return true,
                Err(_) => return false,
            };
        }else {
            return false
        }
    }

    fn undo(&mut self, terminal: &Terminal) -> bool
    {
        // Disable noncanonical mode
        if let Ok(orig) = Termios::from_fd(FD_STDIN)
            {
                let mut noncan = orig.clone();
                noncan.c_lflag &= ICANON;
                noncan.c_lflag &= ECHO;
                noncan.c_lflag &= CREAD;

                match tcsetattr(FD_STDIN, TCSAFLUSH, &noncan)
                    {
                        Ok(_) => return true,
                        Err(_) => return false,
                    };
            }else {
            return false;
        }
    }
}

/// This command is used for enabling and disabling raw mode for the terminal.
#[derive(Copy, Clone)]
pub struct EnableRawModeCommand
{
    original_mode: Option<Termios>,
    command_id: u16
}

impl IContextCommand for EnableRawModeCommand
{
    fn new(context: &Mutex<Context>) -> (Box<EnableRawModeCommand>, u16) {
        let key = 2;

        let mut context = context.lock().unwrap();
        {
            let command = EnableRawModeCommand { original_mode: None, command_id: key };
            context.register_change(Box::from(command), key);
            (Box::from(command), key)
        }
    }

    fn execute(&mut self, terminal: &Terminal) -> bool
    {
        let original_mode = terminal::get_terminal_mode();

        if let Ok(original_mode) = original_mode
        {
            panic!("setting {:?}", original_mode);
            self.original_mode = Some(original_mode);
            let mut new_mode = original_mode;
            terminal::make_raw(&mut new_mode);
            terminal::set_terminal_mode(&new_mode);
            true

        }else {
            return false;
        }
    }

    fn undo(&mut self, terminal: &Terminal) -> bool
    {
        panic!("undoing {:?}", self.original_mode);
        if let Some(original_mode) = self.original_mode
        {

            let result = terminal::set_terminal_mode(&original_mode);

            match result
            {
                Ok(()) => true,
                Err(_) => false
            }
        }else {
            return false;
        }
    }
}
