use crate::{Chips, Mult};
use balatro_macros::Variants;
use enum_assoc::Assoc;

#[derive(Assoc, Variants, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[func(pub const fn base_score(self) -> (Chips, Mult))]
#[func(pub const fn addl_score_per_level(self) -> (Chips, Mult))]
pub enum HandType {
    #[assoc(
        base_score = (Chips(5), Mult(1)),
        addl_score_per_level = (Chips(10), Mult(1)),
    )]
    HighCard,
    #[assoc(
        base_score = (Chips(10), Mult(2)),
        addl_score_per_level = (Chips(10), Mult(2)),
    )]
    Pair,
    #[assoc(
        base_score = (Chips(20), Mult(2)),
        addl_score_per_level = (Chips(20), Mult(1)),
    )]
    TwoPair,
    #[assoc(
        base_score = (Chips(30), Mult(3)),
        addl_score_per_level = (Chips(20), Mult(2)),
    )]
    ThreeOfAKind,
    #[assoc(
        base_score = (Chips(30), Mult(4)),
        addl_score_per_level = (Chips(30), Mult(3)),
    )]
    Straight,
    #[assoc(
        base_score = (Chips(35), Mult(4)),
        addl_score_per_level = (Chips(15), Mult(2)),
    )]
    Flush,
    #[assoc(
        base_score = (Chips(40), Mult(4)),
        addl_score_per_level = (Chips(25), Mult(2)),
    )]
    FullHouse,
    #[assoc(
        base_score = (Chips(60), Mult(7)),
        addl_score_per_level = (Chips(30), Mult(3)),
    )]
    FourOfAKind,
    #[assoc(
        base_score = (Chips(100), Mult(8)),
        addl_score_per_level = (Chips(40), Mult(4)),
    )]
    StraightFlush,
    #[assoc(
        base_score = (Chips(120), Mult(12)),
        addl_score_per_level = (Chips(35), Mult(3)),
    )]
    FiveOfAKind,
    #[assoc(
        base_score = (Chips(140), Mult(14)),
        addl_score_per_level = (Chips(40), Mult(4)),
    )]
    FlushHouse,
    #[assoc(
        base_score = (Chips(160), Mult(16)),
        addl_score_per_level = (Chips(50), Mult(3)),
    )]
    FlushFive,
}

impl HandType {
    pub const fn is_secret(self) -> bool {
        self as u8 >= Self::FiveOfAKind as u8
    }
}
