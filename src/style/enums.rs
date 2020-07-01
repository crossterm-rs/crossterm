pub use self::{attribute::Attribute, color::Color, colored::Colored};

mod attribute;
mod color;
mod colored;

/// Utility function for ANSI parsing in color and colored.
/// Gets the next element of `iter` and tries to parse it as a u8.
fn parse_next_u8<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Option<u8> {
    iter.next()
        .and_then(|s| u8::from_str_radix(s, 10).map(Some).unwrap_or(None))
}
