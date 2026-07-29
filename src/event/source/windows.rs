use std::time::Duration;

use crossterm_winapi::{Console, Handle, InputRecord};

use crate::event::{
    sys::windows::{
        parse::MouseButtonsPressed,
        parse::{handle_key_event, handle_mouse_event},
        poll::WinApiPoll,
        try_enable_vt_input,
    },
    Event,
};

#[cfg(feature = "event-stream")]
use crate::event::sys::Waker;
use crate::event::{
    internal::InternalEvent,
    source::EventSource,
    sys::parse::{decode_utf16_char, Parser},
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
    parser: Parser,
    vt_input_enabled: bool,
}

impl WindowsEventSource {
    pub(crate) fn new() -> std::io::Result<WindowsEventSource> {
        let console = Console::from(Handle::current_in_handle()?);
        let vt_input_enabled = try_enable_vt_input()?;
        Ok(WindowsEventSource {
            console,

            #[cfg(not(feature = "event-stream"))]
            poll: WinApiPoll::new(),
            #[cfg(feature = "event-stream")]
            poll: WinApiPoll::new()?,

            vt_surrogate: None,
            legacy_surrogate: None,
            mouse_buttons_pressed: MouseButtonsPressed::default(),
            parser: Parser::default(),
            vt_input_enabled,
        })
    }
}

impl EventSource for WindowsEventSource {
    fn try_read(&mut self, timeout: Option<Duration>) -> std::io::Result<Option<InternalEvent>> {
        // Return buffered events first
        if let Some(event) = self.parser.next() {
            return Ok(Some(event));
        }

        let poll_timeout = PollTimeout::new(timeout);

        loop {
            if let Some(event_ready) = self.poll.poll(poll_timeout.leftover())? {
                let number = self.console.number_of_console_input_events()?;
                if event_ready && number != 0 {
                    // Process all available input records as a batch.
                    // Batch reading is essential for VT mode because ANSI escape
                    // sequences are spread across multiple KEY_EVENT records.
                    // Note: `number` is read once before the loop. The count can
                    // become stale while we process the batch, so don't rely on
                    // `remaining > 0` alone to decide whether more bytes are
                    // immediately available for ANSI parsing.
                    let mut remaining = number;
                    let mut vt_bytes_consumed = false;
                    for _ in 0..number {
                        remaining -= 1;
                        match self.console.read_single_input_event()? {
                            InputRecord::KeyEvent(record) => {
                                if self.vt_input_enabled && record.u_char != 0 && record.key_down {
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
                                        let more_input_available = if remaining > 0 {
                                            true
                                        } else {
                                            self.console.number_of_console_input_events()? > 0
                                        };
                                        self.parser
                                            .advance(encoded.as_bytes(), more_input_available);
                                    }
                                } else if !self.vt_input_enabled || record.u_char == 0 {
                                    // Non-VT fallback: use existing VK code handling.
                                    // When VT is enabled, keys with u_char==0 (e.g. standalone
                                    // modifier presses) still need VK code handling.
                                    if let Some(event) =
                                        handle_key_event(record, &mut self.legacy_surrogate)
                                    {
                                        self.parser.push_event(InternalEvent::Event(event));
                                    }
                                }
                                // VT enabled, key_down=false, u_char!=0: intentionally
                                // skipped. Release events don't carry new ANSI data, and
                                // crossterm only reports key-press events. In non-VT mode,
                                // handle_key_event would also discard most key-up events.
                            }
                            InputRecord::MouseEvent(record) => {
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
                                // windows starts counting at 0, unix at 1, add one to replicate unix behaviour.
                                self.parser.push_event(InternalEvent::Event(Event::Resize(
                                    (record.size.x as i32 + 1) as u16,
                                    (record.size.y as i32 + 1) as u16,
                                )));
                            }
                            InputRecord::FocusEvent(record) => {
                                let event = if record.set_focus {
                                    Event::FocusGained
                                } else {
                                    Event::FocusLost
                                };
                                self.parser.push_event(InternalEvent::Event(event));
                            }
                            _ => {}
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
                        self.parser.flush();
                    }

                    // Return first available event from the batch
                    if let Some(event) = self.parser.next() {
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
