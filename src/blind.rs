use crate::Money;
use enum_assoc::Assoc;

#[derive(Assoc, Debug, Copy, Clone, PartialEq, Eq)]
#[func(pub const fn reward(self) -> Money)]
#[func(const fn score_mult(self) -> usize)]
pub enum Blind {
    #[assoc(reward = Money(3), score_mult = 2)]
    Small,
    #[assoc(reward = Money(4), score_mult = 3)]
    Big,
    #[assoc(reward = Money(5), score_mult = 5)]
    Boss(Boss),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Boss {
    Hook,
    Ox,
    House,
    Wall,
    Wheel,
    Arm,
    Club,
    Fish,
    Psychic,
    Goad,
    Water,
    Window,
    Manacle,
    Eye,
    Mouth,
    Plant,
    Serpent,
    Pillar,
    Needle,
    Head,
    Tooth,
    Flint,
    Mark,
}
