use std::io;
use std::time::Duration;

use mio::{unix::EventedFd, Events, Poll, PollOpt, Ready, Token};
use signal_hook::iterator::Signals;

use crate::Result;

use super::super::{
    source::EventSource,
    sys::unix::{parse_event, tty_fd, FileDesc},
    timeout::PollTimeout,
    Event, InternalEvent,
};

// Tokens to identify file descriptor
const TTY_TOKEN: Token = Token(0);
const SIGNAL_TOKEN: Token = Token(1);
const WAKE_TOKEN: Token = Token(3);

/// Creates a new pipe and returns `(read, write)` file descriptors.
fn pipe() -> Result<(FileDesc, FileDesc)> {
    let (read_fd, write_fd) = unsafe {
        let mut pipe_fds: [libc::c_int; 2] = [0; 2];
        if libc::pipe(pipe_fds.as_mut_ptr()) == -1 {
            return Err(io::Error::last_os_error().into());
        }
        (pipe_fds[0], pipe_fds[1])
    };

    let read_fd = FileDesc::new(read_fd, true);
    let write_fd = FileDesc::new(write_fd, true);

    Ok((read_fd, write_fd))
}

pub(crate) struct TtyInternalEventSource {
    buffer: Vec<u8>,
    poll: Poll,
    tty_fd: FileDesc,
    events: Events,
    _signals: Signals,
    wake_read_fd: FileDesc,
    wake_write_fd: FileDesc,
}

impl TtyInternalEventSource {
    pub fn new() -> Result<Self> {
        Ok(TtyInternalEventSource::from_file_descriptor(tty_fd()?)?)
    }

    pub(crate) fn from_file_descriptor(input_fd: FileDesc) -> Result<Self> {
        // Get raw file descriptors for
        let tty_raw_fd = input_fd.raw_fd();

        // Setup polling with raw file descriptors
        let tty_ev = EventedFd(&tty_raw_fd);

        // Wake self pipe
        let (wake_read_fd, wake_write_fd) = pipe()?;
        let wake_read_raw_fd = wake_read_fd.raw_fd();
        let wake_read_ev = EventedFd(&wake_read_raw_fd);

        let poll = Poll::new()?;

        let signals = Signals::new(&[signal_hook::SIGWINCH])?;

        // Register tty reader
        poll.register(&tty_ev, TTY_TOKEN, Ready::readable(), PollOpt::level())?;

        // Register signals
        poll.register(&signals, SIGNAL_TOKEN, Ready::readable(), PollOpt::edge())?;

        poll.register(
            &wake_read_ev,
            WAKE_TOKEN,
            Ready::readable(),
            PollOpt::edge(),
        )?;

        Ok(TtyInternalEventSource {
            buffer: Vec::new(),
            poll,
            tty_fd: input_fd,
            events: Events::with_capacity(2),
            _signals: signals,
            wake_read_fd,
            wake_write_fd,
        })
    }
}

impl EventSource for TtyInternalEventSource {
    fn try_read(&mut self, timeout: Option<Duration>) -> Result<Option<InternalEvent>> {
        let mut timeout = PollTimeout::new(timeout);

        loop {
            let event_count = self.poll.poll(&mut self.events, timeout.leftover())?;

            match event_count {
                event_count if event_count > 0 => {
                    let events_count = self
                        .events
                        .iter()
                        .map(|x| x.token())
                        .collect::<Vec<Token>>();

                    for event in events_count {
                        match event {
                            TTY_TOKEN => {
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
                            SIGNAL_TOKEN => {
                                let new_size = crate::terminal::size()?;
                                return Ok(Some(InternalEvent::Event(Event::Resize(
                                    new_size.0, new_size.1,
                                ))));
                            }
                            WAKE_TOKEN => {
                                let _ = self.wake_read_fd.read_byte();
                                return Ok(None);
                            }
                            _ => {}
                        }
                    }
                }
                _ => return Ok(None),
            };

            if timeout.elapsed() {
                return Ok(None);
            }
        }
    }

    fn wake(&self) {
        let _ = self.wake_write_fd.write("W".as_bytes());
    }
}
