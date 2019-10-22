use std::sync::mpsc::Receiver;
use std::sync::Mutex;

use crate::EventSource;
use crate::InputEvent;

pub struct FakeEventSource {
    input_receiver: Mutex<Receiver<InputEvent>>,
}

impl FakeEventSource {
    pub fn new(input_receiver: Receiver<InputEvent>) -> FakeEventSource {
        FakeEventSource {
            input_receiver: Mutex::new(input_receiver),
        }
    }
}

impl EventSource for FakeEventSource {
    fn read_event(&mut self) -> crate::Result<Option<InputEvent>> {
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
}
