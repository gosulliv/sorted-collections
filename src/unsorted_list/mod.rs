//! Module for a sorted list using multiple lists of varying length.
//!
//! This package may be useful for those who are working with large lists, as it adds a little
//! overhead to small lists but is significantly faster than a vector for large lists.
//!
//! # Example Usage
//! ```
//! use sorted_collections::UnsortedList;
//! let mut list: UnsortedList<i64> = UnsortedList::new();
//! assert_eq!(0, list.len());
//!
//! list.push(3);
//! list.push(-22);
//! list.push(11);
//!
//! assert_eq!(vec![3,-22,11], list.into_iter().collect::<Vec<i64>>());
//! ```

use super::sorted_utils::DEFAULT_LOAD_FACTOR;
use std::default::Default;
use std::iter::FromIterator;
use std::ops::{Index, IndexMut};

/// An unsorted list.
/// Usage is about the same as a vector.
/// Performance should be better for large lists.
#[derive(Debug)]
pub struct UnsortedList<T> {
    lists: Vec<Vec<T>>, // There is always at least one element in the outer list.
    load_factor: usize,
    len: usize,
}

impl<T> UnsortedList<T> {
    pub fn new() -> Self {
        Self {
            lists: vec![Vec::new()],
            load_factor: DEFAULT_LOAD_FACTOR,
            len: 0,
        }
    }

    pub fn insert(&mut self, mut i: usize, element: T) {
        let mut outer = 0;
        // biases towards the earlier list.
        while i > self.lists[outer].len() {
            i -= self.lists[outer].len();
            outer += 1;
        }

        self.lists[outer].insert(i, element);
        self.len += 1;
        self.expand(outer);
    }

    /// Splits sublists that are more than double the load level.
    /// Updates the i when the sublist length is less than double the load
    /// level. This requires incrementing the nodes in a traversal from the
    /// leaf node to the root. For an example traversal see self._loc.
    fn expand(&mut self, i: usize) {
        // >= because otherwise contract can fail... better solution for this?
        if self.lists[i].len() >= 2 * self.load_factor {
            self.unchecked_expand(i)
        }
    }

    fn unchecked_expand(&mut self, i: usize) {
        let new_list = {
            let inner = &mut self.lists[i];
            let mid = inner.len() / 2;
            inner.split_off(mid)
        };

        self.lists.insert(i + 1, new_list);
    }

    // TODO: this can make lists that are too big.
    fn contract(&mut self, i: usize) {
        if self.lists.len() > 1 && self.lists[i].len() < self.load_factor / 2 {
            self.unchecked_contract(i)
        }
    }

    /// Contracts with the nearest list.
    fn unchecked_contract(&mut self, i: usize) {
        debug_assert!(self.len() > 1);
        let (low, high) = self.contract_i(i);
        let mut removed_list = self.lists.remove(high);
        self.lists[low].append(&mut removed_list);
    }

    fn contract_i(&self, i: usize) -> (usize, usize) {
        match i {
            0 => (0, 1),
            i if i == self.lists.len() => (self.lists.len() - 2, self.lists.len() - 1),
            i => {
                let other_list: usize = if self.lists[i - 1].len() < self.lists[i + 1].len() {
                    i - 1
                } else {
                    i + 1
                };
                if i < other_list {
                    (i, other_list)
                } else {
                    (other_list, i)
                }
            }
        }
    }
    pub fn first(&self) -> Option<&T> {
        self.lists.first().and_then(|x| x.first())
    }

    pub fn first_mut(&mut self) -> Option<&mut T> {
        self.lists.first_mut().and_then(|x| x.first_mut())
    }

    pub fn last(&mut self) -> Option<&T> {
        self.lists.last().and_then(|x| x.last())
    }

    pub fn last_mut(&mut self) -> Option<&mut T> {
        self.lists.last_mut().and_then(|x| x.last_mut())
    }

    pub fn pop_first(&mut self) -> Option<T> {
        if self.len() == 0 {
            None
        } else {
            self.len -= 1;
            let rv = Some(self.lists[0].remove(0));
            self.contract(0);
            rv
        }
    }

    pub fn push(&mut self, element: T) {
        self.lists.last_mut().unwrap().push(element);
        self.len += 1;
        let len = self.lists.len();
        // FIXME catch with test?
        self.contract(len);
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(rv) = self.lists.last_mut().and_then(|l| l.pop()) {
            self.len -= 1;
            let len = self.lists.len();
            self.contract(len);
            Some(rv)
        } else {
            None
        }
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn iter(&self) -> Iter<T> {
        let mut outer = self.lists.iter();
        let inner = outer.next().unwrap().iter();
        Iter { outer, inner }
    }

    #[inline]
    fn indices(&self, mut i: usize) -> (usize, usize) {
        let mut outer = 0;

        // biases towards the earlier list.
        while i > self.lists[outer].len() {
            i -= self.lists[outer].len();
            outer += 1;
        }
        (outer, i)
    }
}

impl<T: PartialEq> UnsortedList<T> {
    pub fn contains(&self, val: &T) -> bool {
        debug_assert!(!self.lists.is_empty());

        self.lists.iter().any(|list| list.contains(val))
    }
}

pub struct Iter<'a, T: 'a> {
    outer: std::slice::Iter<'a, Vec<T>>,
    inner: std::slice::Iter<'a, T>,
}

impl<'a, T: Ord> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().or_else(|| {
            self.outer.next().and_then(|x| {
                self.inner = x.into_iter();
                self.next()
            })
        })
    }
}

pub struct IntoIter<T> {
    outer: std::vec::IntoIter<Vec<T>>,
    inner: std::vec::IntoIter<T>,
}

impl<T: Ord> Iterator for IntoIter<T> {
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
        let (min, _) = self.inner.size_hint();
        (min, None)
    }
}

impl<T: Ord> IntoIterator for UnsortedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            outer: self.lists.into_iter(),
            inner: Vec::new().into_iter(),
        }
    }
}

impl<T: Ord> Default for UnsortedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Does a probably O(n^2) collection from an iterator -- but it's an iterator, not a
/// collection we're sorting, so what do you expect?
///
/// Actually may not be that bad based on the performance analysis that's todo
impl<'a, T: Ord> FromIterator<T> for UnsortedList<T> {
    fn from_iter<F>(iter: F) -> Self
    where
        F: IntoIterator<Item = T>,
    {
        let mut list = Self::default();
        let mut iter = iter.into_iter();
        while let Some(x) = iter.next() {
            list.push(x);
        }
        list
    }
}

impl<T: Ord> Index<usize> for UnsortedList<T> {
    type Output = T;
    fn index(&self, i: usize) -> &T {
        let (i, j) = self.indices(i);
        &self.lists[i][j]
    }
}

impl<T: Ord> IndexMut<usize> for UnsortedList<T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        let (i, j) = self.indices(i);
        &mut self.lists[i][j]
    }
}

#[cfg(test)]
mod tests;
