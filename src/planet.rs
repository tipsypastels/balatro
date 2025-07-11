use crate::HandType;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Planet(HandType);

macro_rules! planets {
    ($($planet:ident => $type:ident),*$(,)?) => {
        #[allow(non_upper_case_globals)]
        impl Planet {
            $(pub const $planet: Self = Self(HandType::$type);)*
        }
    };
}

planets! {
    Pluto => HighCard,
    Mercury => Pair,
    Uranus => TwoPair,
    Venus => ThreeOfAKind,
    Saturn => Straight,
    Jupiter => Flush,
    Earth => FullHouse,
    Mars => FourOfAKind,
    Neptune => StraightFlush,
    PlanetX => FiveOfAKind,
    Ceres => FlushHouse,
    Eris => FlushFive,
}

impl Planet {
    pub fn variants() -> impl Iterator<Item = Self> {
        HandType::variants().map(Self)
    }

    pub const fn hand_type(self) -> HandType {
        self.0
    }
}
