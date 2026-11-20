use std::{
    io::{self, Write},
    os::{fd::AsRawFd, unix::net::UnixStream},
    sync::Arc,
};

use ::mio::{Interest, Registry, Token, unix::SourceFd};

/// Uses a separate descriptor so registry operations cannot consume an application wake.
#[derive(Clone, Debug)]
pub(crate) struct Waker(Arc<UnixStream>);

impl Waker {
    pub(crate) fn new(registry: &Registry, token: Token) -> io::Result<(Self, UnixStream)> {
        let (sender, receiver) = UnixStream::pair()?;
        sender.set_nonblocking(true)?;
        receiver.set_nonblocking(true)?;
        registry.register(
            &mut SourceFd(&receiver.as_raw_fd()),
            token,
            Interest::READABLE,
        )?;
        Ok((Self(Arc::new(sender)), receiver))
    }

    pub(crate) fn wake(&self) -> io::Result<()> {
        match (&*self.0).write_all(&[0]) {
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => Ok(()),
            result => result,
        }
    }
}
