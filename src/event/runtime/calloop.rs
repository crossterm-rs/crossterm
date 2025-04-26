#[cfg(not(feature = "libc"))]
use std::os::fd::AsFd;
#[cfg(feature = "libc")]
use std::os::fd::AsRawFd;
use std::{collections::VecDeque, io, os::unix::net::UnixStream};

use calloop::{generic::Generic, EventSource, Interest, Mode, Poll, PostAction, TokenFactory};
use signal_hook::low_level::pipe;

use crate::{
    event::{sys::unix::parse::parse_event, Event, InternalEvent},
    terminal::sys::file_descriptor::{tty_fd, FileDesc},
};

// I (@zrzka) wasn't able to read more than 1_022 bytes when testing
// reading on macOS/Linux -> we don't need bigger buffer and 1k of bytes
// is enough.
const TTY_BUFFER_SIZE: usize = 1_024;

fn nonblocking_unix_pair() -> io::Result<(UnixStream, UnixStream)> {
    let (receiver, sender) = UnixStream::pair()?;
    receiver.set_nonblocking(true)?;
    sender.set_nonblocking(true)?;
    Ok((receiver, sender))
}

pub struct UnixInternalEventSource {
    parser: Parser,
    tty_buffer: [u8; TTY_BUFFER_SIZE],
    #[cfg(not(feature = "libc"))]
    tty_source: Generic<FileDesc<'static>>,
    #[cfg(feature = "libc")]
    tty_source: Generic<calloop::generic::FdWrapper<FileDesc<'static>>>,
    sig_source: Generic<UnixStream>,
}

impl UnixInternalEventSource {
    pub fn new() -> io::Result<Self> {
        Ok(UnixInternalEventSource {
            parser: Parser::default(),
            tty_buffer: [0u8; TTY_BUFFER_SIZE],
            tty_source: {
                let fd = {
                    #[cfg(feature = "libc")]
                    unsafe {
                        calloop::generic::FdWrapper::new(tty_fd()?)
                    }
                    #[cfg(not(feature = "libc"))]
                    tty_fd()?
                };
                Generic::new(fd, Interest::READ, Mode::Edge)
            },
            sig_source: {
                let (receiver, sender) = nonblocking_unix_pair()?;
                #[cfg(feature = "libc")]
                pipe::register(libc::SIGWINCH, sender)?;
                #[cfg(not(feature = "libc"))]
                pipe::register(rustix::process::Signal::WINCH.as_raw(), sender)?;
                Generic::new(receiver, Interest::READ, Mode::Edge)
            },
        })
    }
}

impl EventSource for UnixInternalEventSource {
    type Event = Vec<Event>;
    type Metadata = ();
    type Ret = io::Result<()>;
    type Error = io::Error;

    fn register(
        &mut self,
        poll: &mut Poll,
        factory: &mut TokenFactory,
    ) -> Result<(), calloop::Error> {
        self.tty_source.register(poll, factory)?;
        self.sig_source.register(poll, factory)?;
        Ok(())
    }

    fn reregister(
        &mut self,
        poll: &mut Poll,
        factory: &mut TokenFactory,
    ) -> Result<(), calloop::Error> {
        self.tty_source.reregister(poll, factory)?;
        self.sig_source.reregister(poll, factory)?;
        Ok(())
    }

    fn unregister(&mut self, poll: &mut Poll) -> Result<(), calloop::Error> {
        self.tty_source.unregister(poll)?;
        self.sig_source.unregister(poll)?;
        Ok(())
    }

    fn process_events<F>(
        &mut self,
        readiness: calloop::Readiness,
        token: calloop::Token,
        mut callback: F,
    ) -> Result<PostAction, Self::Error>
    where
        F: FnMut(Self::Event, &mut Self::Metadata) -> Self::Ret,
    {
        self.tty_source.process_events(readiness, token, |_, f| {
            loop {
                let read_count = read_complete(f, &mut self.tty_buffer)?;
                if read_count > 0 {
                    self.parser.advance(
                        &self.tty_buffer[..read_count],
                        read_count == TTY_BUFFER_SIZE,
                    );
                }

                if !self.parser.internal_events.is_empty() {
                    break;
                }

                if read_count == 0 {
                    break;
                }
            }

            Ok(calloop::PostAction::Continue)
        })?;

        self.sig_source.process_events(readiness, token, |_, f| {
            #[cfg(feature = "libc")]
            let fd = FileDesc::new(f.as_raw_fd(), false);
            #[cfg(not(feature = "libc"))]
            let fd = FileDesc::Borrowed(f.as_fd());

            // drain the pipe
            while read_complete(&fd, &mut [0; 1024])? != 0 {}

            let new_size = crate::terminal::size()?;
            self.parser
                .internal_events
                .push_back(InternalEvent::Event(Event::Resize(new_size.0, new_size.1)));

            Ok(calloop::PostAction::Continue)
        })?;

        if !self.parser.internal_events.is_empty() {
            let public_events: Self::Event = self
                .parser
                .take_events()
                .into_iter()
                .filter_map(|e| match e {
                    InternalEvent::Event(event) => Some(event),
                    _ => None,
                })
                .collect();
            if !public_events.is_empty() {
                callback(public_events, &mut ()).unwrap();
            }
        };

        Ok(PostAction::Continue)
    }
}

/// read_complete reads from a non-blocking file descriptor
/// until the buffer is full or it would block.
///
/// Similar to `std::io::Read::read_to_end`, except this function
/// only fills the given buffer and does not read beyond that.
fn read_complete(fd: &FileDesc, buf: &mut [u8]) -> io::Result<usize> {
    loop {
        match fd.read(buf) {
            Ok(x) => return Ok(x),
            Err(e) => match e.kind() {
                io::ErrorKind::WouldBlock => return Ok(0),
                io::ErrorKind::Interrupted => continue,
                _ => return Err(e),
            },
        }
    }
}

//
// Following `Parser` structure exists for two reasons:
//
//  * mimic anes Parser interface
//  * move the advancing, parsing, ... stuff out of the `try_read` method
//
#[derive(Debug)]
struct Parser {
    buffer: Vec<u8>,
    internal_events: VecDeque<InternalEvent>,
}

impl Default for Parser {
    fn default() -> Self {
        Parser {
            // This buffer is used for -> 1 <- ANSI escape sequence. Are we
            // aware of any ANSI escape sequence that is bigger? Can we make
            // it smaller?
            //
            // Probably not worth spending more time on this as "there's a plan"
            // to use the anes crate parser.
            buffer: Vec::with_capacity(256),
            // TTY_BUFFER_SIZE is 1_024 bytes. How many ANSI escape sequences can
            // fit? What is an average sequence length? Let's guess here
            // and say that the average ANSI escape sequence length is 8 bytes. Thus
            // the buffer size should be 1024/8=128 to avoid additional allocations
            // when processing large amounts of data.
            //
            // There's no need to make it bigger, because when you look at the `try_read`
            // method implementation, all events are consumed before the next TTY_BUFFER
            // is processed -> events pushed.
            internal_events: VecDeque::with_capacity(128),
        }
    }
}

impl Parser {
    fn advance(&mut self, buffer: &[u8], more: bool) {
        for (idx, byte) in buffer.iter().enumerate() {
            let more = idx + 1 < buffer.len() || more;

            self.buffer.push(*byte);

            match parse_event(&self.buffer, more) {
                Ok(Some(ie)) => {
                    self.internal_events.push_back(ie);
                    self.buffer.clear();
                }
                Ok(None) => {
                    // Event can't be parsed, because we don't have enough bytes for
                    // the current sequence. Keep the buffer and process next bytes.
                }
                Err(_) => {
                    // Event can't be parsed (not enough parameters, parameter is not a number, ...).
                    // Clear the buffer and continue with another sequence.
                    self.buffer.clear();
                }
            }
        }
    }
    fn take_events(&mut self) -> VecDeque<InternalEvent> {
        let mut es = std::mem::replace(&mut self.internal_events, VecDeque::with_capacity(128));
        es.shrink_to_fit();
        es
    }
}
