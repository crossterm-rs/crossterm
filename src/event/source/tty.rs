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
const WAKE_TOKEN: Token = Token(2);

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
    poll: Poll,
    events: Events,
    tty_buffer: Vec<u8>,
    tty_fd: FileDesc,
    signals: Signals,
    wake_read_fd: FileDesc,
    wake_write_fd: FileDesc,
}

impl TtyInternalEventSource {
    pub fn new() -> Result<Self> {
        Ok(TtyInternalEventSource::from_file_descriptor(tty_fd()?)?)
    }

    pub(crate) fn from_file_descriptor(input_fd: FileDesc) -> Result<Self> {
        let poll = Poll::new()?;

        // PollOpt::level vs PollOpt::edge mio documentation:
        //
        // > With edge-triggered events, operations must be performed on the Evented type until
        // > WouldBlock is returned.
        //
        // TL;DR - DO NOT use PollOpt::edge.
        //
        // Because of the try_read nature (loop with returns) we can't use PollOpt::edge. All
        // Evented handles MUST be registered with the PollOpt::level.
        //
        // If you have to use PollOpt::edge and there's no way how to do it with the PollOpt::level,
        // be aware that the whole TtyInternalEventSource have to be rewritten
        // (read everything from each Evented, process without returns, store all InternalEvent events
        // into a buffer and then return first InternalEvent, etc.). Even these changes wont be
        // enough, because Poll::poll wont fire again until additional Evented event happens and
        // we can still have a buffer filled with InternalEvent events.
        let tty_raw_fd = input_fd.raw_fd();
        let tty_ev = EventedFd(&tty_raw_fd);
        poll.register(&tty_ev, TTY_TOKEN, Ready::readable(), PollOpt::level())?;

        let signals = Signals::new(&[signal_hook::SIGWINCH])?;
        poll.register(&signals, SIGNAL_TOKEN, Ready::readable(), PollOpt::level())?;

        let (wake_read_fd, wake_write_fd) = pipe()?;
        let wake_read_raw_fd = wake_read_fd.raw_fd();
        let wake_read_ev = EventedFd(&wake_read_raw_fd);
        poll.register(
            &wake_read_ev,
            WAKE_TOKEN,
            Ready::readable(),
            PollOpt::level(),
        )?;

        Ok(TtyInternalEventSource {
            poll,
            events: Events::with_capacity(3),
            tty_buffer: Vec::new(),
            tty_fd: input_fd,
            signals,
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
                                self.tty_buffer.push(self.tty_fd.read_byte()?);

                                let input_available = self
                                    .poll
                                    .poll(&mut self.events, Some(Duration::from_secs(0)))
                                    .map(|x| x > 0)?;

                                match parse_event(&self.tty_buffer, input_available) {
                                    Ok(None) => {
                                        // Not enough bytes to construct an InternalEvent
                                    }
                                    Ok(Some(ie)) => {
                                        self.tty_buffer.clear();
                                        return Ok(Some(ie));
                                    }
                                    Err(_) => {
                                        // Can't parse an event, clear buffer and start over
                                        self.tty_buffer.clear();
                                    }
                                };
                            }
                            SIGNAL_TOKEN => {
                                for signal in &self.signals {
                                    match signal as libc::c_int {
                                        signal_hook::SIGWINCH => {
                                            // TODO Should we remove tput?
                                            //
                                            // This can take a really long time, because terminal::size can
                                            // launch new process (tput) and then it parses its output. It's
                                            // not a really long time from the absolute time point of view, but
                                            // it's a really long time from the mio, async-std/tokio executor, ...
                                            // point of view.
                                            let new_size = crate::terminal::size()?;
                                            return Ok(Some(InternalEvent::Event(Event::Resize(
                                                new_size.0, new_size.1,
                                            ))));
                                        }
                                        _ => unreachable!(),
                                    };
                                }
                            }
                            WAKE_TOKEN => {
                                // Something happened on the self pipe. Try to read single byte
                                // (see wake() fn) and ignore result. If we can't read the byte,
                                // mio Poll::poll will fire another event with WAKE_TOKEN.
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
        // DO NOT write more than 1 byte. See try_read & WAKE_TOKEN
        // handling - it reads just 1 byte. If you write more than
        // 1 byte, lets say N, then the try_read will be woken up
        // N times.
        let _ = self.wake_write_fd.write(&[0x57]);
    }
}
