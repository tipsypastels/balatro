use super::*;
use crate::{Chips, Mult, Slate};

// TODO: Which parts are mut?
#[derive(Debug)]
#[non_exhaustive]
pub struct Scorer {
    pub jokers: Slate<Joker>,
    pub chips: Chips,
    pub mult: Mult,
}

impl Scorer {
    pub(crate) fn new(jokers: Slate<Joker>) -> Self {
        Self {
            jokers,
            // TODO: Do the hand type calculations before this and start with the hand's score.
            chips: Chips(1),
            mult: Mult(1),
        }
    }
}
