use std::io;

pub(crate) struct Waker;

impl Waker {
    pub(crate) fn new() -> Self {
        Waker {}
    }

    pub(crate) fn wake(&self) -> io::Result<()> {
        Ok(())
    }

    pub(crate) fn clear(&self) -> io::Result<()> {
        Ok(())
    }
}
