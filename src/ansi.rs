/// Wrapper type for write dynamic ansi string
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Ansi<T>(pub T);
