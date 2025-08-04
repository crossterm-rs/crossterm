use super::internal::InternalEvent;
use crate::event::source::no_tty::NoTtyInternalEventSource;
use crate::event::source::EventSource;
use crate::event::{filter::Filter, read::InternalEventReader, timeout::PollTimeout};
use crate::terminal::WindowSize;
use crossbeam_channel::{bounded, Receiver, Sender};
use parking_lot::Mutex;
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone)]
pub struct NoTtyEvent {
    pub(crate) send: Sender<Vec<u8>>,
    pub window_size: Arc<Mutex<WindowSize>>,
    inner: Arc<Mutex<InternalEventReader>>,
}

impl NoTtyEvent {
    pub fn new(recv: Receiver<Vec<u8>>) -> (Self, Receiver<Vec<u8>>) {
        let (s, r) = bounded(0);
        let source = NoTtyInternalEventSource::new(recv);
        let source = source.ok().map(|x| Box::new(x) as Box<dyn EventSource>);
        let event = InternalEventReader::default().with_source(source);

        (
            Self {
                send: s,
                window_size: Arc::new(Mutex::new(WindowSize {
                    rows: 0,
                    columns: 0,
                    width: 0,
                    height: 0,
                })),
                inner: Arc::new(Mutex::new(event)),
            },
            r,
        )
    }
    /// Polls to check if there are any `InternalEvent`s that can be read within the given duration.
    pub(crate) fn poll<F>(&self, timeout: Option<Duration>, filter: &F) -> std::io::Result<bool>
    where
        F: Filter,
    {
        let (mut reader, timeout) = if let Some(timeout) = timeout {
            let poll_timeout = PollTimeout::new(Some(timeout));
            if let Some(reader) = self.inner.try_lock_for(timeout) {
                (reader, poll_timeout.leftover())
            } else {
                return Ok(false);
            }
        } else {
            (self.inner.lock(), None)
        };
        reader.poll(timeout, filter)
    }

    /// Reads a single `InternalEvent`.
    pub(crate) fn read<F>(&self, filter: &F) -> std::io::Result<InternalEvent>
    where
        F: Filter,
    {
        let mut reader = self.inner.lock();
        reader.read(filter)
    }

    /// Reads a single `InternalEvent`. Non-blocking.
    pub(crate) fn try_read<F>(&self, filter: &F) -> Option<InternalEvent>
    where
        F: Filter,
    {
        let mut reader = self.inner.lock();
        reader.try_read(filter)
    }
}
