use super::prelude::*;

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

    fn run_independent(&self, scorer: &mut Scorer) {
        scorer.mult += 4;
    }
}
