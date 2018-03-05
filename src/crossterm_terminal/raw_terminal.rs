#[cfg(not(windows))]
use crossterm_state::commands::unix_command::EnableRawModeCommand;
#[cfg(windows)]
use crossterm_state::commands::win_commands::EnableRawModeCommand;

use { Construct, Context };
use crossterm_state::commands::{ICommand, IContextCommand};

use std::io::{ self, Write};


pub struct RawTerminal<'a, W: Write>
{
    output: W,
    key: i16,
    context: &'a mut Context
}

impl<'a, W: Write> Drop for RawTerminal<'a,W> {
    fn drop(&mut self) {
        self.context.undo_state(self.key);
    }
}

pub trait IntoRawMode: Write + Sized
{
    fn into_raw_mode<'a>(self, context: &'a mut Context) -> io::Result<RawTerminal<Self>>;
}

impl<W: Write> IntoRawMode for W
{
    fn into_raw_mode<'a>(self, context: &'a mut Context) -> io::Result<RawTerminal<Self>>
    {
        let (mut command, key) = EnableRawModeCommand::new(context);
        let success = command.execute();

        if success
        {
            Ok(RawTerminal {output: self, key: key, context: context})

        }else { panic!("cannot move into raw mode") }
    }
}

impl<'a, W: Write> Write for RawTerminal<'a, W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.output.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.output.flush()
    }
}