use std::time::Duration;

use mio::{unix::EventedFd, Events, Poll, PollOpt, Ready, Token};

use crate::{
    input::{
        event_poll::EventPoll,
        events::InternalEvent,
        poll_timer::PollTimer,
        sys::unix::{parse_event, tty_fd, FileDesc},
        EventSource,
    },
    Result,
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
            tty_fd,
            events: Events::with_capacity(2),
        }
    }
}

impl EventSource for TtyInternalEventSource {
    fn try_read(&mut self, timeout: Option<Duration>) -> Result<Option<InternalEvent>> {
        let mut timer = PollTimer::new(timeout);

        loop {
            let poll = |e: &mut Events, timeout: Option<Duration>| {
                self.poll.poll(events, timeout).map(|x| x > 0)?
            };

            match poll(&mut self.events, timer.left_over())? {
                true => {
                    self.buffer.push(self.tty_fd.read_byte()?);

                    let input_available = poll(&mut self.events, Some(Duration::from_secs(0)));

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
                false => {
                    return Ok(None);
                }
            };

            timer.elapsed();
        }
    }
}
