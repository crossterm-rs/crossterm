#[cfg(not(windows))]
use common::commands::unix_command;

#[cfg(windows)]
use common::commands::win_commands;

use common::commands::IAlternateScreenCommand;

use super::{AlternateScreen,RawScreen};
use super::super::super::modules::write::Stdout;

use std::io::Write;
use std::io::Result;
use std::sync::Arc;

pub struct Screen
{
    buffer: Vec<u8>,
    pub stdout: Arc<Stdout>,
}

impl Screen
{
    pub fn new() -> Screen
    {
        return Screen { stdout: Arc::new(Stdout::new(false)), buffer: Vec::new() };
    }

    pub fn from(stdout: Stdout) -> Screen
    {
        return Screen { stdout: Arc::new(stdout), buffer: Vec::new() };
    }

    pub fn enable_raw_modes(&self) -> Result<RawScreen> {
        let mut screen = Screen::from(Stdout::new(true));
        let raw_screen = RawScreen::into_raw_mode(screen)?;
        return Ok(raw_screen)
    }

    pub fn enable_alternate_modes(&self) -> Result<AlternateScreen> {
        let mut stdout = Stdout::new(true);
        let alternate_screen = AlternateScreen::to_alternate_screen(stdout)?;
        return Ok(alternate_screen);
    }
}

impl Write for Screen
{
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.buffer.write(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        self.stdout.write_buf(&self.buffer);
        self.stdout.flush()
    }
}