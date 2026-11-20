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
        if let Some(event) = self.parser.next() {
            return Ok(Some(event));
        }

        let immediate = timeout == Some(Duration::ZERO);
        #[cfg(feature = "event-stream")]
        let wakeable = timeout.is_none();
        let timeout = PollTimeout::new(timeout);
        let mut fresh_batch = false;
        let mut retried_at_deadline = false;

        loop {
            let mut read_input = false;
            let mut retry_read = false;
            while let Some(event) = self.events.iter().nth(self.next_event) {
                let token = event.token();
                self.next_event += 1;
                match token {
                    TTY_TOKEN => {
                        // A saved readiness notification cannot authorize a blocking read.
                        if !fresh_batch {
                            continue;
                        }
                        match self.tty_fd.read(&mut self.tty_buffer) {
                            Ok(0) if event.is_read_closed() || event.is_error() => {
                                return Err(io::ErrorKind::UnexpectedEof.into());
                            }
                            Ok(0) => read_input = true,
                            Ok(count) => {
                                read_input = true;
                                self.parser
                                    .advance(&self.tty_buffer[..count], count == TTY_BUFFER_SIZE);
                            }
                            Err(error) if error.kind() == io::ErrorKind::Interrupted => {
                                let elapsed = timeout.elapsed();
                                retry_read = !elapsed || !retried_at_deadline;
                                retried_at_deadline |= elapsed;
                            }
                            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {}
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
                        if wakeable {
                            return Err(io::Error::new(
                                io::ErrorKind::Interrupted,
                                "Poll operation was woken up by `Waker::wake`",
                            ));
                        }
                    }
                    _ => unreachable!("Synchronize Evented handle registration & token handling"),
                }
            }

            if let Some(event) = self.parser.next() {
                return Ok(Some(event));
            }
            if fresh_batch && timeout.elapsed() && !(retry_read || immediate && read_input) {
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
            fresh_batch = false;
            match self.poll.poll(&mut self.events, timeout.leftover()) {
                Ok(()) => {}
                Err(error) if error.kind() == io::ErrorKind::Interrupted => {
                    self.events.clear();
                    if timeout.elapsed() {
                        if retried_at_deadline {
                            return Ok(None);
                        }
                        retried_at_deadline = true;
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
    fn co_ready_wake_and_input_are_both_delivered() {
        let (mut sender, receiver) = UnixStream::pair().unwrap();
        let mut source =
            UnixInternalEventSource::from_file_descriptor(FileDesc::Owned(receiver.into()))
                .unwrap();
        source.waker().wake().unwrap();
        sender.write_all(b"x").unwrap();

        let key = InternalEvent::Event(Event::Key(KeyEvent::new(
            KeyCode::Char('x'),
            KeyModifiers::NONE,
        )));
        let first = source.try_read(None);
        assert!(
            source
                .events
                .iter()
                .any(|event| event.token() == WAKE_TOKEN)
        );
        assert!(source.events.iter().any(|event| event.token() == TTY_TOKEN));
        match first {
            Err(wake) => {
                assert_eq!(wake.kind(), io::ErrorKind::Interrupted);
                assert_eq!(source.try_read(Some(Duration::ZERO)).unwrap(), Some(key));
            }
            Ok(event) => {
                assert_eq!(event, Some(key));
                let wake = source.try_read(None).unwrap_err();
                assert_eq!(wake.kind(), io::ErrorKind::Interrupted);
            }
        }
        assert_eq!(source.try_read(Some(Duration::ZERO)).unwrap(), None);
    }

    #[cfg(feature = "event-stream")]
    #[test]
    fn decoded_cursor_reply_precedes_saved_control_readiness() {
        let (mut sender, receiver) = UnixStream::pair().unwrap();
        let mut source =
            UnixInternalEventSource::from_file_descriptor(FileDesc::Owned(receiver.into()))
                .unwrap();
        source.waker().wake().unwrap();
        source
            .poll
            .poll(&mut source.events, Some(Duration::ZERO))
            .unwrap();
        assert!(
            source
                .events
                .iter()
                .any(|event| event.token() == WAKE_TOKEN)
        );
        source.parser.advance(b"\x1b[3;4R", false);
        assert_eq!(
            source.try_read(Some(Duration::ZERO)).unwrap(),
            Some(InternalEvent::CursorPosition(3, 2)),
        );
        sender.write_all(b"x").unwrap();
        assert_eq!(
            source.try_read(Some(Duration::ZERO)).unwrap(),
            Some(InternalEvent::Event(Event::Key(KeyEvent::new(
                KeyCode::Char('x'),
                KeyModifiers::NONE,
            )))),
        );
    }

    #[cfg(feature = "bracketed-paste")]
    #[test]
    fn canonical_pty_distinguishes_eof_and_reads_short_paste_chunks() {
        use std::{ffi::OsStr, fs::File, os::unix::ffi::OsStrExt, path::Path};

        use rustix::pty::{OpenptFlags, grantpt, openpt, ptsname, unlockpt};

        let master = openpt(OpenptFlags::RDWR).unwrap();
        grantpt(&master).unwrap();
        unlockpt(&master).unwrap();
        let name = ptsname(&master, Vec::new()).unwrap();
        let slave = File::open(Path::new(OsStr::from_bytes(name.to_bytes()))).unwrap();
        rustix::io::ioctl_fionbio(&slave, true).unwrap();
        let mut master = File::from(master);
        let mut source =
            UnixInternalEventSource::from_file_descriptor(FileDesc::Owned(slave.into())).unwrap();

        let paste = b"\x1b[200~x\ny\x1b[201~\n";
        master.write_all(b"\x04").unwrap();
        master.write_all(paste).unwrap();
        let deadline = std::time::Instant::now() + Duration::from_secs(2);
        while rustix::io::ioctl_fionread(&source.tty_fd).unwrap() < paste.len() as u64 {
            assert!(std::time::Instant::now() < deadline, "paste was not queued");
            std::thread::yield_now();
        }
        assert_eq!(
            source.try_read(Some(Duration::ZERO)).unwrap(),
            Some(InternalEvent::Event(Event::Paste("x\ny".into()))),
        );
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
        const CHILD: &str = "CROSSTERM_TEST_WAKE_REFRESH_CHILD";
        if std::env::var_os(CHILD).is_none() {
            let mut child = std::process::Command::new(std::env::current_exe().unwrap())
                .args([
                    "--exact",
                    concat!(module_path!(), "::wake_survives_refresh_of_drained_input")
                        .split_once("::")
                        .unwrap()
                        .1,
                ])
                .env(CHILD, "1")
                .spawn()
                .unwrap();
            let deadline = std::time::Instant::now() + Duration::from_secs(3);
            loop {
                if let Some(status) = child.try_wait().unwrap() {
                    assert!(status.success(), "wake test child failed: {status}");
                    return;
                }
                if std::time::Instant::now() >= deadline {
                    let _ = child.kill();
                    child.wait().unwrap();
                    panic!("wake did not interrupt the blocking read");
                }
                std::thread::sleep(Duration::from_millis(10));
            }
        }

        let (mut sender, receiver) = UnixStream::pair().unwrap();
        let mut source =
            UnixInternalEventSource::from_file_descriptor(FileDesc::Owned(receiver.into()))
                .unwrap();
        sender.write_all(b"x").unwrap();
        assert!(source.try_read(Some(Duration::ZERO)).unwrap().is_some());
        source.waker().wake().unwrap();
        let error = source.try_read(None).unwrap_err();
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
