use super::HandType;
use crate::{Chips, ChipsAllowMul, Mult};
use std::num::NonZero;

#[derive(Default, Debug)]
pub struct HandTypeStates {
    pub(super) high_card: InnerState,
    pub(super) pair: InnerState,
    pub(super) two_pair: InnerState,
    pub(super) three_of_a_kind: InnerState,
    pub(super) straight: InnerState,
    pub(super) flush: InnerState,
    pub(super) full_house: InnerState,
    pub(super) four_of_a_kind: InnerState,
    pub(super) straight_flush: InnerState,
    pub(super) five_of_a_kind: InnerState,
    pub(super) flush_house: InnerState,
    pub(super) flush_five: InnerState,
}

impl HandTypeStates {
    pub const fn get(&self, hand_type: HandType) -> HandTypeState {
        HandTypeState {
            hand_type,
            state: hand_type.get_state(self),
        }
    }

    pub const fn get_mut(&mut self, hand_type: HandType) -> HandTypeStateMut {
        HandTypeStateMut {
            hand_type,
            state: hand_type.get_state_mut(self),
        }
    }
}

#[derive(Debug)]
pub struct HandTypeState<'a> {
    hand_type: HandType,
    state: &'a InnerState,
}

impl HandTypeState<'_> {
    pub const fn hand_type(&self) -> HandType {
        self.hand_type
    }

    pub const fn is_unlocked(&self) -> bool {
        !self.hand_type.is_secret() || self.state.plays > 0
    }

    pub const fn level(&self) -> NonZero<u16> {
        self.state.level
    }

    pub const fn plays(&self) -> u16 {
        self.state.plays
    }

    pub fn score(&self) -> (Chips, Mult) {
        let (chips, mult) = self.hand_type.base_score();
        let (addl_chips, addl_mult) = self.hand_type.addl_score_per_level();
        let addl_times = self.state.level.get() as u64 - 1;

        (
            chips + (ChipsAllowMul::new(addl_chips) * addl_times).finish(),
            mult + (addl_mult * addl_times),
        )
    }
}

#[derive(Debug)]
pub struct HandTypeStateMut<'a> {
    hand_type: HandType,
    state: &'a mut InnerState,
}

impl HandTypeStateMut<'_> {
    pub const fn hand_type(&self) -> HandType {
        self.as_ref().hand_type()
    }

    pub const fn is_unlocked(&self) -> bool {
        self.as_ref().is_unlocked()
    }

    pub const fn level(&self) -> NonZero<u16> {
        self.as_ref().level()
    }

    pub const fn plays(&self) -> u16 {
        self.as_ref().plays()
    }

    pub fn score(&self) -> (Chips, Mult) {
        self.as_ref().score()
    }

    pub fn level_up(&mut self) {
        self.state.level = self.state.level.saturating_add(1);
    }

    pub fn plays_up(&mut self) {
        self.state.plays += 1;
    }

    const fn as_ref(&self) -> HandTypeState {
        HandTypeState {
            hand_type: self.hand_type,
            state: self.state,
        }
    }
}

#[derive(Debug)]
pub(super) struct InnerState {
    level: NonZero<u16>,
    plays: u16,
}

impl Default for InnerState {
    fn default() -> Self {
        Self {
            level: unsafe { NonZero::new_unchecked(1) },
            plays: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use HandType::*;

    #[test]
    fn base_score() {
        let states = HandTypeStates::default();
        assert_eq!(states.get(HighCard).score(), HighCard.base_score());
    }

    #[test]
    fn level_up_once() {
        let mut states = HandTypeStates::default();

        states.get_mut(HighCard).level_up();

        assert_eq!(states.get(HighCard).score(), (Chips(15), Mult(2)));
    }

    #[test]
    fn level_up_twice() {
        let mut states = HandTypeStates::default();

        states.get_mut(HighCard).level_up();
        states.get_mut(HighCard).level_up();

        assert_eq!(states.get(HighCard).score(), (Chips(25), Mult(3)));
    }

    #[test]
    fn unlocked_non_secret() {
        let states = HandTypeStates::default();

        assert!(states.get(HighCard).is_unlocked());
    }

    #[test]
    fn unlocked_secret() {
        let mut states = HandTypeStates::default();

        assert!(!states.get(FlushFive).is_unlocked());

        states.get_mut(FlushFive).plays_up();

        assert!(states.get(FlushFive).is_unlocked())
    }
}
