use std::{collections::vec_deque::VecDeque, time::Duration};

#[cfg(unix)]
use super::source::tty::TtyInternalEventSource;
#[cfg(windows)]
use super::source::winapi::WinApiEventSource;
use super::{
    poll::EventPoll, poll_internal, read_internal, source::EventSource, timeout::PollTimeout,
    Event, InternalEvent, Result,
};

/// Can be used to read `InternalEvent`s.
pub(crate) struct InternalEventReader {
    events: VecDeque<InternalEvent>,
    event_source: Box<dyn EventSource>,
}

impl Default for InternalEventReader {
    fn default() -> Self {
        #[cfg(windows)]
        let event_source = WinApiEventSource::new();
        #[cfg(unix)]
        let event_source =
            TtyInternalEventSource::new().expect("Failed to setup the default event reader.");

        InternalEventReader {
            event_source: Box::new(event_source),
            events: VecDeque::new(),
        }
    }
}

impl InternalEventReader {
    /// Constructs a new `InternalEventReader`.
    #[cfg(test)]
    pub(crate) fn new(source: Box<dyn EventSource>) -> InternalEventReader {
        InternalEventReader {
            event_source: source,
            events: VecDeque::new(),
        }
    }

    /// Enqueues the given `InternalEvent` onto the internal input buffer.
    #[cfg(unix)]
    pub(crate) fn enqueue(&mut self, event: InternalEvent) {
        self.events.push_back(event);
    }
}

impl EventPoll for InternalEventReader {
    type Output = InternalEvent;

    fn poll(&mut self, timeout: Option<Duration>) -> Result<bool> {
        if !self.events.is_empty() {
            return Ok(true);
        }

        let event = self.event_source.try_read(timeout)?;

        match event {
            None => Ok(false),
            Some(event) => {
                self.events.push_back(event);
                Ok(true)
            }
        }
    }

    fn read(&mut self) -> Result<Self::Output> {
        loop {
            if let Some(event) = self.events.pop_front() {
                return Ok(event);
            }

            let _ = self.poll(None)?;
        }
    }
}

/// Can be used to read `Event`s.
pub struct EventReader {
    events: VecDeque<Event>,
}

impl Default for EventReader {
    fn default() -> Self {
        EventReader {
            events: VecDeque::new(),
        }
    }
}

impl EventPoll for EventReader {
    type Output = Event;

    fn poll(&mut self, timeout: Option<Duration>) -> Result<bool> {
        if !self.events.is_empty() {
            return Ok(true);
        }

        let mut timer = PollTimeout::new(timeout);

        loop {
            if poll_internal(timer.leftover())? {
                match read_internal() {
                    Ok(InternalEvent::Event(ev)) => {
                        self.events.push_back(ev);
                        return Ok(true);
                    }
                    #[cfg(unix)]
                    Ok(event) => {
                        // write internal event back, we don't need it. But user might.
                        super::enqueue_internal(event);
                    }
                    _ => {}
                }
            } else {
                return Ok(false);
            }

            if timer.elapsed() {
                return Ok(false);
            }
        }
    }

    fn read(&mut self) -> Result<Self::Output> {
        loop {
            if let Some(event) = self.events.pop_front() {
                return Ok(event);
            }

            let _ = self.poll(None)?;
        }
    }
}
