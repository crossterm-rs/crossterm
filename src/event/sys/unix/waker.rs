use std::sync::{Arc, Mutex};

use mio::{Evented, Poll, PollOpt, Ready, Registration, SetReadiness, Token};

use crate::Result;

struct WakerInner {
    registration: Registration,
    set_readiness: SetReadiness,
}

impl WakerInner {
    fn new() -> Self {
        let (registration, set_readiness) = Registration::new2();

        Self {
            registration,
            set_readiness,
        }
    }

    fn wake(&self) -> Result<()> {
        self.set_readiness.set_readiness(Ready::readable())?;
        Ok(())
    }

    fn reset(&self) -> Result<()> {
        self.set_readiness.set_readiness(Ready::empty())?;
        Ok(())
    }
}

#[derive(Clone)]
pub(crate) struct Waker {
    inner: Arc<Mutex<WakerInner>>,
}

impl Waker {
    pub(crate) fn new() -> Result<Self> {
        Ok(Self {
            inner: Arc::new(Mutex::new(WakerInner::new())),
        })
    }

    pub(crate) fn wake(&self) -> Result<()> {
        self.inner.lock().unwrap().wake()
    }

    pub(crate) fn reset(&self) -> Result<()> {
        self.inner.lock().unwrap().reset()
    }
}

impl Evented for Waker {
    fn register(
        &self,
        poll: &Poll,
        token: Token,
        interest: Ready,
        opts: PollOpt,
    ) -> ::std::io::Result<()> {
        self.inner
            .lock()
            .unwrap()
            .registration
            .register(poll, token, interest, opts)
    }

    fn reregister(
        &self,
        poll: &Poll,
        token: Token,
        interest: Ready,
        opts: PollOpt,
    ) -> ::std::io::Result<()> {
        self.inner
            .lock()
            .unwrap()
            .registration
            .reregister(poll, token, interest, opts)
    }

    #[allow(deprecated)]
    fn deregister(&self, poll: &Poll) -> ::std::io::Result<()> {
        self.inner.lock().unwrap().registration.deregister(poll)
    }
}
