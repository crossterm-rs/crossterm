use std::thread;
use std::time::{Duration, Instant};

use crossterm_winapi::{Console, Handle};

use crate::input::events::InternalEvent;
use crate::input::sys::winapi::read_single_event;
use crate::input::EventSource;
use crate::Result;

pub struct WinApiEventSource;

impl WinApiEventSource {
    pub fn new() -> WinApiEventSource {
        WinApiEventSource
    }
}

impl EventSource for WinApiEventSource {
    fn read(&mut self) -> Result<Option<InternalEvent>> {
        match read_single_event()? {
            Some(event) => {
                return Ok(Some(InternalEvent::Input(event.clone())));
            }
            None => {
                return Ok(None);
            }
        }
    }

    fn poll(&mut self, timeout: Option<Duration>) -> crate::Result<bool> {
        let start_time = Instant::now();
        loop {
            if let Some(timeout) = timeout {
                if start_time.elapsed() >= timeout {
                    return Ok(false);
                }
            }

            let number_of_events =
                Console::from(Handle::current_in_handle()?).number_of_console_input_events()?;

            if number_of_events != 0 {
                return Ok(true);
            }

            thread::sleep(Duration::from_millis(100))
        }
    }
}
