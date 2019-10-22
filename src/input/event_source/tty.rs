use crate::EventSource;
use crate::input::events::InternalEvent;
use crate::input::sys::unix::{FileDesc, tty_fd, TtyPoll};
use crate::InputEvent;
use crate::Result;

pub struct TTYEventSource {
    source: TtyPoll,
}

impl TTYEventSource {
    pub fn new() -> TTYEventSource {
        TTYEventSource::from_file_descriptor(tty_fd().unwrap())
    }

    pub fn from_file_descriptor(input_fd: FileDesc) -> TTYEventSource {
        TTYEventSource {
            source: TtyPoll::new(input_fd),
        }
    }
}

impl EventSource for TTYEventSource {
    fn read_event(&mut self) -> Result<Option<InputEvent>> {
        match self.source.tty_poll() {
            Ok(Some(InternalEvent::Input(event))) => return Ok(Some(event)),
            Ok(Some(InternalEvent::CursorPosition(_, _))) => return Ok(None),
            Ok(None) => Ok(None),
            Err(e) => return Err(e),
        }
    }
}
