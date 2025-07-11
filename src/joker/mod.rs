use crate::{Edition, HasEdition, Money};
use std::{any::Any, fmt::Debug, rc::Rc};

mod event;
mod impls;
mod list;

pub use event::*;
pub use impls::*;
pub use list::*;

#[derive(Debug, Clone)]
pub struct Joker {
    // TODO: Add sync feature.
    kind: Rc<dyn JokerKind>,
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
            kind: Rc::new(kind),
            edition: None,
        }
    }

    pub fn builder(kind: impl JokerKind) -> JokerBuilder {
        JokerBuilder(Self::new(kind))
    }

    pub fn is<J: JokerKind>(&self) -> bool {
        let kind: &dyn Any = &*self.kind;
        kind.is::<J>()
    }

    pub fn name(&self) -> &'static str {
        self.kind.name()
    }

    pub fn rarity(&self) -> Rarity {
        self.kind.rarity()
    }

    pub fn price(&self) -> Money {
        // TODO: If rental, Money(1).
        self.kind.price()
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

    fn run_independent(&self, event: RunIndependentEvent) {
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
