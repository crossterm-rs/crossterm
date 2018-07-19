use std::io;
use std::io::Write;

use super::*;

pub struct WindowsInput
{ }

impl WindowsInput
{
    pub fn new() -> WindowsInput
    {
        WindowsInput {}
    }
}

impl ITerminalInput for WindowsInput
{
    fn read_char(&self) -> io::Result<String>
    {
        let mut rv = String::new();
        Ok(rv)
    }

    fn read_key(&self) -> io::Result<()>
    {
        let mut rv = String::new();
        Ok(())
    }

    fn read_async(&self)
    {

    }

    fn read_until(&self, delimiter: u8)
    {

    }
}