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
    pub fn new(raw_mode: bool) -> Screen
    {
        if raw_mode
        {
            RawScreen::into_raw_mode();;
            return Screen { stdout: Arc::new(Stdout::new(true)), buffer: Vec::new() };
        }

        return Screen::default();
    }

    pub fn from(stdout: Stdout) -> Screen
    {
        return Screen { stdout: Arc::new(stdout), buffer: Vec::new() };
    }

    pub fn enable_raw_modes(&self) -> Result<()> {
        RawScreen::into_raw_mode()?;
        return Ok(())
    }

    pub fn enable_alternate_modes(&self, raw_mode: bool) -> Result<AlternateScreen> {
        let mut stdout = Stdout::new(raw_mode);

        if raw_mode
        {
            RawScreen::into_raw_mode();
        }

        let alternate_screen = AlternateScreen::to_alternate_screen(stdout)?;
        return Ok(alternate_screen);
    }
}

impl Default for Screen
{
    fn default() -> Self {
        return Screen { stdout: Arc::new(Stdout::new(false)), buffer: Vec::new() };
    }
}

impl Drop for Screen
{
    fn drop(&mut self) {
        if self.stdout.is_in_raw_mode
        {
            RawScreen::disable_raw_modes();
        }
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