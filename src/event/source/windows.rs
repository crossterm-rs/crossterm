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
    internal::InternalEvent, source::EventSource, sys::parse::Parser, timeout::PollTimeout,
};

pub(crate) struct WindowsEventSource {
    console: Console,
    poll: WinApiPoll,
    surrogate_buffer: Option<u16>,
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

            surrogate_buffer: None,
            mouse_buttons_pressed: MouseButtonsPressed::default(),
            parser: Parser::default(),
            vt_input_enabled,
        })
    }

    /// Decode a UTF-16 code unit, handling surrogate pairs.
    fn decode_utf16_char(&mut self, utf16: u16) -> Option<char> {
        if (0xD800..=0xDBFF).contains(&utf16) {
            // High surrogate — store and wait for low surrogate
            self.surrogate_buffer = Some(utf16);
            None
        } else if (0xDC00..=0xDFFF).contains(&utf16) {
            // Low surrogate — combine with stored high surrogate
            if let Some(high) = self.surrogate_buffer.take() {
                std::char::decode_utf16([high, utf16]).next()?.ok()
            } else {
                None
            }
        } else {
            self.surrogate_buffer = None;
            std::char::from_u32(utf16 as u32)
        }
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
                    let mut remaining = number;
                    for _ in 0..number {
                        remaining -= 1;
                        match self.console.read_single_input_event()? {
                            InputRecord::KeyEvent(record) => {
                                if self.vt_input_enabled && record.u_char != 0 && record.key_down {
                                    // VT path: feed unicode character to ANSI parser as UTF-8.
                                    // With ENABLE_VIRTUAL_TERMINAL_INPUT, special keys produce
                                    // ANSI escape sequences as individual character bytes in
                                    // KEY_EVENT records.
                                    if let Some(ch) = self.decode_utf16_char(record.u_char) {
                                        let mut buf = [0u8; 4];
                                        let encoded = ch.encode_utf8(&mut buf);
                                        self.parser.advance(encoded.as_bytes(), remaining > 0);
                                    }
                                } else if !self.vt_input_enabled || record.u_char == 0 {
                                    // Non-VT fallback: use existing VK code handling.
                                    // When VT is enabled, keys with u_char==0 (e.g. standalone
                                    // modifier presses) still need VK code handling.
                                    if let Some(event) =
                                        handle_key_event(record, &mut self.surrogate_buffer)
                                    {
                                        self.parser.push_event(InternalEvent::Event(event));
                                    }
                                }
                                // VT enabled, key_down=false, u_char!=0: skip (release events
                                // don't carry new ANSI data)
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
