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
        let input = Box::from(WindowsInput::new());

        #[cfg(not(target_os = "windows"))]
        let cursor = Box::from(UnixInput::new());

        TerminalInput {
            terminal_input: input,
            context,
        }
    }

    pub fn read_line(&self) -> io::Result<String>
    {
        let mut rv = String::new();
        io::stdin().read_line(&mut rv)?;
        let len = rv.trim_right_matches(&['\r', '\n'][..]).len();
        rv.truncate(len);
        Ok(rv)
    }

    fn read_char(&self) -> io::Result<String>
    {
        // todo: read char
        Ok(String::new())
    }

    fn read_key(&self) -> io::Result<()>
    {
        // todo: read pressed key
        Ok(())
    }

    fn read_async(&self)
    {
        // todo: async reading
    }

    fn read_until(&self, delimiter: u8)
    {   // todo: read until char
    }
}

pub fn input(context: &Rc<Context>) -> Box<TerminalInput>
{
    return Box::from(TerminalInput::new(context.clone()));
}