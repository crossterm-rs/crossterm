#[cfg(unix)]
pub mod unix;

#[cfg(windows)]
pub mod winapi;

use crossterm_utils::TerminalOutput;

use std::io;

/// This command is used for switching to alternate screen and back to main screen.
pub struct ToAlternateScreenCommand;

impl ToAlternateScreenCommand {
    pub fn new() -> ToAlternateScreenCommand {
        ToAlternateScreenCommand
    }
}

impl IAlternateScreenCommand for ToAlternateScreenCommand {
    /// enable alternate screen.
    fn enable(&self, stdout: &mut TerminalOutput) -> io::Result<()> {
        stdout.write_str(csi!("?1049h"))?;
        Ok(())
    }

    /// disable alternate screen.
    fn disable(&self, stdout: &TerminalOutput) -> io::Result<()> {
        stdout.write_str(csi!("?1049l"))?;
        Ok(())
    }
}

/// This trait provides a way to execute some state changing commands.
pub trait IStateCommand {
    fn execute(&mut self) -> io::Result<()>;
    fn undo(&mut self) -> io::Result<()>;
}

pub trait IEnableAnsiCommand {
    fn enable(&self) -> io::Result<bool>;
    fn disable(&self) -> io::Result<()>;
}

// This trait provides an interface for switching to alternate screen and back.
pub trait IAlternateScreenCommand: Sync + Send {
    fn enable(&self, stdout: &mut TerminalOutput) -> io::Result<()>;
    fn disable(&self, stdout: &TerminalOutput) -> io::Result<()>;
}

// This trait provides an interface for switching to raw mode and back.
pub trait IRawScreenCommand: Sync + Send {
    fn enable(&mut self) -> io::Result<()>;
    fn disable(&self) -> io::Result<()>;
}
