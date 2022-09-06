use std::{
    io::Write,
    os::unix::net::UnixStream,
    sync::{Arc, Mutex},
};

use crate::Result;

/// Allows to wake up the EventSource::try_read() method.
#[derive(Clone, Debug)]
pub(crate) struct Waker {
    inner: Arc<Mutex<UnixStream>>,
}

impl Waker {
    /// Create a new `Waker`.
    pub(crate) fn new(writer: UnixStream) -> Self {
        Self {
            inner: Arc::new(Mutex::new(writer)),
        }
    }

    /// Wake up the [`Poll`] associated with this `Waker`.
    ///
    /// Readiness is set to `Ready::readable()`.
    pub(crate) fn wake(&self) -> Result<()> {
        self.inner.lock().unwrap().write(&[0])?;
        Ok(())
    }

    /// Resets the state so the same waker can be reused.
    ///
    /// This function is not impl
    #[allow(dead_code, clippy::clippy::unnecessary_wraps)]
    pub(crate) fn reset(&self) -> Result<()> {
        Ok(())
    }
}
