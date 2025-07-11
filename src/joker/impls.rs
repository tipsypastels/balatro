use super::{JokerKind, RunIndependentEvent};
use crate::{Money, Rarity};
use rand::Rng;

#[derive(Debug)]
pub struct JimboJoker;

impl JokerKind for JimboJoker {
    fn name(&self) -> &'static str {
        "Joker"
    }

    fn rarity(&self) -> Rarity {
        Rarity::Common
    }

    fn price(&self) -> Money {
        Money(2)
    }

    fn run_independent(&self, event: RunIndependentEvent) {
        event.score.mult += 4;
    }
}

#[derive(Debug)]
pub struct MisprintJoker;

impl JokerKind for MisprintJoker {
    fn name(&self) -> &'static str {
        "Misprint"
    }

    fn rarity(&self) -> Rarity {
        Rarity::Common
    }

    fn price(&self) -> Money {
        Money(4)
    }

    fn run_independent(&self, event: RunIndependentEvent) {
        event.score.mult += event.rng.random_range(0..=23);
    }
}

#[derive(Debug)]
pub struct StencilJoker;

impl JokerKind for StencilJoker {
    fn name(&self) -> &'static str {
        "Joker Stencil"
    }

    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }

    fn price(&self) -> Money {
        Money(8)
    }

    fn run_independent(&self, event: RunIndependentEvent) {}
}
