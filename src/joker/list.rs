use super::{Joker, JokerKind};
use crate::ListWithNegatives;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct JokerList(ListWithNegatives<Joker>);

impl JokerList {
    pub fn new(base_cap: usize) -> Self {
        Self(ListWithNegatives::new(base_cap))
    }

    pub fn kind<J: JokerKind>(&self) -> impl Iterator<Item = Joker> {
        self.iter().filter(|joker| joker.is::<J>()).cloned()
    }

    pub fn has_kind<J: JokerKind>(&self) -> bool {
        self.kind::<J>().next().is_some()
    }

    pub fn is_full(&self) -> bool {
        self.0.is_full()
    }

    pub fn base_cap(&self) -> usize {
        self.0.base_cap
    }

    pub fn cap(&self) -> usize {
        self.0.cap()
    }

    pub fn push(&mut self, joker: Joker) -> Result<(), Joker> {
        self.0.push(joker)
    }

    pub fn remove(&mut self, index: usize) -> Joker {
        self.0.remove(index)
    }
}

impl Deref for JokerList {
    type Target = [Joker];

    fn deref(&self) -> &Self::Target {
        &self.0.vec
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn finding_jokers_by_type_id() {
        let mut list = JokerList::new(1);

        list.push(Joker::new(JimboJoker)).unwrap();

        assert!(list.has_kind::<JimboJoker>());
        assert_eq!(list.kind::<JimboJoker>().collect::<Vec<_>>().len(), 1);

        assert!(!list.has_kind::<StencilJoker>());
        assert_eq!(list.kind::<StencilJoker>().collect::<Vec<_>>().len(), 0);
    }
}
