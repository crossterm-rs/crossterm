use std::{thread, time::Duration};

use crossterm_winapi::{Console, Handle, InputEventType, KeyEventRecord, MouseEvent};

use crate::{
    input::{
        event_source::EventSource,
        events::InternalEvent,
        poll_timeout::PollTimeOut,
        sys::winapi::{handle_key_event, handle_mouse_event},
    },
    Result,
};

pub struct WinApiEventSource;

impl WinApiEventSource {
    pub fn new() -> WinApiEventSource {
        WinApiEventSource
    }
}

impl EventSource for WinApiEventSource {
    fn try_read(&mut self, timeout: Option<Duration>) -> Result<Option<InternalEvent>> {
        let mut poll_timout = PollTimeOut::new(timeout);

        loop {
            let number_of_events =
                Console::from(Handle::current_in_handle()?).number_of_console_input_events()?;

            if number_of_events != 0 {
                let console = Console::from(Handle::current_in_handle()?);

                let input = console.read_single_input_event()?;

                let event = match input.event_type {
                    InputEventType::KeyEvent => {
                        handle_key_event(unsafe { KeyEventRecord::from(*input.event.KeyEvent()) })?
                    }
                    InputEventType::MouseEvent => {
                        handle_mouse_event(unsafe { MouseEvent::from(*input.event.MouseEvent()) })?
                    }
                    InputEventType::WindowBufferSizeEvent
                    | InputEventType::FocusEvent
                    | InputEventType::MenuEvent => None,
                };

                match event {
                    None => return Ok(None),
                    Some(event) => return Ok(Some(InternalEvent::Input(event))),
                }
            }

            if poll_timout.elapsed() {
                break;
            }

            thread::sleep(Duration::from_millis(50))
        }

        Ok(None)
    }
}
