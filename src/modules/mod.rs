pub mod cursor;
pub mod input;
pub mod write;
pub mod style;
//pub mod handle;
pub mod terminal;

use super::common::commands;
use super::common::functions;
use super::common::traits;
pub use self::write::IStdout;
