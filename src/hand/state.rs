use super::HandType;
use crate::{Chips, ChipsAllowMul, Mult, Planet};
use std::num::NonZero;

#[derive(Default, Debug, Clone)]
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
    pub fn get(&self, hand_type: HandType) -> HandTypeState {
        HandTypeState {
            hand_type,
            inner: hand_type.get_state(self).clone(),
        }
    }

    pub fn use_planet(&self, planet: Planet) -> Self {
        self.level_up(planet.hand_type())
    }

    pub fn use_black_hole(&self) -> Self {
        self.update_all(InnerState::level_up)
    }

    pub fn level_up(&self, hand_type: HandType) -> Self {
        self.update(hand_type, InnerState::level_up)
    }

    pub fn plays_up(&self, hand_type: HandType) -> Self {
        self.update(hand_type, InnerState::plays_up)
    }

    fn update(&self, hand_type: HandType, f: impl FnOnce(InnerState) -> InnerState) -> Self {
        let old_state = hand_type.get_state(self).clone();
        let new_state = f(old_state);

        let mut this = self.clone();
        *hand_type.get_state_mut(&mut this) = new_state;
        this
    }

    fn update_all(&self, f: impl Fn(InnerState) -> InnerState) -> Self {
        let mut this = self.clone();

        for hand_type in HandType::variants() {
            let old_state = hand_type.get_state(self).clone();
            let new_state = f(old_state);

            *hand_type.get_state_mut(&mut this) = new_state;
        }

        this
    }
}

#[derive(Debug, Clone)]
pub struct HandTypeState {
    hand_type: HandType,
    inner: InnerState,
}

impl HandTypeState {
    pub const fn hand_type(self) -> HandType {
        self.hand_type
    }

    pub const fn is_unlocked(self) -> bool {
        !self.hand_type.is_secret() || self.inner.plays > 0
    }

    pub const fn level(self) -> NonZero<u16> {
        self.inner.level
    }

    pub const fn plays(self) -> u16 {
        self.inner.plays
    }

    pub fn score(self) -> (Chips, Mult) {
        let (chips, mult) = self.hand_type.base_score();
        let (addl_chips, addl_mult) = self.hand_type.addl_score_per_level();
        let addl_times = self.inner.level.get() as u64 - 1;

        (
            chips + (ChipsAllowMul::new(addl_chips) * addl_times).finish(),
            mult + (addl_mult * addl_times),
        )
    }
}

#[derive(Debug, Clone)]
pub(super) struct InnerState {
    level: NonZero<u16>,
    plays: u16,
}

impl InnerState {
    fn level_up(self) -> Self {
        let mut this = self.clone();
        this.level = this.level.saturating_add(1);
        this
    }

    fn plays_up(self) -> Self {
        let mut this = self.clone();
        this.plays += 1;
        this
    }
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

    macro_rules! assert_levels {
        ($states:ident, { $($hand_type:ident => $level:literal),*$(,)? }) => {
            for hand_type in HandType::variants() {
                let expected = match hand_type {
                    $($hand_type =>  { $level })*
                    _ => 1,
                };

                assert_eq!($states.get(hand_type).level().get(), expected);
            }
        };
    }

    macro_rules! assert_score {
        ($states:ident, { $($hand_type:ident => ($chips:literal, $mult:literal ))*$(,)? }) => {
            for hand_type in HandType::variants() {
                let expected = match hand_type {
                    $($hand_type => { (Chips($chips), Mult($mult)) })*
                    _ => hand_type.base_score(),
                };

                assert_eq!($states.get(hand_type).score(), expected);
            }
        };
    }

    #[test]
    fn base_score() {
        let states = HandTypeStates::default();

        assert_levels!(states, {});
        assert_score!(states, {});
    }

    #[test]
    fn level_up_once() {
        let states = HandTypeStates::default().level_up(HighCard);

        assert_levels!(states, { HighCard => 2 });
        assert_score!(states, { HighCard => (15, 2)});
    }

    #[test]
    fn level_up_twice() {
        let states = HandTypeStates::default()
            .level_up(HighCard)
            .level_up(HighCard);

        assert_levels!(states, { HighCard => 3 });
        assert_score!(states, { HighCard => (25, 3)});
    }

    #[test]
    fn use_planet() {
        let states = HandTypeStates::default().use_planet(Planet::Pluto);

        assert_levels!(states, { HighCard => 2 });
        assert_score!(states, { HighCard => (15, 2)});
    }

    #[test]
    fn use_black_hole() {
        let states = HandTypeStates::default().use_black_hole();

        for hand_type in HandType::variants() {
            assert_eq!(states.get(hand_type).level().get(), 2);
        }
    }

    #[test]
    fn unlocked_non_secret() {
        let states = HandTypeStates::default();

        assert!(states.get(HighCard).is_unlocked());
    }

    #[test]
    fn unlocked_secret() {
        let states = HandTypeStates::default();

        assert!(!states.get(FlushFive).is_unlocked());

        let states = states.plays_up(FlushFive);

        assert!(states.get(FlushFive).is_unlocked())
    }
}
