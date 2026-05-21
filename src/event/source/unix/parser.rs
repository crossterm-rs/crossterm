//! Streaming wrapper around [`parse_event`] used by the unix event sources.
//!
//! [`parse_event`] is stateless: each call sees a single ANSI escape candidate.
//! The [`Parser`] accumulates bytes across `read()` boundaries until a full sequence has been
//! recognised, then emits the resulting [`InternalEvent`]s to its caller.
//!
//! [`Parser`] also enforces [`crate::event::BracketedPasteLimits`], aborting an inflight
//! bracketed paste that exceeds the configured limits.

use std::collections::VecDeque;
#[cfg(feature = "bracketed-paste")]
use std::time::Instant;

use crate::event::internal::InternalEvent;
use crate::event::sys::unix::parse::parse_event;
#[cfg(feature = "bracketed-paste")]
use crate::event::{bracketed_paste_limits, BracketedPasteLimits, Event, PasteAbortReason};

#[cfg(feature = "bracketed-paste")]
const BRACKETED_PASTE_START: &[u8] = b"\x1B[200~";

/// Streams bytes from a tty into a queue of parsed [`InternalEvent`]s.
#[derive(Debug)]
pub(crate) struct Parser {
    /// Bytes belonging to a single in-progress ANSI escape sequence.
    buffer: Vec<u8>,
    /// Events ready for the caller to consume.
    internal_events: VecDeque<InternalEvent>,
    /// Wall-clock instant at which the current bracketed paste began, or `None` if no
    /// bracketed paste is in progress.
    #[cfg(feature = "bracketed-paste")]
    paste_started_at: Option<Instant>,
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
            #[cfg(feature = "bracketed-paste")]
            paste_started_at: None,
        }
    }
}

impl Parser {
    pub(crate) fn advance(&mut self, buffer: &[u8], more: bool) {
        // Hoist the atomics-backed limit read out of the per-byte hot path. Once limits
        // are installed they tend to stay installed for the lifetime of the application,
        // and a single `read()` typically delivers a contiguous burst of bytes.
        #[cfg(feature = "bracketed-paste")]
        let limits = bracketed_paste_limits();
        for (idx, byte) in buffer.iter().enumerate() {
            let more = idx + 1 < buffer.len() || more;

            self.buffer.push(*byte);

            #[cfg(feature = "bracketed-paste")]
            self.update_paste_tracking();

            #[cfg(feature = "bracketed-paste")]
            if self.maybe_abort_paste(limits) {
                continue;
            }

            match parse_event(&self.buffer, more) {
                Ok(Some(ie)) => {
                    self.internal_events.push_back(ie);
                    self.buffer.clear();
                    #[cfg(feature = "bracketed-paste")]
                    {
                        self.paste_started_at = None;
                    }
                }
                Ok(None) => {
                    // Event can't be parsed, because we don't have enough bytes for
                    // the current sequence. Keep the buffer and process next bytes.
                }
                Err(_) => {
                    // Event can't be parsed (not enough parameters, parameter is not a number, ...).
                    // Clear the buffer and continue with another sequence.
                    self.buffer.clear();
                    #[cfg(feature = "bracketed-paste")]
                    {
                        self.paste_started_at = None;
                    }
                }
            }
        }
    }

    /// Records the start instant the moment the bracketed-paste prefix is fully buffered.
    ///
    /// The instant is captured unconditionally (even when no limits are configured) so
    /// that limits installed mid-paste still see the true paste start. This costs one
    /// [`Instant::now`] call per paste, which is negligible compared to the user-driven
    /// cadence of paste events.
    #[cfg(feature = "bracketed-paste")]
    fn update_paste_tracking(&mut self) {
        if self.paste_started_at.is_none()
            && self.buffer.len() == BRACKETED_PASTE_START.len()
            && self.buffer == BRACKETED_PASTE_START
        {
            self.paste_started_at = Some(Instant::now());
        }
    }

    /// If an inflight bracketed paste exceeds `limits`, hand the accumulated buffer off
    /// to the caller as an [`Event::PasteAborted`] and reset the parser state. Returns
    /// `true` when an abort fired, so the caller can skip the normal `parse_event`
    /// dispatch for the byte that triggered it.
    ///
    /// `limits` is taken as a parameter (rather than re-read here) so the atomics behind
    /// [`bracketed_paste_limits`] are read at most once per [`Self::advance`] call rather
    /// than once per byte.
    #[cfg(feature = "bracketed-paste")]
    fn maybe_abort_paste(&mut self, limits: BracketedPasteLimits) -> bool {
        let Some(started_at) = self.paste_started_at else {
            return false;
        };
        debug_assert!(
            self.buffer.starts_with(BRACKETED_PASTE_START),
            "buffer must start with the bracketed-paste prefix whenever paste_started_at is set",
        );
        let reason = if limits
            .max_bytes
            .map_or(false, |cap| self.buffer.len() > cap)
        {
            PasteAbortReason::SizeLimit
        } else if limits
            .max_duration
            .map_or(false, |cap| started_at.elapsed() > cap)
        {
            PasteAbortReason::Timeout
        } else {
            return false;
        };
        // Move the buffered bytes (including the `\x1B[200~` prefix) into the event so the
        // caller can salvage them if desired. The buffer is replaced with a fresh
        // allocation of the original capacity so subsequent escape sequences don't pay
        // reallocation costs.
        let buffered_bytes = std::mem::replace(&mut self.buffer, Vec::with_capacity(256));
        self.paste_started_at = None;
        self.internal_events
            .push_back(InternalEvent::Event(Event::PasteAborted {
                reason,
                buffered_bytes,
            }));
        true
    }
}

impl Iterator for Parser {
    type Item = InternalEvent;

    fn next(&mut self) -> Option<Self::Item> {
        self.internal_events.pop_front()
    }
}

#[cfg(all(test, feature = "bracketed-paste"))]
mod tests {
    use std::time::Duration;

    use parking_lot::{Mutex, MutexGuard};

    use crate::event::{
        bracketed_paste_limits, internal::InternalEvent, set_bracketed_paste_limits,
        BracketedPasteLimits, Event, PasteAbortReason,
    };

    use super::Parser;

    /// The limits are process-global, so tests that touch them must run serially.
    ///
    /// This mutex is taken at the start of every limits-affecting test and held
    /// until the previous limits have been restored.
    static LIMITS_LOCK: Mutex<()> = Mutex::new(());

    struct LimitGuard<'a> {
        previous: BracketedPasteLimits,
        _serialize: MutexGuard<'a, ()>,
    }

    impl LimitGuard<'_> {
        fn new(limits: BracketedPasteLimits) -> Self {
            let serialize = LIMITS_LOCK.lock();
            let previous = bracketed_paste_limits();
            set_bracketed_paste_limits(limits);
            LimitGuard {
                previous,
                _serialize: serialize,
            }
        }
    }

    impl Drop for LimitGuard<'_> {
        fn drop(&mut self) {
            set_bracketed_paste_limits(self.previous);
        }
    }

    fn drain(parser: &mut Parser) -> Vec<InternalEvent> {
        std::iter::from_fn(|| parser.next()).collect()
    }

    #[test]
    fn completes_paste_within_size_limit() {
        let _guard = LimitGuard::new(BracketedPasteLimits {
            max_duration: None,
            max_bytes: Some(1024),
        });
        let mut parser = Parser::default();
        parser.advance(b"\x1B[200~hello\x1B[201~", false);
        assert_eq!(
            drain(&mut parser),
            vec![InternalEvent::Event(Event::Paste("hello".into()))]
        );
    }

    #[test]
    fn aborts_paste_when_size_limit_exceeded() {
        let _guard = LimitGuard::new(BracketedPasteLimits {
            max_duration: None,
            // Six bytes for the prefix plus one byte of payload is the smallest
            // overflow we can observe.
            max_bytes: Some(6),
        });
        let mut parser = Parser::default();
        parser.advance(b"\x1B[200~abcdef", false);
        let events = drain(&mut parser);
        // The first byte past the cap aborts the paste; subsequent bytes parse normally
        // as key events (the parser has dropped its inflight paste state).
        let (first, rest) = events.split_first().expect("at least one event");
        let InternalEvent::Event(Event::PasteAborted {
            reason,
            buffered_bytes,
        }) = first
        else {
            panic!("expected PasteAborted, got {first:?}");
        };
        assert_eq!(*reason, PasteAbortReason::SizeLimit);
        // The aborted buffer includes the `\x1B[200~` prefix plus the byte that tripped
        // the cap.
        assert_eq!(buffered_bytes, b"\x1B[200~a");
        for event in rest {
            assert!(
                matches!(event, InternalEvent::Event(Event::Key(_))),
                "expected Key event after abort, got {event:?}",
            );
        }
    }

    #[test]
    fn aborts_paste_when_timeout_exceeded() {
        let _guard = LimitGuard::new(BracketedPasteLimits {
            max_duration: Some(Duration::from_millis(1)),
            max_bytes: None,
        });
        let mut parser = Parser::default();
        parser.advance(b"\x1B[200~a", false);
        std::thread::sleep(Duration::from_millis(20));
        // The next byte after the timeout has elapsed should trip the abort.
        parser.advance(b"b", false);
        let events = drain(&mut parser);
        assert!(
            matches!(
                events.as_slice(),
                [InternalEvent::Event(Event::PasteAborted {
                    reason: PasteAbortReason::Timeout,
                    ..
                })]
            ),
            "unexpected events: {events:?}",
        );
    }

    #[test]
    fn parser_recovers_to_normal_input_after_abort() {
        let _guard = LimitGuard::new(BracketedPasteLimits {
            max_duration: None,
            max_bytes: Some(8),
        });
        let mut parser = Parser::default();
        // Trigger an abort, then feed a complete ArrowLeft escape sequence.
        parser.advance(b"\x1B[200~xxxxxxxx", false);
        parser.advance(b"\x1B[D", false);
        let events = drain(&mut parser);
        assert!(
            matches!(events[0], InternalEvent::Event(Event::PasteAborted { .. })),
            "first event must be PasteAborted, got {:?}",
            events.first(),
        );
        assert!(
            matches!(events[1], InternalEvent::Event(Event::Key(_))),
            "second event must be a key event, got {:?}",
            events.get(1),
        );
    }

    #[test]
    fn limits_default_unbounded_preserves_old_behaviour() {
        let _guard = LimitGuard::new(BracketedPasteLimits::default());
        let mut parser = Parser::default();
        // Feed 64 KiB of payload without a closing marker; nothing should be emitted.
        parser.advance(b"\x1B[200~", false);
        parser.advance(&vec![b'x'; 64 * 1024], false);
        assert!(drain(&mut parser).is_empty());
    }
}
