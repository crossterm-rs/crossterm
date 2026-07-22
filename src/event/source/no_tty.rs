use std::{collections::VecDeque, io, time::Duration};

use bytes::Bytes;
use tokio::sync::mpsc::Receiver;

use crate::event::{internal::InternalEvent, sys::unix::parse::parse_event, timeout::PollTimeout};

pub struct NoTtyInternalEventSource {
    parser: Parser,
    recv: Receiver<Bytes>,
}

impl NoTtyInternalEventSource {
    pub fn new(recv: Receiver<Bytes>) -> io::Result<Self> {
        Ok(NoTtyInternalEventSource {
            parser: Parser::default(),
            recv,
        })
    }

    /// Tries to read an `InternalEvent` within the given duration.
    ///
    /// Returns `Ok(None)` if the timeout expires before an event is available, and an
    /// error with kind [`io::ErrorKind::BrokenPipe`] if the input channel's sender has
    /// been dropped.
    pub(crate) async fn try_read(
        &mut self,
        timeout: Option<Duration>,
    ) -> io::Result<Option<InternalEvent>> {
        if let Some(event) = self.parser.next() {
            return Ok(Some(event));
        }

        let timeout = PollTimeout::new(timeout);

        loop {
            let t = timeout
                .leftover()
                .unwrap_or(std::time::Duration::from_secs(u64::MAX));
            let data = match tokio::time::timeout(t, self.recv.recv()).await {
                Ok(Some(d)) => d,
                // Sender dropped: signal a broken pipe so callers can bail out.
                Ok(None) => {
                    return Err(io::Error::new(
                        io::ErrorKind::BrokenPipe,
                        "no-tty input channel disconnected",
                    ))
                }
                // Timed out waiting for data.
                Err(_elapsed) => return Ok(None),
            };

            if data.is_empty() {
                return Ok(None);
            }
            self.parser.advance(&data, false);

            if let Some(event) = self.parser.next() {
                return Ok(Some(event));
            }

            // Processing above can take some time, check if timeout expired
            if timeout.elapsed() {
                return Ok(None);
            }
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
}

impl Iterator for Parser {
    type Item = InternalEvent;

    fn next(&mut self) -> Option<Self::Item> {
        self.internal_events.pop_front()
    }
}
