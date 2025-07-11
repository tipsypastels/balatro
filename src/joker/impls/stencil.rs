use super::prelude::*;

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

    fn run_independent(&self, scorer: &mut Scorer) {
        let empty = scorer.jokers.free_len();
        let stencils = scorer.jokers.kind::<Self>().count();
        let mult = empty + stencils;

        scorer.mult *= mult as u64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use StencilJoker as This;

    #[test]
    fn only_joker() {
        let jokers = jokers![5: This];
        let mut scorer = Scorer::new(jokers);

        This.run_independent(&mut scorer);

        assert_eq!(scorer.mult, Mult(5));
    }
}
