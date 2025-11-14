#[cfg(feature = "event-stream")]
#[cfg(not(feature = "no-tty"))]
pub(crate) mod waker;

#[cfg(feature = "events")]
pub(crate) mod parse;
