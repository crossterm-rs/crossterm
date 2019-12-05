use std::io;

use mio::{Evented, Poll, PollOpt, Ready, Registration, SetReadiness, Token};

#[derive(Debug)]
pub(crate) struct Waker {
    registration: Registration,
    set_readiness: SetReadiness,
}

impl Waker {
    pub(crate) fn new() -> Self {
        let (registration, set_readiness) = Registration::new2();

        Waker {
            registration,
            set_readiness,
        }
    }

    pub(crate) fn wake(&self) -> io::Result<()> {
        self.set_readiness.set_readiness(Ready::readable())
    }

    pub(crate) fn clear(&self) -> io::Result<()> {
        self.set_readiness.set_readiness(Ready::empty())
    }
}

impl Evented for Waker {
    fn register(
        &self,
        poll: &Poll,
        token: Token,
        interest: Ready,
        opts: PollOpt,
    ) -> io::Result<()> {
        self.registration.register(poll, token, interest, opts)
    }

    fn reregister(
        &self,
        poll: &Poll,
        token: Token,
        interest: Ready,
        opts: PollOpt,
    ) -> io::Result<()> {
        self.registration.reregister(poll, token, interest, opts)
    }

    #[allow(deprecated)]
    fn deregister(&self, poll: &Poll) -> io::Result<()> {
        self.registration.deregister(poll)
    }
}
