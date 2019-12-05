use std::sync::{Arc, Mutex};

use crossterm_winapi::Semaphore;

use crate::Result;

#[derive(Clone)]
pub(crate) struct Waker {
    inner: Arc<Mutex<Semaphore>>,
}

impl Waker {
    pub(crate) fn new() -> Result<Self> {
        let inner = Semaphore::new()?;

        Ok(Self {
            inner: Arc::new(Mutex::new(inner)),
        })
    }

    pub(crate) fn wake(&self) -> Result<()> {
        self.inner.lock().unwrap().release()?;
        Ok(())
    }

    pub(crate) fn reset(&self) -> Result<()> {
        *self.inner.lock().unwrap() = Semaphore::new()?;
        Ok(())
    }

    pub(crate) fn semaphore(&self) -> Semaphore {
        self.inner.lock().unwrap().clone()
    }
}
