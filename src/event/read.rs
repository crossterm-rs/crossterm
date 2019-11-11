use std::{collections::vec_deque::VecDeque, time::Duration};

#[cfg(unix)]
use super::source::tty::TtyInternalEventSource;
#[cfg(windows)]
use super::source::winapi::WinApiEventSource;
use super::{
    poll::EventPoll, poll_internal, read_internal, source::EventSource, timeout::PollTimeout,
    Event, InternalEvent, Result,
};
use crate::event::mask::{EventMask, EventOnlyMask};

/// Can be used to read `InternalEvent`s.
pub(crate) struct InternalEventReader {
    events: VecDeque<InternalEvent>,
    event_source: Box<dyn EventSource>,
}

impl Default for InternalEventReader {
    fn default() -> Self {
        #[cfg(windows)]
        let event_source = WinApiEventSource::default();
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
    pub(crate) fn new(source: Box<dyn EventSource>) -> Self {
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

    fn read(&mut self, mask: impl EventMask) -> Result<Self::Output> {
        loop {
            if let Some(event) = self.events.pop_front() {
                if mask.filter(&event) {
                    return Ok(event);
                } else {
                    self.events.push_front(event);
                }
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

        let mut timeout = PollTimeout::new(timeout);

        loop {
            if poll_internal(timeout.leftover())? {
                match read_internal(EventOnlyMask) {
                    Ok(InternalEvent::Event(ev)) => {
                        self.events.push_back(ev);
                        return Ok(true);
                    }
                    _ => {}
                }
            } else {
                return Ok(false);
            }

            if timeout.elapsed() {
                return Ok(false);
            }
        }
    }

    fn read(&mut self, _: impl EventMask) -> Result<Self::Output> {
        loop {
            if let Some(event) = self.events.pop_front() {
                return Ok(event);
            }

            let _ = self.poll(None)?;
        }
    }
}
