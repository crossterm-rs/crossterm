use crate::style::Attribute;
use std::ops::{BitAnd, BitOr, BitXor};

/// a bitset for all possible attributes
///
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Attributes(u64);

impl From<Attribute> for Attributes {
    fn from(attribute: Attribute) -> Self {
        Self(attribute.bytes())
    }
}

impl From<&[Attribute]> for Attributes {
    fn from(arr: &[Attribute]) -> Self {
        let mut attributes = Attributes::default();
        for &attr in arr {
            attributes.push(attr);
        }
        attributes
    }
}

impl BitAnd<Attribute> for Attributes {
    type Output = Self;
    fn bitand(self, rhs: Attribute) -> Self {
        Self(self.0 & rhs.bytes())
    }
}
impl BitAnd for Attributes {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl BitOr<Attribute> for Attributes {
    type Output = Self;
    fn bitor(self, rhs: Attribute) -> Self {
        Self(self.0 | rhs.bytes())
    }
}
impl BitOr for Attributes {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl BitXor<Attribute> for Attributes {
    type Output = Self;
    fn bitxor(self, rhs: Attribute) -> Self {
        Self(self.0 ^ rhs.bytes())
    }
}
impl BitXor for Attributes {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}

impl Attributes {
    #[inline(always)]
    pub fn push(&mut self, attribute: Attribute) {
        self.0 |= attribute.bytes();
    }
    #[inline(always)]
    pub fn remove(&mut self, attribute: Attribute) {
        self.0 &= !attribute.bytes();
    }
    #[inline(always)]
    pub fn toggle(&mut self, attribute: Attribute) {
        self.0 ^= attribute.bytes();
    }
    #[inline(always)]
    pub fn has(&self, attribute: Attribute) -> bool {
        self.0 & attribute.bytes() != 0
    }
    #[inline(always)]
    pub fn extend(&mut self, attributes: &Attributes) {
        self.0 = self.0 | attributes.0;
    }
    /// tells whether there's absolutely no attribute
    /// (not even a Reset) in the set
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::{Attribute, Attributes};

    #[test]
    fn test_attributes() {
        let mut attributes: Attributes = Attribute::Bold.into();
        assert!(attributes.has(Attribute::Bold));
        attributes.push(Attribute::Italic);
        assert!(attributes.has(Attribute::Italic));
        attributes.remove(Attribute::Italic);
        assert!(!attributes.has(Attribute::Italic));
        attributes.toggle(Attribute::Bold);
        assert!(attributes.is_empty());
    }
}
