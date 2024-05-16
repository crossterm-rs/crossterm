//! Interface to the C system library.
//!
//! This uses either rustix or libc.

#[cfg(feature = "rustix")]
pub(crate) use rustix::*;

#[cfg(not(feature = "rustix"))]
mod minirustix;

#[cfg(not(feature = "rustix"))]
pub(crate) use minirustix::*;
