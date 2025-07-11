use super::*;

#[derive(Debug, Clone)]
pub(crate) struct ListWithNegatives<T: HasEdition<Negative = ()>> {
    pub vec: Vec<T>,
    pub base_cap: usize,
    pub neg_cnt: usize,
}

impl<T: HasEdition<Negative = ()>> ListWithNegatives<T> {
    pub fn new(base_cap: usize) -> Self {
        Self {
            vec: Vec::new(),
            base_cap,
            neg_cnt: 0,
        }
    }

    pub fn is_full(&self) -> bool {
        self.vec.len() == self.cap()
    }

    pub fn cap(&self) -> usize {
        self.base_cap + self.neg_cnt
    }

    pub fn push(&mut self, item: T) -> Result<(), T> {
        if item.is_negative() {
            self.vec.push(item);
            self.neg_cnt += 1;
            Ok(())
        } else if !self.is_full() {
            self.vec.push(item);
            Ok(())
        } else {
            Err(item)
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        let item = self.vec.remove(index);
        if item.is_negative() {
            self.neg_cnt -= 1;
        }
        item
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ListWithNegatives as List;

    struct Item(bool);

    impl HasEdition for Item {
        type Scoring = std::convert::Infallible;
        type Negative = ();

        fn edition(&self) -> Option<Edition<Self>> {
            self.0.then_some(Edition::Negative(()))
        }
    }

    const X: Item = Item(false);
    const XNEG: Item = Item(true);

    #[test]
    fn normal_only() {
        let mut list = List::new(3);

        assert!(list.push(X).is_ok());
        assert!(list.push(X).is_ok());
        assert!(list.push(X).is_ok());
        assert!(list.is_full());

        assert_eq!(list.vec.len(), 3);
        assert_eq!(list.cap(), 3);
    }

    #[test]
    fn some_negs_not_full() {
        let mut list = List::new(3);

        assert!(list.push(X).is_ok());
        assert!(list.push(XNEG).is_ok());
        assert!(list.push(X).is_ok());
        assert!(!list.is_full());

        assert_eq!(list.vec.len(), 3);
        assert_eq!(list.cap(), 4);
    }

    #[test]
    fn some_negs_full() {
        let mut list = List::new(3);

        assert!(list.push(X).is_ok());
        assert!(list.push(XNEG).is_ok());
        assert!(list.push(X).is_ok());
        assert!(list.push(X).is_ok());
        assert!(list.is_full());

        assert_eq!(list.vec.len(), 4);
        assert_eq!(list.cap(), 4);
    }

    #[test]
    fn push_neg_after_full() {
        let mut list = List::new(3);

        assert!(list.push(X).is_ok());
        assert!(list.push(X).is_ok());
        assert!(list.push(X).is_ok());
        assert!(list.is_full());

        assert!(list.push(XNEG).is_ok());
        assert!(list.is_full());
    }

    #[test]
    fn remove_negs_changing_cap() {
        let mut list = List::new(3);

        assert!(list.push(X).is_ok());
        assert!(list.push(XNEG).is_ok());
        assert!(list.push(X).is_ok());
        assert!(list.push(X).is_ok());

        assert_eq!(list.vec.len(), 4);
        assert_eq!(list.cap(), 4);

        assert!(list.remove(1).is_negative());

        assert_eq!(list.vec.len(), 3);
        assert_eq!(list.cap(), 3);
    }
}
