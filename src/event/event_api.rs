mod types;
mod api;
#[cfg(feature = "event-stream")]
#[cfg(unix)]
pub(crate) mod waker;
pub(crate) mod filter;
pub(crate) mod read;
pub(crate) mod source;
#[cfg(feature = "event-stream")]
pub(crate) mod stream;
pub(crate) mod timeout;
pub(crate) mod sys;

#[cfg(feature = "event-stream")]
pub use stream::EventStream;

pub use self::{
    types::*,
    api::*
};