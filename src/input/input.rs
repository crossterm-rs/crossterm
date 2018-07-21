use std::io;

use super::*;
use Context;
use std::rc::Rc;

pub struct TerminalInput
{
    context: Rc<Context>,
    terminal_input: Box<ITerminalInput>,
}

impl TerminalInput
{
    pub fn new(context: Rc<Context>) -> TerminalInput
    {
        #[cfg(target_os = "windows")]
        let input = Box::from(WindowsInput::new(context.clone()));

        #[cfg(not(target_os = "windows"))]
        let input = Box::from(UnixInput::new());

        TerminalInput {
            terminal_input: input,
            context,
        }
    }

    pub fn read_line(&self) -> io::Result<String>
    {
        self.terminal_input.read_line()
    }

    pub fn read_char(&self) -> io::Result<char>
    {
        return self.terminal_input.read_char()
    }

    pub fn read_key(&self) -> io::Result<Key>
    {
        self.terminal_input.read_pressed_key()
    }

    pub fn read_async(&self) -> AsyncReader
    {
        self.terminal_input.read_async()
    }

    pub fn read_until_async(&self, delimiter: u8) -> AsyncReader
    {   self.terminal_input.read_until_async(delimiter)
    }
}

pub fn input(context: &Rc<Context>) -> Box<TerminalInput>
{
    return Box::from(TerminalInput::new(context.clone()));
}