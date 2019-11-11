use std::{collections::vec_deque::VecDeque, time::Duration};

use crate::event::filter::{EventFilter, Filter};

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
        let event_source =
            WinApiEventSource::new().expect("Failed to setup the default event reader.");
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

    fn read(&mut self, event_filter: impl Filter) -> Result<Self::Output> {
        let mut unsatisfied_events = VecDeque::new();

        loop {
            if let Some(event) = self.events.pop_front() {
                if event_filter.filter(&event) {
                    if !unsatisfied_events.is_empty() {
                        while let Some(event) = unsatisfied_events.pop_front() {
                            self.events.push_back(event);
                        }
                    }

                    return Ok(event);
                } else {
                    // We can not directly write events back to `self.events`.
                    // If we did, we would put our self's into an endless loop
                    // that would enqueue -> dequeue -> enqueue etc.
                    // This happens because `poll` in this function will always return true if there are events in it's.
                    // And because we just put the non-fulfilling event there this is going to be the case.
                    // Instead we can store them into the temporary buffer,
                    // and then when the filter is fulfilled write all events back in order.
                    unsatisfied_events.push_back(event);
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
                match read_internal(EventFilter) {
                    Ok(InternalEvent::Event(ev)) => {
                        self.events.push_back(ev);
                        return Ok(true);
                    }
                    _ => { /* unreachable */ }
                }
            } else {
                return Ok(false);
            }

            if timeout.elapsed() {
                return Ok(false);
            }
        }
    }

    fn read(&mut self, _: impl Filter) -> Result<Self::Output> {
        loop {
            if let Some(event) = self.events.pop_front() {
                return Ok(event);
            }

            let _ = self.poll(None)?;
        }
    }
}
