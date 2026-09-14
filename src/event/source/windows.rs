use std::time::Duration;

use crossterm_winapi::{Console, ConsoleMode, Handle, InputRecord};

use crate::event::{
    Event, KeyModifiers,
    sys::windows::{
        parse::{self, MouseButtonsPressed, handle_key_event, handle_mouse_event},
        poll::WinApiPoll,
    },
};

#[cfg(feature = "event-stream")]
use crate::event::sys::Waker;
use crate::event::{
    internal::InternalEvent,
    source::EventSource,
    sys::parse::{Parser, decode_utf16_char},
    timeout::PollTimeout,
};

pub(crate) struct WindowsEventSource {
    console: Console,
    poll: WinApiPoll,
    /// Surrogate buffer for the VT path (decode_utf16_char).
    vt_surrogate: Option<u16>,
    /// Surrogate buffer for the non-VT fallback path (handle_key_event).
    /// Separate from vt_surrogate because both paths can execute within a
    /// single batch: VT path for u_char != 0 events, non-VT for u_char == 0.
    legacy_surrogate: Option<u16>,
    mouse_buttons_pressed: MouseButtonsPressed,
    /// Candidate character from conhost's packet release, awaiting the immediately adjacent
    /// duplicate VT character record. The source clears this on every non-key record and mode
    /// transition; it is deliberately a finite record-state check, not a timing heuristic.
    alt_numpad_pending: Option<u16>,
    parser: Parser,
    /// Console mode for the same CONIN$ handle used by `console`.
    ///
    /// Keeping this handle alongside the input reader avoids opening and closing CONIN$ for
    /// every mode query.  The mode is intentionally queried once per input batch, rather than
    /// cached for the lifetime of the event source: raw mode can be toggled between batches.
    console_mode: ConsoleMode,
}

impl WindowsEventSource {
    pub(crate) fn new() -> std::io::Result<WindowsEventSource> {
        let input_handle = Handle::current_in_handle()?;
        let console = Console::from(input_handle.clone());
        let console_mode = ConsoleMode::from(input_handle);
        Ok(WindowsEventSource {
            console,

            #[cfg(not(feature = "event-stream"))]
            poll: WinApiPoll::new(),
            #[cfg(feature = "event-stream")]
            poll: WinApiPoll::new()?,

            vt_surrogate: None,
            legacy_surrogate: None,
            mouse_buttons_pressed: MouseButtonsPressed::default(),
            alt_numpad_pending: None,
            parser: Parser::default(),
            console_mode,
        })
    }
}

impl EventSource for WindowsEventSource {
    fn try_read(&mut self, timeout: Option<Duration>) -> std::io::Result<Option<InternalEvent>> {
        // Return buffered events first
        if let Some(event) = self.parser.next_event() {
            return Ok(Some(event));
        }

        let poll_timeout = PollTimeout::new(timeout);

        loop {
            if let Some(event_ready) = self.poll.poll(poll_timeout.leftover())? {
                if event_ready {
                    // Raw mode may have been enabled or disabled since the last call.  Query
                    // the mode of the same handle used to read this batch, so the event
                    // lifecycle never routes records using stale VT state.  Mode-query errors
                    // are propagated because silently selecting the VK path could misdecode a
                    // batch that was actually produced in VT mode.
                    let mode = self.console_mode.mode()?;
                    let vt_input_enabled =
                        mode & crate::event::sys::windows::ENABLE_VIRTUAL_TERMINAL_INPUT != 0;
                    let raw_mode = crate::terminal::sys::is_raw_mode_from_console_mode(mode);
                    if !vt_input_enabled {
                        // The candidate is meaningful only while the VT/Win32 hybrid route is
                        // active. A raw-mode toggle must not carry it into the legacy path.
                        self.alt_numpad_pending = None;
                    }

                    // Process all available input records as a batch.
                    // Batch reading is essential for VT mode because ANSI escape
                    // sequences are spread across multiple KEY_EVENT records.
                    // `read_console_input` snapshots the queue and returns only the records
                    // actually read.  This removes the per-record race introduced by repeatedly
                    // reading `number` single records; the API still has the same race as the
                    // master branch if another reader drains the whole queue before this call.
                    let input_records = self.console.read_console_input()?;
                    if input_records.is_empty() {
                        continue;
                    }

                    let batch_len = input_records.len();
                    let mut vt_bytes_consumed = false;
                    for (index, record) in input_records.into_iter().enumerate() {
                        match record {
                            InputRecord::KeyEvent(record) => {
                                let suppress_alt_numpad = vt_input_enabled
                                    && parse::suppress_alt_numpad_duplicate(
                                        &mut self.alt_numpad_pending,
                                        record.virtual_key_code as u16,
                                        record.key_down,
                                        record.u_char,
                                        KeyModifiers::from(&record.control_key_state)
                                            .contains(KeyModifiers::ALT),
                                    );
                                if suppress_alt_numpad {
                                    continue;
                                }

                                if vt_input_enabled && record.u_char != 0 && record.key_down {
                                    vt_bytes_consumed = true;
                                    // VT path: feed unicode character to ANSI parser as UTF-8.
                                    // With ENABLE_VIRTUAL_TERMINAL_INPUT, special keys produce
                                    // ANSI escape sequences as individual character bytes in
                                    // KEY_EVENT records. Non-key events (mouse, focus, resize)
                                    // don't touch vt_surrogate, so interleaved events between
                                    // surrogate pair halves are harmless.
                                    if let Some(ch) =
                                        decode_utf16_char(&mut self.vt_surrogate, record.u_char)
                                    {
                                        let mut buf = [0u8; 4];
                                        let encoded = ch.encode_utf8(&mut buf);
                                        // Preserve incomplete ANSI sequences (for example a
                                        // trailing ESC from bracketed paste) across batch
                                        // boundaries. If this is the last record in the current
                                        // snapshot, probe the console queue once more before
                                        // deciding that no additional bytes are pending.
                                        let more_input_available = if index + 1 < batch_len {
                                            true
                                        } else {
                                            self.console.number_of_console_input_events()? > 0
                                        };
                                        self.parser.advance_with_raw_mode(
                                            encoded.as_bytes(),
                                            more_input_available,
                                            raw_mode,
                                        );
                                    }
                                } else if vt_input_enabled && record.u_char != 0 && !record.key_down
                                {
                                    if let Some(event) = parse::handle_vt_key_release(
                                        record,
                                        &mut self.legacy_surrogate,
                                        raw_mode,
                                    ) {
                                        self.parser.push_event(InternalEvent::Event(event));
                                    }
                                } else {
                                    // Non-VT fallback: use existing VK code handling.  This is
                                    // also required for VT batches when the record is a key-up
                                    // event or has no Unicode character: handle_key_event keeps
                                    // release events and Alt-code records intact.
                                    if let Some(event) =
                                        handle_key_event(record, &mut self.legacy_surrogate)
                                    {
                                        self.parser.push_event(InternalEvent::Event(event));
                                    }
                                }
                            }
                            InputRecord::MouseEvent(record) => {
                                // Alt-numpad deduplication is record-adjacent. Any non-key input
                                // between the packet release and a character invalidates it.
                                self.alt_numpad_pending = None;
                                let mouse_event =
                                    handle_mouse_event(record, &self.mouse_buttons_pressed);
                                self.mouse_buttons_pressed = MouseButtonsPressed {
                                    left: record.button_state.left_button(),
                                    right: record.button_state.right_button(),
                                    middle: record.button_state.middle_button(),
                                };
                                if let Some(event) = mouse_event {
                                    self.parser.push_event(InternalEvent::Event(event));
                                }
                            }
                            InputRecord::WindowBufferSizeEvent(record) => {
                                self.alt_numpad_pending = None;
                                // windows starts counting at 0, unix at 1, add one to replicate unix behaviour.
                                self.parser.push_event(InternalEvent::Event(Event::Resize(
                                    (record.size.x as i32 + 1) as u16,
                                    (record.size.y as i32 + 1) as u16,
                                )));
                            }
                            InputRecord::FocusEvent(record) => {
                                self.alt_numpad_pending = None;
                                let event = if record.set_focus {
                                    Event::FocusGained
                                } else {
                                    Event::FocusLost
                                };
                                self.parser.push_event(InternalEvent::Event(event));
                            }
                            _ => {
                                self.alt_numpad_pending = None;
                            }
                        }
                    }

                    // Flush any lone ESC (or other stalled sequence) from the parser buffer:
                    //   1. No VT bytes in this batch at all: the ESC was written in a
                    //      previous batch and held because the queue appeared non-empty;
                    //      now the remaining queue entries are all non-key events, so flush.
                    //   2. VT bytes were consumed but the console queue is now empty: the
                    //      buffered sequence won't be completed by a subsequent batch, so
                    //      force-emit it rather than leaving it stuck indefinitely.
                    if !vt_bytes_consumed || self.console.number_of_console_input_events()? == 0 {
                        self.parser.flush_with_raw_mode(raw_mode);
                    }

                    // Return first available event from the batch
                    if let Some(event) = self.parser.next_event() {
                        return Ok(Some(event));
                    }
                }
            }

            if poll_timeout.elapsed() {
                return Ok(None);
            }
        }
    }

    #[cfg(feature = "event-stream")]
    fn waker(&self) -> Waker {
        self.poll.waker()
    }
}
