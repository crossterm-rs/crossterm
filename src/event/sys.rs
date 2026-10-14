#[cfg(unix)]
pub(crate) use unix::waker::Waker;
#[cfg(windows)]
pub(crate) use windows::waker::Waker;

#[cfg(unix)]
pub(crate) mod unix;
#[cfg(windows)]
pub(crate) mod windows;
