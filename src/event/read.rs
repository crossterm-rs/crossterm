use std::{collections::vec_deque::VecDeque, time::Duration};

use super::filter::Filter;

#[cfg(unix)]
use super::source::tty::TtyInternalEventSource;
#[cfg(windows)]
use super::source::windows::WindowsEventSource;
use super::{source::EventSource, timeout::PollTimeout, InternalEvent, Result};

/// Can be used to read `InternalEvent`s.
pub(crate) struct InternalEventReader {
    events: VecDeque<InternalEvent>,
    event_source: Option<Box<dyn EventSource>>,
}

impl Default for InternalEventReader {
    fn default() -> Self {
        #[cfg(windows)]
        let event_source = WindowsEventSource::new();
        #[cfg(unix)]
        let event_source = TtyInternalEventSource::new();

        let event_source = match event_source {
            Ok(source) => Some(Box::new(source) as Box<dyn EventSource>),
            Err(_) => None,
        };

        InternalEventReader {
            event_source,
            events: VecDeque::new(),
        }
    }
}

impl InternalEventReader {
    pub(crate) fn wake(&self) {
        if let Some(source) = self.event_source.as_ref() {
            source.wake();
        }
    }

    pub(crate) fn poll<F>(&mut self, timeout: Option<Duration>, filter: &F) -> Result<bool>
    where
        F: Filter,
    {
        for event in &self.events {
            if filter.filter(&event) {
                return Ok(true);
            }
        }

        let event_source = match self.event_source.as_mut() {
            Some(source) => source,
            None => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to initialize input reader",
                )
                .into())
            }
        };

        let poll_timeout = PollTimeout::new(timeout);
        let mut skipped_events = VecDeque::new();

        loop {
            let maybe_event = match event_source.try_read(timeout)? {
                None => None,
                Some(event) => {
                    if filter.filter(&event) {
                        Some(event)
                    } else {
                        skipped_events.push_back(event);
                        None
                    }
                }
            };

            if poll_timeout.elapsed() || maybe_event.is_some() {
                while let Some(event) = skipped_events.pop_front() {
                    self.events.push_back(event);
                }

                if let Some(event) = maybe_event {
                    self.events.push_front(event);
                    return Ok(true);
                }

                return Ok(false);
            }
        }
    }

    pub(crate) fn read<F>(&mut self, filter: &F) -> Result<InternalEvent>
    where
        F: Filter,
    {
        let mut skipped_events = VecDeque::new();

        loop {
            while let Some(event) = self.events.pop_front() {
                if filter.filter(&event) {
                    while let Some(event) = skipped_events.pop_front() {
                        self.events.push_back(event);
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
                    skipped_events.push_back(event);
                }
            }

            let _ = self.poll(None, filter)?;
        }
    }
}
