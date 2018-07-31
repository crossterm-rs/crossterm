//! This is an ANSI specific implementation for the screen manager
//! This module is used for windows 10 terminals and unix terminals by default.
//! This module uses the stdout to write to the console.

use super::IScreenManager;

use std::any::Any;
use std::cell::RefCell;
use std::sync::{Arc,Mutex};
use std::io::{self, Read, Write,Stdout};
use std::str::from_utf8;

/// This struct is an ANSI escape code implementation for screen related actions.
pub struct AnsiScreenManager {
    is_alternate_screen: bool,
    is_raw_screen: bool,
    output: Box<Stdout>,
}

impl IScreenManager for AnsiScreenManager {
    fn set_is_raw_screen(&mut self, value: bool) {
        self.is_raw_screen = value;
    }

    fn set_is_alternate_screen(&mut self, value: bool) {
        self.is_alternate_screen = value;
    }

    fn is_raw_screen(&self) -> bool {
        self.is_raw_screen
    }

    fn is_alternate_screen(&self) -> bool {
        self.is_alternate_screen
    }

    fn write_str(&self, string: &str) -> io::Result<usize> {
        let out = &self.output;
        let mut handle = out.lock();
        write!(handle, "{}", string)?;
        Ok(0)
    }

    fn write(&self, buf: &[u8]) -> io::Result<usize> {
        {
            let out = &self.output;
            let mut handle = out.lock();
            handle.write(buf)?;
        }
        Ok(0)
    }

    fn flush(&self) -> io::Result<()> {
        let out = &self.output;
        let mut handle = out.lock();
        handle.flush();

        Ok(())
    }

    fn as_any(&self) -> &Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut Any {
        self
    }
}

impl AnsiScreenManager {
    pub fn new() -> Self {
        AnsiScreenManager {
            output: Box::from(io::stdout()),
            is_alternate_screen: false,
            is_raw_screen: false,
        }
    }
}
