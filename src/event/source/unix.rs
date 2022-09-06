use std::io::Read;
use std::os::unix::prelude::AsRawFd;
use std::{collections::VecDeque, io, os::unix::net::UnixStream, time::Duration};

use signal_hook::consts::SIGWINCH;
use signal_hook::low_level::pipe;

use crate::event::Event;
use crate::Result;

#[cfg(feature = "event-stream")]
use super::super::sys::Waker;
use super::{
    super::{
        sys::unix::{
            file_descriptor::{tty_fd, FileDesc},
            parse::parse_event,
        },
        InternalEvent,
    },
    select::{FdSelector, Selector},
    EventSource,
};

// I (@zrzka) wasn't able to read more than 1_022 bytes when testing
// reading on macOS/Linux -> we don't need bigger buffer and 1k of bytes
// is enough.
const TTY_BUFFER_SIZE: usize = 1_024;

pub(crate) struct UnixInternalEventSource {
    parser: Parser,
    tty_buffer: [u8; TTY_BUFFER_SIZE],
    tty_fd: FileDesc,
    selector: Selector,
    resize_signal: UnixStream,
}

impl UnixInternalEventSource {
    pub fn new() -> Result<Self> {
        UnixInternalEventSource::from_file_descriptor(tty_fd()?)
    }

    pub(crate) fn from_file_descriptor(input_fd: FileDesc) -> Result<Self> {
        let (mut read, write) = UnixStream::pair()?;
        pipe::register(SIGWINCH, write)?;

        Ok(UnixInternalEventSource {
            resize_signal: read,
            parser: Parser::default(),
            tty_buffer: [0u8; TTY_BUFFER_SIZE],
            tty_fd: input_fd,
            selector: Selector::default(),
        })
    }
}

impl EventSource for UnixInternalEventSource {
    fn try_read(&mut self, timeout: Option<Duration>) -> Result<Option<InternalEvent>> {
        let tty_fd = self.tty_fd.raw_fd();
        let resize_fd = self.resize_signal.as_raw_fd();
        self.selector.set(FdSelector::read(tty_fd));
        self.selector.set(FdSelector::read(resize_fd));

        // todo signals and such
        let result = self.selector.select(timeout)?;
        if result == 0 {
            return Ok(None);
        }
        for FdSelector { fd, .. } in self.selector.iter() {
            if fd == tty_fd {
                loop {
                    match self.tty_fd.read(&mut self.tty_buffer, TTY_BUFFER_SIZE) {
                        Ok(read_count) => {
                            if read_count > 0 {
                                self.parser.advance(
                                    &self.tty_buffer[..read_count],
                                    read_count == TTY_BUFFER_SIZE,
                                );
                            }
                        }
                        Err(e) => {
                            // No more data to read at the moment. We will receive another event
                            if e.kind() == io::ErrorKind::WouldBlock {
                                break;
                            }
                            // once more data is available to read.
                            else if e.kind() == io::ErrorKind::Interrupted {
                                continue;
                            }
                        }
                    };

                    if let Some(event) = self.parser.next() {
                        return Ok(Some(event));
                    }
                }
            } else if fd == resize_fd {
                let mut buff = [0];
                if self.resize_signal.read(&mut buff)? > 0 {
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
            }
            // TODO other fds
        }
        Ok(None)
    }

    #[cfg(feature = "event-stream")]
    fn waker(&self) -> Waker {
        todo!()
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
}

impl Iterator for Parser {
    type Item = InternalEvent;

    fn next(&mut self) -> Option<Self::Item> {
        self.internal_events.pop_front()
    }
}
