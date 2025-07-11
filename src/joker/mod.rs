use crate::{Edition, HasEdition, Money, ScoreBuilder};
use std::{any::Any, cell::RefCell, fmt::Debug, rc::Rc};

mod event;
mod jokers;

pub use event::*;
pub use jokers::*;

#[derive(Debug, Clone)]
pub struct Joker {
    // TODO: Add sync feature.
    kind: Rc<RefCell<dyn JokerKind>>,
    edition: Option<Edition<Self>>,
    // TODO: Add stickers.
}

impl HasEdition for Joker {
    type Scoring = ();
    type Negative = ();

    fn edition(&self) -> Option<Edition<Self>> {
        self.edition.as_ref().copied()
    }
}

impl Joker {
    pub fn new(kind: impl JokerKind) -> Self {
        Self {
            kind: Rc::new(RefCell::new(kind)),
            edition: None,
        }
    }

    pub fn builder(kind: impl JokerKind) -> JokerBuilder {
        JokerBuilder(Self::new(kind))
    }

    pub fn name(&self) -> &'static str {
        self.kind.borrow().name()
    }

    pub fn rarity(&self) -> Rarity {
        self.kind.borrow().rarity()
    }

    pub fn price(&self) -> Money {
        // TODO: If rental, Money(1).
        self.kind.borrow().price()
    }
}

#[derive(Debug, Clone)]
pub struct JokerBuilder(Joker);

impl JokerBuilder {
    pub fn edition(mut self, edition: Edition<Joker>) -> Self {
        self.0.edition = Some(edition);
        self
    }

    pub fn build(self) -> Joker {
        self.0
    }
}

pub trait JokerKind: Any + Debug {
    fn name(&self) -> &'static str;
    fn rarity(&self) -> Rarity;
    fn price(&self) -> Money;

    fn run_independent(&mut self, event: RunIndependentEvent) {
        let _ = event;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Legendary,
}
