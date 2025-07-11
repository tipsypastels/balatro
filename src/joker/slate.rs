use super::{Joker, JokerKind};
use crate::Slate;

pub trait JokerSlateExt {
    fn kind<J: JokerKind>(&self) -> impl Iterator<Item = Joker>;
    fn has_kind<J: JokerKind>(&self) -> bool;
}

impl JokerSlateExt for Slate<Joker> {
    fn kind<J: JokerKind>(&self) -> impl Iterator<Item = Joker> {
        self.iter().filter(|joker| joker.is::<J>()).cloned()
    }

    fn has_kind<J: JokerKind>(&self) -> bool {
        self.kind::<J>().next().is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn finding_jokers_by_type_id() {
        let mut slate = Slate::<Joker>::new(1);

        slate.push(Joker::new(JimboJoker)).unwrap();

        assert!(slate.has_kind::<JimboJoker>());
        assert_eq!(slate.kind::<JimboJoker>().collect::<Vec<_>>().len(), 1);

        assert!(!slate.has_kind::<StencilJoker>());
        assert_eq!(slate.kind::<StencilJoker>().collect::<Vec<_>>().len(), 0);
    }
}
