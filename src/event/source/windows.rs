use std::time::Duration;

use crossterm_winapi::{Console, Handle, InputEventType, KeyEventRecord, MouseEvent};

use crate::event::{sys::windows::WinApiPoll, Event};

use super::super::{
    source::EventSource,
    sys::windows::{handle_key_event, handle_mouse_event},
    timeout::PollTimeout,
    InternalEvent, Result,
};

#[cfg(feature = "event-stream")]
use super::super::sys::Waker;

pub(crate) struct WindowsEventSource {
    console: Console,
    poll: WinApiPoll,
}

impl WindowsEventSource {
    pub(crate) fn new() -> Result<WindowsEventSource> {
        let console = Console::from(Handle::current_in_handle()?);
        Ok(WindowsEventSource {
            console,
            poll: WinApiPoll::new()?,
        })
    }
}

impl EventSource for WindowsEventSource {
    fn try_read(&mut self, timeout: Option<Duration>) -> Result<Option<InternalEvent>> {
        let poll_timeout = PollTimeout::new(timeout);

        loop {
            if let Some(event_ready) = self.poll.poll(timeout)? {
                if event_ready && self.console.number_of_console_input_events()? != 0 {
                    let input = self.console.read_single_input_event()?;

                    let event = match input.event_type {
                        InputEventType::KeyEvent => handle_key_event(unsafe {
                            KeyEventRecord::from(*input.event.KeyEvent())
                        })?,
                        InputEventType::MouseEvent => handle_mouse_event(unsafe {
                            MouseEvent::from(*input.event.MouseEvent())
                        })?,
                        InputEventType::WindowBufferSizeEvent => {
                            let new_size = crate::terminal::size()?;
                            Some(Event::Resize(new_size.0, new_size.1))
                        }
                        InputEventType::FocusEvent | InputEventType::MenuEvent => None,
                    };

                    return Ok(match event {
                        None => None,
                        Some(event) => Some(InternalEvent::Event(event)),
                    });
                }
            } else {
                return Ok(None);
            }

            if poll_timeout.elapsed() {
                return Ok(None);
            }
        }
    }

    #[cfg(feature = "event-stream")]
    fn try_read_waker(&self) -> Waker {
        self.poll.poll_waker()
    }
}
