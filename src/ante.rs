use std::num::NonZero;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Ante(NonZero<u8>);

impl Default for Ante {
    fn default() -> Self {
        Self(unsafe { NonZero::new_unchecked(1) })
    }
}
