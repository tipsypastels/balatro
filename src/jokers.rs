use crate::{JokerKind, Money, Rarity, ScoreBuilder};

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

    fn run_independent(&mut self, score: &mut ScoreBuilder) {
        score.mult *= 4;
    }
}
