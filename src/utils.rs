//! # Utils
//!
//! **The `crossterm_utils` crate is deprecated and no longer maintained. The GitHub repository will
//! be archived soon. All the code is being moved to the `crossterm`
//! [crate](https://github.com/crossterm-rs/crossterm). You can learn more in
//! the [Merge sub-crates to the crossterm crate](https://github.com/crossterm-rs/crossterm/issues/265)
//! issue.**
//!
//! This crate is **not meant for standalone use** and is really just a library
//! with some common used code for the [`crossterm`](https://crates.io/crates/crossterm)
//! crate and the above named modules.
//!
//! This crate will be deprecated soon and no longer maintained. It's highly recommended
//! to not use it.

pub use self::command::{Command, ExecutableCommand, Output, QueueableCommand};
pub use self::error::{ErrorKind, Result};
#[cfg(windows)]
pub use self::functions::supports_ansi;

mod command;
pub mod error;
mod functions;
pub mod macros;
pub mod sys;
