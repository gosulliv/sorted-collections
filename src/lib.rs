//! Expandable, hopefully reasonably-cache friendly list types written entirely in safe Rustvisibility.

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

pub mod sorted_list;
mod sorted_utils;
pub mod unsorted_list;

pub use sorted_list::SortedList;
pub use unsorted_list::UnsortedList;

use std::iter::FusedIterator;

// Iterators live here so that their members can be private and they can be shared between lists.

pub struct Iter<'a, T: 'a> {
    outer: std::slice::Iter<'a, Vec<T>>,
    inner: std::slice::Iter<'a, T>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().or_else(|| {
            self.outer.next().and_then(|x| {
                self.inner = x.iter();
                self.next()
            })
        })
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.inner.len() + self.outer.len(), None)
    }
}
impl<'a, T> FusedIterator for Iter<'a, T> {}

pub struct IntoIter<T> {
    outer: std::vec::IntoIter<Vec<T>>,
    inner: std::vec::IntoIter<T>,
}
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().or_else(|| {
            self.outer.next().and_then(|x| {
                self.inner = x.into_iter();
                self.next()
            })
        })
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.inner.len() + self.outer.len(), None)
    }
}
impl<'a, T> FusedIterator for IntoIter<T> {}

#[cfg(test)]
mod tests {
    // no tests yet.
    // Could use some proptests for size_hint.
}
