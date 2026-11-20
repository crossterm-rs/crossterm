use std::{collections::VecDeque, io, time::Duration};
#[cfg(feature = "event-stream")]
use std::{
    io::Read,
    os::{fd::AsRawFd, unix::net::UnixStream},
};

use mio::{Events, Interest, Poll, Token, unix::SourceFd};
use signal_hook_mio::v1_0::Signals;

#[cfg(feature = "event-stream")]
use crate::event::sys::Waker;
use crate::event::{
    Event, internal::InternalEvent, source::EventSource, sys::unix::parse::parse_event,
    timeout::PollTimeout,
};
use crate::terminal::sys::file_descriptor::{FileDesc, tty_fd};

// Tokens to identify file descriptor
const TTY_TOKEN: Token = Token(0);
const SIGNAL_TOKEN: Token = Token(1);
#[cfg(feature = "event-stream")]
const WAKE_TOKEN: Token = Token(2);

// I (@zrzka) wasn't able to read more than 1_022 bytes when testing
// reading on macOS/Linux -> we don't need bigger buffer and 1k of bytes
// is enough.
const TTY_BUFFER_SIZE: usize = 1_024;

pub(crate) struct UnixInternalEventSource {
    poll: Poll,
    events: Events,
    next_event: usize,
    tty_needs_refresh: bool,
    parser: Parser,
    tty_buffer: [u8; TTY_BUFFER_SIZE],
    tty_fd: FileDesc<'static>,
    signals: Signals,
    #[cfg(feature = "event-stream")]
    waker: Waker,
    #[cfg(feature = "event-stream")]
    wake_receiver: UnixStream,
}

impl UnixInternalEventSource {
    pub fn new() -> io::Result<Self> {
        UnixInternalEventSource::from_file_descriptor(tty_fd()?)
    }

    pub(crate) fn from_file_descriptor(input_fd: FileDesc<'static>) -> io::Result<Self> {
        let poll = Poll::new()?;
        let registry = poll.registry();

        let tty_raw_fd = input_fd.raw_fd();
        let mut tty_ev = SourceFd(&tty_raw_fd);
        registry.register(&mut tty_ev, TTY_TOKEN, Interest::READABLE)?;

        let mut signals = Signals::new([signal_hook::consts::SIGWINCH])?;
        registry.register(&mut signals, SIGNAL_TOKEN, Interest::READABLE)?;

        #[cfg(feature = "event-stream")]
        let (waker, wake_receiver) = Waker::new(registry, WAKE_TOKEN)?;

        Ok(UnixInternalEventSource {
            poll,
            events: Events::with_capacity(3),
            next_event: 0,
            tty_needs_refresh: false,
            parser: Parser::default(),
            tty_buffer: [0u8; TTY_BUFFER_SIZE],
            tty_fd: input_fd,
            signals,
            #[cfg(feature = "event-stream")]
            waker,
            #[cfg(feature = "event-stream")]
            wake_receiver,
        })
    }
}

impl EventSource for UnixInternalEventSource {
    fn try_read(&mut self, timeout: Option<Duration>) -> io::Result<Option<InternalEvent>> {
        let immediate = timeout == Some(Duration::ZERO);
        let timeout = PollTimeout::new(timeout);
        let mut fresh_batch = false;

        loop {
            let mut full_read = false;
            while let Some(token) = self.events.iter().nth(self.next_event).map(|e| e.token()) {
                self.next_event += 1;
                match token {
                    TTY_TOKEN => {
                        // A saved readiness notification cannot authorize a blocking read.
                        if !fresh_batch {
                            continue;
                        }
                        match self.tty_fd.read(&mut self.tty_buffer) {
                            Ok(0) => return Err(io::ErrorKind::UnexpectedEof.into()),
                            Ok(count) => {
                                full_read = count == TTY_BUFFER_SIZE;
                                self.parser.advance(&self.tty_buffer[..count], full_read);
                            }
                            Err(error)
                                if matches!(
                                    error.kind(),
                                    io::ErrorKind::WouldBlock | io::ErrorKind::Interrupted
                                ) => {}
                            Err(error) => return Err(error),
                        }
                        if let Some(event) = self.parser.next() {
                            return Ok(Some(event));
                        }
                    }
                    SIGNAL_TOKEN => {
                        if self.signals.pending().next() == Some(signal_hook::consts::SIGWINCH) {
                            let new_size = crate::terminal::size()?;
                            return Ok(Some(InternalEvent::Event(Event::Resize(
                                new_size.0, new_size.1,
                            ))));
                        }
                    }
                    #[cfg(feature = "event-stream")]
                    WAKE_TOKEN => {
                        match self.wake_receiver.read(&mut [0; 1024]) {
                            Ok(_) => {}
                            Err(error)
                                if matches!(
                                    error.kind(),
                                    io::ErrorKind::WouldBlock | io::ErrorKind::Interrupted
                                ) => {}
                            Err(error) => return Err(error),
                        }
                        self.poll.registry().reregister(
                            &mut SourceFd(&self.wake_receiver.as_raw_fd()),
                            WAKE_TOKEN,
                            Interest::READABLE,
                        )?;
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::Interrupted,
                            "Poll operation was woken up by `Waker::wake`",
                        ));
                    }
                    _ => unreachable!("Synchronize Evented handle registration & token handling"),
                }
            }

            if let Some(event) = self.parser.next() {
                return Ok(Some(event));
            }
            if fresh_batch && timeout.elapsed() && !(immediate && full_read) {
                return Ok(None);
            }

            // Recheck skipped or partially consumed TTY readiness without waiting for a new edge.
            if self.tty_needs_refresh {
                let registry = self.poll.registry();
                let fd = self.tty_fd.raw_fd();
                let mut source = SourceFd(&fd);
                match registry.reregister(&mut source, TTY_TOKEN, Interest::READABLE) {
                    // Mio's poll(2) backend removes the registration on hangup.
                    Err(error) if error.kind() == io::ErrorKind::NotFound => {
                        registry.register(&mut source, TTY_TOKEN, Interest::READABLE)?;
                    }
                    result => result?,
                }
                self.tty_needs_refresh = false;
            }
            self.events.clear();
            self.next_event = 0;
            match self.poll.poll(&mut self.events, timeout.leftover()) {
                Ok(()) => {}
                Err(error) if error.kind() == io::ErrorKind::Interrupted => {
                    self.events.clear();
                    if timeout.elapsed() {
                        return Ok(None);
                    }
                    continue;
                }
                Err(error) => {
                    self.events.clear();
                    return Err(error);
                }
            }
            if self.events.is_empty() {
                return Ok(None);
            }
            fresh_batch = true;
            self.tty_needs_refresh = self.events.iter().any(|event| event.token() == TTY_TOKEN);
        }
    }

    #[cfg(feature = "event-stream")]
    fn waker(&self) -> Waker {
        self.waker.clone()
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
                    // Event can't be parsed (not enough parameters, parameter is not a number,
                    // ...). Clear the buffer and continue with another
                    // sequence.
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

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use std::{io::Write, os::unix::net::UnixStream};

    use super::*;
    use crate::event::{KeyCode, KeyEvent, KeyModifiers};

    #[cfg(feature = "event-stream")]
    #[test]
    fn wake_before_input_does_not_strand_the_original_byte() {
        let (mut sender, receiver) = UnixStream::pair().unwrap();
        let mut source =
            UnixInternalEventSource::from_file_descriptor(FileDesc::Owned(receiver.into()))
                .unwrap();
        source.waker().wake().unwrap();
        sender.write_all(b"x").unwrap();

        let wake = source.try_read(Some(Duration::ZERO)).unwrap_err();
        assert_eq!(wake.kind(), io::ErrorKind::Interrupted);
        assert_eq!(
            source
                .events
                .iter()
                .map(|event| event.token())
                .collect::<Vec<_>>(),
            [WAKE_TOKEN, TTY_TOKEN],
            "the Linux test must exercise one wake-first readiness batch",
        );
        assert_eq!(
            source.try_read(Some(Duration::ZERO)).unwrap(),
            Some(InternalEvent::Event(Event::Key(KeyEvent::new(
                KeyCode::Char('x'),
                KeyModifiers::NONE,
            )))),
        );
        assert_eq!(source.try_read(Some(Duration::ZERO)).unwrap(), None);
    }

    #[test]
    fn incomplete_input_returns_to_poll_and_keeps_the_prefix() {
        let (mut sender, receiver) = UnixStream::pair().unwrap();
        receiver
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        let mut source =
            UnixInternalEventSource::from_file_descriptor(FileDesc::Owned(receiver.into()))
                .unwrap();
        sender.write_all(b"\x1b[").unwrap();

        let start = std::time::Instant::now();
        assert_eq!(
            source.try_read(Some(Duration::from_millis(20))).unwrap(),
            None
        );
        assert!(
            start.elapsed() < Duration::from_secs(3),
            "read blocked after a partial CSI"
        );

        sender.write_all(b"A").unwrap();
        assert_eq!(
            source.try_read(Some(Duration::ZERO)).unwrap(),
            Some(InternalEvent::Event(Event::Key(KeyEvent::new(
                KeyCode::Up,
                KeyModifiers::NONE,
            )))),
        );
    }

    #[test]
    fn hangup_does_not_hide_the_tail_of_a_full_read() {
        let (mut sender, receiver) = UnixStream::pair().unwrap();
        let mut source =
            UnixInternalEventSource::from_file_descriptor(FileDesc::Owned(receiver.into()))
                .unwrap();
        let mut input = [b'x'; TTY_BUFFER_SIZE + 1];
        input[TTY_BUFFER_SIZE] = b'y';
        sender.write_all(&input).unwrap();
        drop(sender);

        for expected in input {
            assert_eq!(
                source.try_read(Some(Duration::ZERO)).unwrap(),
                Some(InternalEvent::Event(Event::Key(KeyEvent::new(
                    KeyCode::Char(char::from(expected)),
                    KeyModifiers::NONE,
                )))),
            );
        }
        let eof = source.try_read(Some(Duration::ZERO)).unwrap_err();
        assert_eq!(eof.kind(), io::ErrorKind::UnexpectedEof);
    }

    #[cfg(feature = "event-stream")]
    #[test]
    fn wake_survives_refresh_of_drained_input() {
        let (mut sender, receiver) = UnixStream::pair().unwrap();
        let mut source =
            UnixInternalEventSource::from_file_descriptor(FileDesc::Owned(receiver.into()))
                .unwrap();
        sender.write_all(b"x").unwrap();
        assert!(source.try_read(Some(Duration::ZERO)).unwrap().is_some());
        source.waker().wake().unwrap();
        let error = source.try_read(Some(Duration::ZERO)).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::Interrupted);
    }

    #[cfg(feature = "bracketed-paste")]
    #[test]
    fn zero_timeout_reads_a_complete_queued_paste() {
        let (mut sender, receiver) = UnixStream::pair().unwrap();
        let mut source =
            UnixInternalEventSource::from_file_descriptor(FileDesc::Owned(receiver.into()))
                .unwrap();
        let text = "x".repeat(TTY_BUFFER_SIZE);
        write!(sender, "\x1b[200~{text}\x1b[201~").unwrap();
        assert_eq!(
            source.try_read(Some(Duration::ZERO)).unwrap(),
            Some(InternalEvent::Event(Event::Paste(text))),
        );
    }
}
