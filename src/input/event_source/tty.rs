use std::collections::VecDeque;
use std::process::Output;
use std::time::{Duration, Instant};

use mio::unix::EventedFd;
use mio::{Events, Poll, PollOpt, Ready, Token};

use crate::input::event_poll::{EventPoll, PollTimeOut};
use crate::input::events::InternalEvent;
use crate::input::sys::unix::{parse_event, tty_fd, FileDesc};
use crate::input::{Event, EventSource};
use crate::Result;

// Tokens to identify file descriptor
const TTY_TOKEN: Token = Token(0);

/// Can be used to read `byte`s from the TTY.
/// This a wrapper around `mio::Poll`.
pub(crate) struct TtyBytePoll {
    poll: Poll,
    tty_fd: FileDesc,
    events: Events,
}

impl TtyBytePoll {
    /// Constructs a new instance of `TtyPoll`
    pub(crate) fn new(tty_fd: FileDesc) -> TtyBytePoll {
        // Get raw file descriptors for
        let tty_raw_fd = tty_fd.raw_fd();

        // Setup polling with raw file descriptors
        let tty_ev = EventedFd(&tty_raw_fd);

        let poll = Poll::new().unwrap();
        poll.register(&tty_ev, TTY_TOKEN, Ready::readable(), PollOpt::level())
            .unwrap();

        TtyBytePoll {
            poll,
            tty_fd,
            events: Events::with_capacity(2),
        }
    }
}

impl EventPoll for TtyBytePoll {
    type Output = u8;

    fn poll(&mut self, timeout: Option<Duration>) -> Result<bool> {
        Ok(self.poll.poll(&mut self.events, timeout).map(|x| x > 0)?)
    }

    fn read(&mut self) -> Result<Self::Output> {
        self.tty_fd.read_byte()
    }
}

pub(crate) struct TtyInternalEventSource {
    tty_poll: TtyBytePoll,
    buffer: Vec<u8>,
}

impl TtyInternalEventSource {
    pub fn new() -> Result<TtyInternalEventSource> {
        Ok(TtyInternalEventSource::from_file_descriptor(tty_fd()?))
    }

    pub fn from_file_descriptor(input_fd: FileDesc) -> TtyInternalEventSource {
        let tty_poll = TtyBytePoll::new(input_fd);
        let buffer = Vec::new();

        TtyInternalEventSource { tty_poll, buffer }
    }
}

impl EventSource for TtyInternalEventSource {
    fn try_read(&mut self, timeout: Option<Duration>) -> Result<(bool, Option<InternalEvent>)> {
        let mut poll_timout = PollTimeOut::new(timeout);

        loop {
            match self.tty_poll.poll(poll_timout.left_over())? {
                true => {
                    self.buffer.push(self.tty_poll.read()?);

                    let input_available = self.tty_poll.poll(Some(Duration::from_secs(0)))?;

                    match parse_event(&self.buffer, input_available) {
                        Ok(None) => {
                            // Not enough bytes to construct an InternalEvent
                        }
                        Ok(Some(ie)) => {
                            self.buffer.clear();
                            return Ok((true, Some(ie)));
                        }
                        Err(_) => {
                            // Can't parse an event, clear buffer and start over
                            self.buffer.clear();
                        }
                    };
                }
                false => {
                    return Ok((false, None));
                }
            };

            poll_timout.elapsed();
        }
    }
}
