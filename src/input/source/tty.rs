use std::time::Duration;

use mio::{unix::EventedFd, Events, Poll, PollOpt, Ready, Token};

use crate::Result;

use super::super::{
    events::InternalEvent,
    source::EventSource,
    sys::unix::{parse_event, tty_fd, FileDesc},
    timeout::PollTimeout,
};

// Tokens to identify file descriptor
const TTY_TOKEN: Token = Token(0);

pub(crate) struct TtyInternalEventSource {
    buffer: Vec<u8>,
    poll: Poll,
    tty_fd: FileDesc,
    events: Events,
}

impl TtyInternalEventSource {
    pub fn new() -> Result<TtyInternalEventSource> {
        Ok(TtyInternalEventSource::from_file_descriptor(tty_fd()?))
    }

    pub(crate) fn from_file_descriptor(input_fd: FileDesc) -> TtyInternalEventSource {
        let buffer = Vec::new();

        // Get raw file descriptors for
        let tty_raw_fd = input_fd.raw_fd();

        // Setup polling with raw file descriptors
        let tty_ev = EventedFd(&tty_raw_fd);

        let poll = Poll::new().unwrap();
        poll.register(&tty_ev, TTY_TOKEN, Ready::readable(), PollOpt::level())
            .unwrap();

        TtyInternalEventSource {
            buffer,
            poll,
            tty_fd: input_fd,
            events: Events::with_capacity(2),
        }
    }
}

impl EventSource for TtyInternalEventSource {
    fn try_read(&mut self, timeout: Option<Duration>) -> Result<Option<InternalEvent>> {
        let timeout = PollTimeout::new(timeout);

        loop {
            match self.poll.poll(&mut self.events, timeout.leftover())? {
                event_count if event_count > 0 => {
                    self.buffer.push(self.tty_fd.read_byte()?);

                    let input_available = self
                        .poll
                        .poll(&mut self.events, Some(Duration::from_secs(0)))
                        .map(|x| x > 0)?;

                    match parse_event(&self.buffer, input_available) {
                        Ok(None) => {
                            // Not enough bytes to construct an InternalEvent
                        }
                        Ok(Some(ie)) => {
                            self.buffer.clear();
                            return Ok(Some(ie));
                        }
                        Err(_) => {
                            // Can't parse an event, clear buffer and start over
                            self.buffer.clear();
                        }
                    };
                }
                _ => return Ok(None),
            };

            if timeout.elapsed() {
                return Ok(None);
            }
        }
    }
}
