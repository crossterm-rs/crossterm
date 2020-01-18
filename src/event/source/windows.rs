use std::time::Duration;

use crossterm_winapi::{Console, Handle, InputEvent, KeyEventRecord, MouseEvent};

use crate::event::{sys::windows::poll::WinApiPoll, Event};

#[cfg(feature = "event-stream")]
use super::super::sys::Waker;
use super::super::{
    source::EventSource,
    sys::windows::parse::{handle_key_event, handle_mouse_event},
    timeout::PollTimeout,
    InternalEvent, Result,
};

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
            if let Some(event_ready) = self.poll.poll(poll_timeout.leftover())? {
                if event_ready && self.console.number_of_console_input_events()? != 0 {
                    let event = match self.console.read_single_input_event()? {
                        InputEvent::KeyEvent(record) => handle_key_event(record)?,
                        InputEvent::MouseEvent(record) => handle_mouse_event(record)?,
                        InputEvent::WindowBufferSizeEvent(record) => {
                            Some(Event::Resize(record.size.x, record.size.y))
                        }
                        _ => None,
                    };

                    if let Some(event) = event {
                        return Ok(Some(InternalEvent::Event(event)));
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
