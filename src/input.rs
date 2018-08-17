//! With this module you can perform actions that are input related.
//! Like reading a line, reading a character and reading asynchronously.

use std::io;
use std::sync::Arc;
use super::*;

pub type TerminalInput = Box<ITerminalInput>;

pub fn input() -> TerminalInput {
    #[cfg(target_os = "windows")]
    let r = Box::from(WindowsInput::new());
    
    #[cfg(not(target_os = "windows"))]
    let r = Box::from(UnixInput::new());
    r
}

