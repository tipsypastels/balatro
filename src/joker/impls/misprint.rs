use super::prelude::*;

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

    fn run_independent(&self, scorer: &mut Scorer) {
        scorer.mult += rand::rng().random_range(0..=23);
    }
}
