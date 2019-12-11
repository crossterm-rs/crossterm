#[cfg(feature = "event-stream")]
pub(crate) use waker::Waker;

#[cfg(feature = "event-stream")]
mod waker;

pub(crate) mod file_descriptor;
pub(crate) mod parse;
