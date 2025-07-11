use crate::{Edition, HasEdition, Rank, Suit};
use std::convert::Infallible;

#[derive(Debug, Clone)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
    pub enhancement: Option<Enhancement>,
    pub edition: Option<Edition<Self>>,
    pub seal: Option<Seal>,
}

impl HasEdition for Card {
    type Scoring = ();
    type Negative = Infallible;

    fn edition(&self) -> Option<Edition<Self>> {
        self.edition.as_ref().copied()
    }
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Self {
            rank,
            suit,
            enhancement: None,
            edition: None,
            seal: None,
        }
    }

    pub fn builder(rank: Rank, suit: Suit) -> CardBuilder {
        CardBuilder(Self::new(rank, suit))
    }
}

#[derive(Debug, Clone)]
pub struct CardBuilder(Card);

impl CardBuilder {
    pub fn enhancement(mut self, enhancement: Enhancement) -> Self {
        self.0.enhancement = Some(enhancement);
        self
    }

    pub fn edition(mut self, edition: Edition<Card>) -> Self {
        self.0.edition = Some(edition);
        self
    }

    pub fn seal(mut self, seal: Seal) -> Self {
        self.0.seal = Some(seal);
        self
    }

    pub fn build(self) -> Card {
        self.0
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Enhancement {
    Bonus,
    Mult,
    Wild,
    Glass,
    Steel,
    Stone,
    Gold,
    Lucky,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Seal {
    Gold,
    Red,
    Blue,
    Purple,
}
