use std::io::Read;
use std::{collections::VecDeque, io, os::unix::net::UnixStream, time::Duration};

use signal_hook::low_level::pipe;

use crate::event::timeout::PollTimeout;
use crate::event::Event;
use crate::Result;

mod select;
use self::select::Selector;

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
    EventSource,
};

/// Holds a prototypical Waker and a receiver we can wait on when doing select().
#[cfg(feature = "event-stream")]
struct WakePipe {
    receiver: UnixStream,
    waker: Waker,
}

#[cfg(feature = "event-stream")]
impl WakePipe {
    fn new() -> Result<Self> {
        let (receiver, sender) = UnixStream::pair()?;
        Ok(WakePipe {
            receiver,
            waker: Waker::new(sender),
        })
    }
}

// I (@zrzka) wasn't able to read more than 1_022 bytes when testing
// reading on macOS/Linux -> we don't need bigger buffer and 1k of bytes
// is enough.
const TTY_BUFFER_SIZE: usize = 1_024;

pub(crate) struct UnixInternalEventSource {
    parser: Parser,
    tty_buffer: [u8; TTY_BUFFER_SIZE],
    tty: FileDesc,
    winch_signal_receiver: UnixStream,
    #[cfg(feature = "event-stream")]
    wake_pipe: WakePipe,
}

impl UnixInternalEventSource {
    pub fn new() -> Result<Self> {
        UnixInternalEventSource::from_file_descriptor(tty_fd()?)
    }

    pub(crate) fn from_file_descriptor(input_fd: FileDesc) -> Result<Self> {
        Ok(UnixInternalEventSource {
            parser: Parser::default(),
            tty_buffer: [0u8; TTY_BUFFER_SIZE],
            tty: input_fd,
            winch_signal_receiver: {
                let (receiver, sender) = UnixStream::pair()?;
                // Unregistering is unnecessary because EventSource is a singleton
                pipe::register(libc::SIGWINCH, sender)?;
                receiver
            },
            #[cfg(feature = "event-stream")]
            wake_pipe: WakePipe::new()?,
        })
    }
}

impl EventSource for UnixInternalEventSource {
    fn try_read(&mut self, timeout: Option<Duration>) -> Result<Option<InternalEvent>> {
        let timeout = PollTimeout::new(timeout);
        let mut selector = Selector::default();

        while timeout.leftover().map_or(true, |t| !t.is_zero()) {
            // check if there are buffered events from the last read
            if let Some(event) = self.parser.next() {
                return Ok(Some(event));
            }
            selector.add(&self.tty);
            selector.add(&self.winch_signal_receiver);

            #[cfg(feature = "event-stream")]
            selector.add(&self.wake_pipe.receiver);

            let _ = selector.select(timeout.leftover())?;
            if selector.get(&self.tty).is_some() {
                loop {
                    match self.tty.read(&mut self.tty_buffer, TTY_BUFFER_SIZE) {
                        Ok(read_count) => {
                            if read_count > 0 {
                                self.parser.advance(
                                    &self.tty_buffer[..read_count],
                                    read_count == TTY_BUFFER_SIZE,
                                );
                            }
                        }
                        Err(e) => {
                            match e.kind() {
                                // No more data to read at the moment. We will receive another event
                                // once more data is available to read.
                                io::ErrorKind::WouldBlock => break,
                                io::ErrorKind::Interrupted => continue,
                                _ => return Err(e),
                            }
                        }
                    };

                    if let Some(event) = self.parser.next() {
                        return Ok(Some(event));
                    }
                }
            }
            if selector.get(&self.winch_signal_receiver).is_some() {
                let mut buff = [0];
                if self.winch_signal_receiver.read(&mut buff)? > 0 {
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
            #[cfg(feature = "event-stream")]
            if selector.get(&self.wake_pipe.receiver).is_some() {
                if self.wake_pipe.receiver.read(&mut [0])? > 0 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Interrupted,
                        "Poll operation was woken up by `Waker::wake`",
                    ));
                }
            }
        }
        Ok(None)
    }

    #[cfg(feature = "event-stream")]
    fn waker(&self) -> Waker {
        self.wake_pipe.waker.clone()
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
