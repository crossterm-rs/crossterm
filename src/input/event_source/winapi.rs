use crate::input::EventSource;
use crate::input::sys::winapi::read_single_event;
use crate::InputEvent;
use crate::Result;

pub struct WinApiEventSource;

impl WinApiEventSource {
    pub fn new() -> WinApiEventSource {
        WinApiEventSource
    }
}

impl EventSource for WinApiEventSource {
    fn read_event(&mut self) -> Result<Option<InputEvent>> {
        read_single_event()
    }
}
