use super::HandType;
use crate::{Chips, ChipsAllowMul, Mult, Planet};
use std::num::NonZero;

macro_rules! states {
    ($($field:ident: $member:ident),*$(,)?) => {
        #[derive(Debug, Default, Clone)]
        pub struct HandTypeStates {
            $($field: InnerState),*
        }

        impl HandTypeStates {
            $(
                pub const fn $field(&self) -> HandTypeState {
                    self.get(HandType::$member)
                }
            )*

            const fn get_state(&self, hand_type: HandType) -> InnerState {
                match hand_type {
                    $(HandType::$member => { self.$field })*
                }
            }

            const fn get_state_mut(&mut self, hand_type: HandType) -> &mut InnerState {
                match hand_type {
                    $(HandType::$member => { &mut self.$field })*
                }
            }
        }
    };
}

states! {
    high_card: HighCard,
    pair: Pair,
    two_pair: TwoPair,
    three_of_a_kind: ThreeOfAKind,
    straight: Straight,
    flush: Flush,
    full_house: FullHouse,
    four_of_a_kind: FourOfAKind,
    straight_flush: StraightFlush,
    five_of_a_kind: FiveOfAKind,
    flush_house: FlushHouse,
    flush_five: FlushFive,
}

impl HandTypeStates {
    pub const fn get(&self, hand_type: HandType) -> HandTypeState {
        HandTypeState {
            hand_type,
            inner: self.get_state(hand_type),
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
        let old_state = self.get_state(hand_type);
        let new_state = f(old_state);

        let mut this = self.clone();
        *this.get_state_mut(hand_type) = new_state;
        this
    }

    fn update_all(&self, f: impl Fn(InnerState) -> InnerState) -> Self {
        let mut this = self.clone();

        for hand_type in HandType::variants() {
            let old_state = self.get_state(hand_type);
            let new_state = f(old_state);

            *this.get_state_mut(hand_type) = new_state;
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

#[derive(Debug, Clone, Copy)]
struct InnerState {
    level: NonZero<u16>,
    plays: u16,
}

impl InnerState {
    fn level_up(self) -> Self {
        let mut this = self;
        this.level = this.level.saturating_add(1);
        this
    }

    fn plays_up(self) -> Self {
        let mut this = self;
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
    fn getters_return_same_hand_type() {
        let states = HandTypeStates::default();

        for hand_type in HandType::variants() {
            assert_eq!(states.get(hand_type).hand_type(), hand_type);
        }
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
    fn increasing_plays() {
        let states = HandTypeStates::default();

        assert_eq!(states.get(HighCard).plays(), 0);

        let states = states.plays_up(HighCard);

        assert_eq!(states.get(HighCard).plays(), 1);
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
