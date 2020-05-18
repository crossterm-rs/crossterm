use std::sync::{Arc, Mutex};

use mio::{Token, Registry};

use crate::{Result, ErrorKind};

/// Allows to wake up the `mio::Poll::poll()` method.
#[derive(Clone, Debug)]
pub(crate) struct Waker {
    inner: Arc<Mutex<mio::Waker>>,
}

impl Waker {
    /// Creates a new waker.
    ///
    /// `Waker` implements the `mio::Evented` trait and you have to register
    /// it in order to use it.
    pub(crate) fn new(registry: &Registry, waker_token: Token) -> Result<Self> {
        Ok(Self {
            inner: Arc::new(Mutex::new(mio::Waker::new(registry, waker_token)?)),
        })
    }

    /// Wakes the `mio::Poll.poll()` method.
    ///
    /// Readiness is set to `Ready::readable()`.
    pub(crate) fn wake(&self) -> Result<()> {
        self.inner.lock().unwrap().wake()
            .map_err(|e| ErrorKind::IoError(e))
    }

    /// Resets the state so the same waker can be reused.
    ///
    /// Readiness is set back to `Ready::empty()`.
    pub(crate) fn reset(&self) -> Result<()> {
        Ok(())
    }
}