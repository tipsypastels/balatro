use balatro_macros::Variants;
use enum_assoc::Assoc;

#[derive(Assoc, Variants, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[func(pub const fn score(self) -> u64)]
pub enum Rank {
    #[assoc(score = 2)]
    Two,
    #[assoc(score = 3)]
    Three,
    #[assoc(score = 4)]
    Four,
    #[assoc(score = 5)]
    Five,
    #[assoc(score = 6)]
    Six,
    #[assoc(score = 7)]
    Seven,
    #[assoc(score = 8)]
    Eight,
    #[assoc(score = 9)]
    Nine,
    #[assoc(score = 10)]
    Ten,
    #[assoc(score = 10)]
    Jack,
    #[assoc(score = 10)]
    Queen,
    #[assoc(score = 10)]
    King,
    #[assoc(score = 11)]
    Ace,
}

impl Rank {
    pub const fn is_face(self) -> bool {
        matches!(self, Self::Jack | Self::Queen | Self::King)
    }

    pub const fn is_odd(self) -> bool {
        !self.score().is_multiple_of(2)
    }

    pub const fn is_even(self) -> bool {
        !self.is_face() && !self.is_odd()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Rank::*;

    #[test]
    fn face() {
        for rank in Rank::variants() {
            assert_eq!(rank.is_face(), matches!(rank, Jack | Queen | King));
        }
    }

    #[test]
    fn odd() {
        for rank in Rank::variants() {
            assert_eq!(
                rank.is_odd(),
                matches!(rank, Three | Five | Seven | Nine | Ace),
            );
        }
    }

    #[test]
    fn even() {
        for rank in Rank::variants() {
            assert_eq!(
                rank.is_even(),
                matches!(rank, Two | Four | Six | Eight | Ten),
            )
        }
    }
}
