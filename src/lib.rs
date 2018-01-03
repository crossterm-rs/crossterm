#[macro_use]
pub mod shared;

pub mod terminal_cursor;
pub mod terminal_style;
mod kernel;

pub use shared::traits::Construct;
pub use terminal_cursor::cursor;
pub use terminal_style::paint;



