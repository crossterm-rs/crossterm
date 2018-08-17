//! This module provides one place to work with the screen.
//!
//!   In Rust we can call `stdout()` to get an handle to the current default console handle.
//!   For example when in unix systems you want to print something to the main screen you can use the following code:
//!
//!   ```
//!   write!(std::io::stdout(), "{}", "some text").
//!   ```
//!
//!   But things change when we are in alternate screen modes.
//!   We can not simply use `stdout()` to get a handle to the alternate screen, since this call returns the current default console handle (mainscreen).
//!
//!   Instead we need to store an handle to the screen output.
//!   This handle could be used to put into alternate screen modes and back into main screen modes.
//!   Through this stored handle Crossterm can execute its command on the current screen whether it be alternate screen or main screen.
//!
//!   For unix systems we store the handle gotten from `stdout()` for windows systems that are not supporting ANSI escape codes we store WinApi `HANDLE` struct witch will provide access to the current screen.
//!
//! This is the reason why this module exits: it is to provide access to the current terminal screen whether it will be the alternate screen and main screen.

use super::*;
use ::input::TerminalInput;

use modules::write::IStdout;

use std::any::Any;
use std::fmt::Display;
use std::io::{self, Write};
use std::default::Default;

#[cfg(target_os = "windows")]
use winapi::um::winnt::HANDLE;


pub type TerminalOutput = Box<IStdout + Send + Sync>;

pub fn terminal_output() -> TerminalOutput {
    #[cfg(target_os = "windows")]
    let r = functions::get_module::<Box<IStdout + Send + Sync>>(
        Box::from(WinApiStdout::new()),
        Box::from(AnsiStdout::new()),
    ).unwrap();
    
    #[cfg(not(target_os = "windows"))]
    let r = Box::from(AnsiStdout::new()) as Box<IStdout + Send + Sync>;
    r
}

