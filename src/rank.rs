use enum_assoc::Assoc;

#[derive(Assoc, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
