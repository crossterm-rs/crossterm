//! This is an ANSI specific implementation for the screen manager
//! This module is used for windows 10 terminals and unix terminals by default.
//! This module uses the stdout to write to the console.

use std::any::Any;
use std::io::{self, Write};

use super::IScreenManager;

pub struct AnsiScreenManager {
    pub is_alternate_screen: bool,
    output: Box<Write>,
}

impl IScreenManager for AnsiScreenManager {
    fn toggle_is_alternate_screen(&mut self, is_alternate_screen: bool) {
        self.is_alternate_screen = is_alternate_screen;
    }

    fn write_ansi(&mut self, string: String) {
        write!(self.output, "{}", string);
        self.flush();
    }

    fn write_ansi_str(&mut self, string: &str) {
        write!(self.output, "{}", string);
        self.flush();
    }

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.output.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.output.flush()
    }

    fn as_any(&mut self) -> &mut Any {
        self
    }
}

impl AnsiScreenManager {
    pub fn new() -> Self {
        AnsiScreenManager {
            output: (Box::from(io::stdout()) as Box<Write>),
            is_alternate_screen: false,
        }
    }
}
