use super::state::{HandTypeStates, InnerState};
use crate::{Chips, Mult};
use balatro_macros::Variants;
use enum_assoc::Assoc;

#[derive(Assoc, Variants, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[func(pub const fn base_score(self) -> (Chips, Mult))]
#[func(pub const fn addl_score_per_level(self) -> (Chips, Mult))]
#[func(pub(super) const fn get_state(self, states: &HandTypeStates) -> &InnerState)]
#[func(pub(super) const fn get_state_mut(self, states: &mut HandTypeStates) -> &mut InnerState)]
pub enum HandType {
    #[assoc(
        base_score = (Chips(5), Mult(1)),
        addl_score_per_level = (Chips(10), Mult(1)),
        get_state = &states.high_card,
        get_state_mut = &mut states.high_card,
    )]
    HighCard,
    #[assoc(
        base_score = (Chips(10), Mult(2)),
        addl_score_per_level = (Chips(10), Mult(2)),
        get_state = &states.pair,
        get_state_mut = &mut states.pair,
    )]
    Pair,
    #[assoc(
        base_score = (Chips(20), Mult(2)),
        addl_score_per_level = (Chips(20), Mult(1)),
        get_state = &states.two_pair,
        get_state_mut = &mut states.two_pair,
    )]
    TwoPair,
    #[assoc(
        base_score = (Chips(30), Mult(3)),
        addl_score_per_level = (Chips(20), Mult(2)),
        get_state = &states.three_of_a_kind,
        get_state_mut = &mut states.three_of_a_kind,
    )]
    ThreeOfAKind,
    #[assoc(
        base_score = (Chips(30), Mult(4)),
        addl_score_per_level = (Chips(30), Mult(3)),
        get_state = &states.straight,
        get_state_mut = &mut states.straight,
    )]
    Straight,
    #[assoc(
        base_score = (Chips(35), Mult(4)),
        addl_score_per_level = (Chips(15), Mult(2)),
        get_state = &states.flush,
        get_state_mut = &mut states.flush,
    )]
    Flush,
    #[assoc(
        base_score = (Chips(40), Mult(4)),
        addl_score_per_level = (Chips(25), Mult(2)),
        get_state = &states.full_house,
        get_state_mut = &mut states.full_house,
    )]
    FullHouse,
    #[assoc(
        base_score = (Chips(60), Mult(7)),
        addl_score_per_level = (Chips(30), Mult(3)),
        get_state = &states.four_of_a_kind,
        get_state_mut = &mut states.four_of_a_kind,
    )]
    FourOfAKind,
    #[assoc(
        base_score = (Chips(100), Mult(8)),
        addl_score_per_level = (Chips(40), Mult(4)),
        get_state = &states.straight_flush,
        get_state_mut = &mut states.straight_flush,
    )]
    StraightFlush,
    #[assoc(
        base_score = (Chips(120), Mult(12)),
        addl_score_per_level = (Chips(35), Mult(3)),
        get_state = &states.five_of_a_kind,
        get_state_mut = &mut states.five_of_a_kind,
    )]
    FiveOfAKind,
    #[assoc(
        base_score = (Chips(140), Mult(14)),
        addl_score_per_level = (Chips(40), Mult(4)),
        get_state = &states.flush_house,
        get_state_mut = &mut states.flush_house,
    )]
    FlushHouse,
    #[assoc(
        base_score = (Chips(160), Mult(16)),
        addl_score_per_level = (Chips(50), Mult(3)),
        get_state = &states.flush_five,
        get_state_mut = &mut states.flush_five,
    )]
    FlushFive,
}

impl HandType {
    pub const fn is_secret(self) -> bool {
        self as u8 >= Self::FiveOfAKind as u8
    }
}
