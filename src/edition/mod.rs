use crate::sealed::Sealed;
use perfect_derive::perfect_derive;
use std::convert::Infallible;

mod list;

pub use list::*;

#[perfect_derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Edition<T: HasEdition> {
    Foil(T::Scoring),
    Holographic(T::Scoring),
    Polychrome(T::Scoring),
    Negative(T::Negative),
}

pub trait HasEdition: Sized {
    type Scoring: EditionMode;
    type Negative: EditionMode;

    fn edition(&self) -> Option<Edition<Self>>;
}

pub trait EditionMode: Sealed + Copy + Eq {}

impl EditionMode for () {}
impl EditionMode for Infallible {}
