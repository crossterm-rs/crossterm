use std::io;

use super::*;
use std::rc::Rc;
use {Context, ScreenManager };

pub struct TerminalInput<'terminal> {
    terminal_input: Box<ITerminalInput>,
    screen_manager: &'terminal ScreenManager
}

impl<'terminal> TerminalInput<'terminal> {
    pub fn new(screen_manager: &'terminal ScreenManager) -> TerminalInput<'terminal> {
        #[cfg(target_os = "windows")]
        let input = Box::from(WindowsInput::new());

        #[cfg(not(target_os = "windows"))]
        let input = Box::from(UnixInput::new());

        TerminalInput {
            terminal_input: input,
            screen_manager: screen_manager
        }
    }

    pub fn read_line(&self) -> io::Result<String> {
        self.terminal_input.read_line(&self.screen_manager)
    }

    pub fn read_char(&self) -> io::Result<char> {
        return self.terminal_input.read_char(&self.screen_manager);
    }

    pub fn read_async(&self) -> AsyncReader {
        self.terminal_input.read_async(&self.screen_manager)
    }

    pub fn read_until_async(&self, delimiter: u8) -> AsyncReader {
        self.terminal_input.read_until_async(delimiter,&self.screen_manager)
    }
}

pub fn input(screen_manager: &ScreenManager) -> TerminalInput {
    return TerminalInput::new(screen_manager)
}
