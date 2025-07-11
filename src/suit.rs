use enum_assoc::Assoc;

#[derive(Assoc, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[func(pub const fn family(self) -> SuitFamily)]
pub enum Suit {
    #[assoc(family = SuitFamily::Black)]
    Club,
    #[assoc(family = SuitFamily::Red)]
    Diamond,
    #[assoc(family = SuitFamily::Red)]
    Heart,
    #[assoc(family = SuitFamily::Black)]
    Spade,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SuitFamily {
    Red,
    Black,
}
