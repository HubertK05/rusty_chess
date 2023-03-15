use std::collections::HashSet;
use std::hash::Hash;

pub trait ContainsFrom<T> {
    fn contains_from(&self, elems: &[T]) -> bool;
}

impl<T: Eq + Hash> ContainsFrom<T> for HashSet<T> {
    fn contains_from(&self, elems: &[T]) -> bool {
        for elem in elems {
            if self.contains(elem) {
                return true
            }
        }
        return false
    }
}
