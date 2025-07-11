use super::*;
use im_rc::Vector;

#[derive(Debug, Clone)]
pub struct Slate<T: Clone> {
    vector: Vector<T>,
    base_cap: usize,
    neg_cnt: usize,
}

impl<T> Slate<T>
where
    T: HasEdition<Negative = ()> + Clone,
{
    pub fn new(base_cap: usize) -> Self {
        Self {
            vector: Vector::new(),
            base_cap,
            neg_cnt: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.vector.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.len() == self.cap()
    }

    pub fn len(&self) -> usize {
        self.vector.len()
    }

    pub fn free_len(&self) -> usize {
        self.cap() - self.len()
    }

    pub fn cap(&self) -> usize {
        self.base_cap + self.neg_cnt
    }

    pub fn base_cap(&self) -> usize {
        self.base_cap
    }

    pub fn push(&mut self, item: T) -> Result<(), T> {
        if item.is_negative() {
            self.vector.push_back(item);
            self.neg_cnt += 1;
            Ok(())
        } else if !self.is_full() {
            self.vector.push_back(item);
            Ok(())
        } else {
            Err(item)
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        let item = self.vector.remove(index);
        if item.is_negative() {
            self.neg_cnt -= 1;
        }
        item
    }

    pub fn iter(&self) -> SlateIter<T> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> SlateIterMut<T> {
        self.into_iter()
    }
}

impl<'a, T: Clone> IntoIterator for &'a Slate<T> {
    type Item = &'a T;
    type IntoIter = SlateIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        SlateIter(self.vector.iter())
    }
}

impl<'a, T: Clone> IntoIterator for &'a mut Slate<T> {
    type Item = &'a mut T;
    type IntoIter = SlateIterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        SlateIterMut(self.vector.iter_mut())
    }
}

impl<T: Clone> IntoIterator for Slate<T> {
    type Item = T;
    type IntoIter = SlateIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        SlateIntoIter(self.vector.into_iter())
    }
}

pub struct SlateIter<'a, T>(im_rc::vector::Iter<'a, T>);

impl<'a, T: Clone> Iterator for SlateIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub struct SlateIterMut<'a, T>(im_rc::vector::IterMut<'a, T>);

impl<'a, T: Clone> Iterator for SlateIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub struct SlateIntoIter<T>(im_rc::vector::ConsumingIter<T>);

impl<T: Clone> Iterator for SlateIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
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
        let mut slate = Slate::new(3);

        assert!(slate.push(X).is_ok());
        assert!(slate.push(X).is_ok());
        assert!(slate.push(X).is_ok());
        assert!(slate.is_full());

        assert_eq!(slate.len(), 3);
        assert_eq!(slate.cap(), 3);
    }

    #[test]
    fn some_negs_not_full() {
        let mut slate = Slate::new(3);

        assert!(slate.push(X).is_ok());
        assert!(slate.push(XNEG).is_ok());
        assert!(slate.push(X).is_ok());
        assert!(!slate.is_full());

        assert_eq!(slate.len(), 3);
        assert_eq!(slate.cap(), 4);
    }

    #[test]
    fn some_negs_full() {
        let mut slate = Slate::new(3);

        assert!(slate.push(X).is_ok());
        assert!(slate.push(XNEG).is_ok());
        assert!(slate.push(X).is_ok());
        assert!(slate.push(X).is_ok());
        assert!(slate.is_full());

        assert_eq!(slate.len(), 4);
        assert_eq!(slate.cap(), 4);
    }

    #[test]
    fn push_neg_after_full() {
        let mut slate = Slate::new(3);

        assert!(slate.push(X).is_ok());
        assert!(slate.push(X).is_ok());
        assert!(slate.push(X).is_ok());
        assert!(slate.is_full());

        assert!(slate.push(XNEG).is_ok());
        assert!(slate.is_full());
    }

    #[test]
    fn remove_negs_changing_cap() {
        let mut slate = Slate::new(3);

        assert!(slate.push(X).is_ok());
        assert!(slate.push(XNEG).is_ok());
        assert!(slate.push(X).is_ok());
        assert!(slate.push(X).is_ok());

        assert_eq!(slate.len(), 4);
        assert_eq!(slate.cap(), 4);

        assert!(slate.remove(1).is_negative());

        assert_eq!(slate.len(), 3);
        assert_eq!(slate.cap(), 3);
    }

    #[test]
    fn free_len() {
        let mut slate = Slate::new(3);

        assert_eq!(slate.free_len(), 3);

        slate.push(X).unwrap();
        slate.push(X).unwrap();

        assert_eq!(slate.free_len(), 1);

        slate.push(X).unwrap();

        assert_eq!(slate.free_len(), 0);
    }

    #[test]
    fn free_len_negs() {
        let mut slate = Slate::new(3);

        slate.push(XNEG).unwrap();

        assert_eq!(slate.free_len(), 3);

        slate.push(X).unwrap();

        assert_eq!(slate.free_len(), 2);
    }
}
