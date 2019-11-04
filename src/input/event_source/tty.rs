use crate::input::events::InternalEvent;
use crate::input::sys::unix::{tty_fd, FileDesc, TtyPoll};
use crate::input::EventSource;
use crate::Result;
use std::time::Duration;

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
    fn read(&mut self) -> Result<Option<InternalEvent>> {
        self.source.read()
    }

    fn poll(&mut self, timeout: Option<Duration>) -> Result<bool> {
        return self.source.poll(timeout);
    }
}
