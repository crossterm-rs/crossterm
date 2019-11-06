use std::{
    collections::VecDeque,
    sync::{mpsc::Receiver, Mutex},
    time::Duration,
};

use crate::{
    input::{event_source::EventSource, events::InternalEvent},
    Result,
};

/// This event source can be used for test purposes. And gives you direct control over the events read by crossterm.
pub struct FakeEventSource {
    input_receiver: Mutex<Receiver<InternalEvent>>,
    internal_buf: VecDeque<InternalEvent>,
}

impl FakeEventSource {
    /// Constructs a new `FakeEventSource` with the given `Receiver`, use the sender to trigger the event reader..
    pub fn new(input_receiver: Receiver<InternalEvent>) -> FakeEventSource {
        FakeEventSource {
            input_receiver: Mutex::new(input_receiver),
            internal_buf: VecDeque::new(),
        }
    }
}

impl EventSource for FakeEventSource {
    fn try_read(&mut self, timeout: Option<Duration>) -> Result<(bool, Option<InternalEvent>)> {
        if let Some(timeout) = timeout {
            if let Ok(val) = self.input_receiver.lock().unwrap().recv_timeout(timeout) {
                return Ok((true, Some(val)));
            } else {
                return Ok((false, None));
            }
        } else {
            let val = self.input_receiver.lock().unwrap().recv().unwrap();
            return Ok((true, Some(val)));
        }
    }
}
