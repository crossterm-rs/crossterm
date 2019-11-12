use std::{thread, time::Duration};

use crossterm_winapi::{Console, Handle, InputEventType, KeyEventRecord, MouseEvent};

use super::super::{
    source::EventSource,
    sys::winapi::enable_mouse_capture,
    sys::winapi::{handle_key_event, handle_mouse_event},
    timeout::PollTimeout,
    InternalEvent, Result,
};
use crate::event::Event;

pub(crate) struct WinApiEventSource;

impl WinApiEventSource {
    pub(crate) fn new() -> Result<Self> {
        enable_mouse_capture()?;

        Ok(WinApiEventSource)
    }
}

impl EventSource for WinApiEventSource {
    fn try_read(&mut self, timeout: Option<Duration>) -> Result<Option<InternalEvent>> {
        let mut timeout = PollTimeout::new(timeout);
        let console = Console::from(Handle::current_in_handle()?);

        loop {
            let number_of_events = console.number_of_console_input_events()?;

            if number_of_events != 0 {
                let input = console.read_single_input_event()?;

                let event = match input.event_type {
                    InputEventType::KeyEvent => {
                        handle_key_event(unsafe { KeyEventRecord::from(*input.event.KeyEvent()) })?
                    }
                    InputEventType::MouseEvent => {
                        handle_mouse_event(unsafe { MouseEvent::from(*input.event.MouseEvent()) })?
                    }
                    InputEventType::WindowBufferSizeEvent => {
                        let new_size = crate::terminal::size()?;
                        Some(Event::Resize(new_size.0, new_size.1))
                    }
                    InputEventType::FocusEvent | InputEventType::MenuEvent => None,
                };

                if let Some(event) = event {
                    return Ok(Some(InternalEvent::Event(event)));
                }
            }

            if timeout.elapsed() {
                return Ok(None);
            }

            thread::sleep(Duration::from_millis(50))
        }
    }

    // TODO See TtyInternalEventSource::wake
    fn wake(&self) {}
}
