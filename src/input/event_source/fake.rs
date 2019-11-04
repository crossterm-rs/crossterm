use std::sync::mpsc::Receiver;
use std::sync::Mutex;
use std::time::Duration;

use crate::input::events::InternalEvent;
use crate::input::EventSource;

pub struct FakeEventSource {
    input_receiver: Mutex<Receiver<InternalEvent>>,
}

impl FakeEventSource {
    pub fn new(input_receiver: Receiver<InternalEvent>) -> FakeEventSource {
        FakeEventSource {
            input_receiver: Mutex::new(input_receiver),
        }
    }
}

impl EventSource for FakeEventSource {
    fn read(&mut self) -> crate::Result<Option<InternalEvent>> {
        let input_receiver = self
            .input_receiver
            .lock()
            .expect("Can't acquire input receiver lock");

        Ok(Some(
            input_receiver
                .recv()
                .expect("Can't receive input from channel"),
        ))
    }

    fn poll(&mut self, timeout: Option<Duration>) -> crate::Result<bool> {
        // implement poll for receiver
        return Ok(true);
    }
}
