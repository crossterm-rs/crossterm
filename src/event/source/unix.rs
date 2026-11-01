#[cfg(feature = "use-dev-tty")]
pub(crate) mod tty;

#[cfg(not(feature = "use-dev-tty"))]
pub(crate) mod mio;

#[cfg(feature = "use-dev-tty")]
pub(crate) use self::tty::UnixInternalEventSource;

#[cfg(not(feature = "use-dev-tty"))]
pub(crate) use self::mio::UnixInternalEventSource;

/// Recognize bounded OSC 7501 replies only while an explicit query is active.
#[cfg(feature = "osc7501")]
#[derive(Debug, Default)]
pub(super) struct OscParser {
    pub(super) querying: bool,
}

#[cfg(feature = "osc7501")]
impl OscParser {
    pub(super) fn set_query(
        &mut self,
        active: bool,
        buffer: &mut Vec<u8>,
        events: &mut std::collections::VecDeque<crate::event::internal::InternalEvent>,
    ) {
        self.querying = active;
        if !active && (buffer == b"\x1b" || buffer.starts_with(b"\x1b]")) {
            Self::replay(buffer, events, false);
        }
    }

    pub(super) fn advance(
        &mut self,
        buffer: &mut Vec<u8>,
        events: &mut std::collections::VecDeque<crate::event::internal::InternalEvent>,
    ) -> bool {
        const PREFIX: &[u8] = b"\x1b]7501;?";
        if !self.querying || !buffer.starts_with(b"\x1b]") {
            return false;
        }
        if buffer.len() <= PREFIX.len() && PREFIX.starts_with(buffer) {
            return true;
        }
        if buffer.starts_with(PREFIX) && buffer.len() <= 4096 {
            let tail = &buffer[PREFIX.len()..];
            let byte = *buffer.last().unwrap();
            let escaped = buffer[buffer.len() - 2] == b'\x1b';
            let valid_tail =
                tail[0] == b':' || tail == b"\x07" || tail == b"\x1b" || tail == b"\x1b\\";
            if valid_tail && ((byte == b'\x07' && !escaped) || (escaped && byte == b'\\')) {
                events.push_back(crate::event::internal::InternalEvent::ProgramStatusSupport);
                buffer.clear();
                return true;
            }
            if valid_tail && !escaped && (byte == b'\x1b' || (byte >= b' ' && byte != b'\x7f')) {
                return true;
            }
        }
        // Not an acknowledgement (or too large). Decode every buffered byte as
        // ordinary input, rather than dropping Alt+] and its following keystrokes.
        Self::replay(buffer, events, true);
        true
    }

    fn replay(
        buffer: &mut Vec<u8>,
        events: &mut std::collections::VecDeque<crate::event::internal::InternalEvent>,
        input_available: bool,
    ) {
        let pending = std::mem::take(buffer);
        for (index, byte) in pending.iter().enumerate() {
            buffer.push(*byte);
            match crate::event::sys::unix::parse::parse_event(
                buffer,
                input_available || index + 1 < pending.len(),
            ) {
                Ok(Some(event)) => {
                    events.push_back(event);
                    buffer.clear();
                }
                Ok(None) => {}
                Err(_) => buffer.clear(),
            }
        }
    }
}

#[cfg(all(test, feature = "osc7501"))]
mod tests {
    use super::UnixInternalEventSource;
    use crate::event::{Event, KeyCode, internal::InternalEvent, source::EventSource};
    use crate::terminal::sys::file_descriptor::FileDesc;
    use std::{io::Write, os::unix::net::UnixStream, time::Duration};

    #[test]
    fn alt_bracket_and_following_keys_survive_inside_and_outside_queries() {
        use crate::event::{KeyEvent, KeyModifiers};
        for active in [false, true] {
            let (receiver, mut sender) = UnixStream::pair().unwrap();
            let mut source =
                UnixInternalEventSource::from_file_descriptor(FileDesc::Owned(receiver.into()))
                    .unwrap();
            source.set_program_status_query(active);
            sender.write_all(b"\x1b]abc").unwrap();
            for event in [
                KeyEvent::new(KeyCode::Char(']'), KeyModifiers::ALT),
                KeyCode::Char('a').into(),
                KeyCode::Char('b').into(),
                KeyCode::Char('c').into(),
            ] {
                assert_eq!(
                    source.try_read(Some(Duration::from_millis(20))).unwrap(),
                    Some(InternalEvent::Event(Event::Key(event)))
                );
            }
            sender.write_all("\x1b]75\x1b[Aé".as_bytes()).unwrap();
            for event in [
                KeyEvent::new(KeyCode::Char(']'), KeyModifiers::ALT),
                KeyCode::Char('7').into(),
                KeyCode::Char('5').into(),
                KeyCode::Up.into(),
                KeyCode::Char('é').into(),
            ] {
                assert_eq!(
                    source.try_read(Some(Duration::from_millis(20))).unwrap(),
                    Some(InternalEvent::Event(Event::Key(event)))
                );
            }
            source.set_program_status_query(false);
        }
    }

    #[test]
    fn osc_replies_are_internal_and_preserve_following_input_across_reads() {
        for reply in [
            b"\x1b]7501;?\x07".as_slice(),
            b"\x1b]7501;?\x1b\\",
            b"\x1b]7501;?:states=idle,working:kinds=auth:future=value\x1b\\",
        ] {
            // Every split, including an isolated initial ESC and split ST, must
            // survive separate reads while a capability query is active.
            for split in 1..reply.len() {
                // Like stdin, leave the FD blocking to exercise the timeout contract.
                let (receiver, mut sender) = UnixStream::pair().unwrap();
                let mut source =
                    UnixInternalEventSource::from_file_descriptor(FileDesc::Owned(receiver.into()))
                        .unwrap();
                source.set_program_status_query(true);
                sender.write_all(&reply[..split]).unwrap();
                assert_eq!(
                    source.try_read(Some(Duration::from_millis(1))).unwrap(),
                    None
                );
                sender.write_all(&reply[split..]).unwrap();
                sender.write_all(b"a\x1b[?1;2cb").unwrap();
                for event in [
                    InternalEvent::ProgramStatusSupport,
                    InternalEvent::Event(Event::Key(KeyCode::Char('a').into())),
                    InternalEvent::PrimaryDeviceAttributes,
                    InternalEvent::Event(Event::Key(KeyCode::Char('b').into())),
                ] {
                    assert_eq!(
                        source.try_read(Some(Duration::from_millis(100))).unwrap(),
                        Some(event)
                    );
                }
                source.set_program_status_query(false);
            }
        }
    }

    #[test]
    fn incomplete_and_non_reply_input_is_replayed_when_the_query_ends() {
        use crate::event::{KeyEvent, KeyModifiers};
        for input in [
            b"\x1b".as_slice(),
            b"\x1b]",
            b"\x1b]75",
            b"\x1b]7501;?",
            b"\x1b]7501;?no",
            b"\x1b]52",
        ] {
            let (receiver, mut sender) = UnixStream::pair().unwrap();
            let mut source =
                UnixInternalEventSource::from_file_descriptor(FileDesc::Owned(receiver.into()))
                    .unwrap();
            source.set_program_status_query(true);
            sender.write_all(input).unwrap();
            let mut actual = Vec::new();
            if let Some(event) = source.try_read(Some(Duration::from_millis(1))).unwrap() {
                actual.push(event);
            }
            source.set_program_status_query(false);
            sender.write_all(b"q").unwrap();
            while let Some(event) = source.try_read(Some(Duration::from_millis(1))).unwrap() {
                actual.push(event);
            }
            let mut expected = if input.len() == 1 {
                vec![InternalEvent::Event(Event::Key(KeyCode::Esc.into()))]
            } else {
                let mut keys = vec![InternalEvent::Event(Event::Key(KeyEvent::new(
                    KeyCode::Char(']'),
                    KeyModifiers::ALT,
                )))];
                keys.extend(input[2..].iter().map(|byte| {
                    InternalEvent::Event(Event::Key(KeyCode::Char(*byte as char).into()))
                }));
                keys
            };
            expected.push(InternalEvent::Event(Event::Key(KeyCode::Char('q').into())));
            assert_eq!(actual, expected, "{input:?}");
        }
    }

    #[test]
    fn reply_size_boundary_preserves_oversized_input() {
        use crate::event::{KeyEvent, KeyModifiers};
        for length in [4096, 4097] {
            let (receiver, mut sender) = UnixStream::pair().unwrap();
            let mut source =
                UnixInternalEventSource::from_file_descriptor(FileDesc::Owned(receiver.into()))
                    .unwrap();
            source.set_program_status_query(true);
            let mut input = b"\x1b]7501;?:x=".to_vec();
            input.resize(length - 1, b'a');
            input.push(b'\x07');
            let mut actual = Vec::new();
            for chunk in input.chunks(512) {
                sender.write_all(chunk).unwrap();
                if let Some(event) = source.try_read(Some(Duration::from_millis(1))).unwrap() {
                    actual.push(event);
                }
            }
            source.set_program_status_query(false);
            while let Some(event) = source.try_read(Some(Duration::from_millis(1))).unwrap() {
                actual.push(event);
            }
            let expected = if length == 4096 {
                vec![InternalEvent::ProgramStatusSupport]
            } else {
                let mut keys = vec![InternalEvent::Event(Event::Key(KeyEvent::new(
                    KeyCode::Char(']'),
                    KeyModifiers::ALT,
                )))];
                keys.extend(input[2..length - 1].iter().map(|byte| {
                    InternalEvent::Event(Event::Key(KeyCode::Char(*byte as char).into()))
                }));
                keys.push(InternalEvent::Event(Event::Key(KeyEvent::new(
                    KeyCode::Char('g'),
                    KeyModifiers::CONTROL,
                ))));
                keys
            };
            assert_eq!(actual, expected);
        }
    }
}
