use crate::sealed::Sealed;
use perfect_derive::perfect_derive;
use std::convert::Infallible;

mod list;

pub(crate) use list::*;

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

pub trait HasEditionExt: HasEdition {
    fn is_foil(&self) -> bool
    where
        Self: HasEdition<Scoring = ()>,
    {
        self.edition()
            .is_some_and(|e| matches!(e, Edition::Foil(())))
    }

    fn is_holographic(&self) -> bool
    where
        Self: HasEdition<Scoring = ()>,
    {
        self.edition()
            .is_some_and(|e| matches!(e, Edition::Holographic(())))
    }

    fn is_polychrome(&self) -> bool
    where
        Self: HasEdition<Scoring = ()>,
    {
        self.edition()
            .is_some_and(|e| matches!(e, Edition::Polychrome(())))
    }

    fn is_negative(&self) -> bool
    where
        Self: HasEdition<Negative = ()>,
    {
        self.edition()
            .is_some_and(|e| matches!(e, Edition::Negative(())))
    }
}

impl<H: HasEdition> HasEditionExt for H {}

pub trait EditionMode: Sealed + Copy + Eq {}

impl EditionMode for () {}
impl EditionMode for Infallible {}
