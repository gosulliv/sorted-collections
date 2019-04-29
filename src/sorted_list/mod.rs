//! Module for a sorted list using multiple lists of varying length.
//!
//! Copied from Grant Jenks' sorted containers.

// TODO:
// make sure the index is never truly empty (should be [0] if empty).
//
// other invariants?

#[cfg(test)]
mod tests;

use super::sorted_utils;
use std::default::Default;
use std::iter::FromIterator;
use std::ops::{Index, IndexMut};

pub const DEFAULT_LOAD_FACTOR: usize = 1000;

#[derive(Debug)]
pub struct SortedList<T: Ord> {
    lists: Vec<Vec<T>>, // There is always at least one element in this list.
    load_factor: usize,
    len: usize,
}

/// A sorted list with no `unsafe` code.
///
/// It is a logic error for an item to be modified in such a way that the item's ordering relative
/// to any other item, as determined by the `Ord` trait, changes while it is in the heap. This is
/// normally only possible through `Cell`, `RefCell`, global state, I/O, or unsafe code.
impl<T: Ord> SortedList<T> {
    pub fn contains(&self, val: &T) -> bool {
        debug_assert!(!self.lists.is_empty());

        self.lists.iter().any(|list| list.contains(val))
    }

    pub fn add(&mut self, new_val: T) {
        let i_changed = sorted_utils::insert_list_of_lists(&mut self.lists, new_val);
        self.len += 1;
        self.expand(i_changed);
    }

    /// Splits sublists that are more than double the load level.
    /// Updates the index when the sublist length is less than double the load
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

    fn contract(&mut self, i: usize) {
        if self.lists.len() > 1 && self.lists[i].len() < self.load_factor / 2 {
            self.unchecked_contract(i)
        }
    }

    // TODO: this can make lists that are too big.
    /// Contracts with the nearest list.
    fn unchecked_contract(&mut self, i: usize) {
        debug_assert!(self.lists.len() > 1);
        let (low, high) = match i {
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
        };

        let mut removed_list = self.lists.remove(high);
        self.lists[low].append(&mut removed_list);
    }

    pub fn first(&self) -> Option<&T> {
        self.lists.first().and_then(|x| x.first())
    }

    /// Returns a reference to the last (maximum) value in the list.
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

    pub fn pop_last(&mut self) -> Option<T> {
        if let Some(rv) = self.lists.last_mut().and_then(|l| l.pop()) {
            self.len -= 1;
            let len = self.len;
            self.contract(len);
            Some(rv)
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn iter(&self) -> Iter<T> {
        let mut outer = self.lists.iter();
        let inner = outer.next().unwrap().iter();
        Iter { outer, inner }
    }
}

impl<T: Ord> Index<usize> for SortedList<T> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        let mut i = i;
        for list in &self.lists {
            if list.len() > i {
                return &list[i];
            } else {
                i = i - list.len();
            }
        }
        panic!("element greater than list size");
    }
}

impl<T: Ord> IndexMut<usize> for SortedList<T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        let mut i = i;
        for list in &mut self.lists {
            if list.len() > i {
                return &mut list[i];
            } else {
                i = i - list.len();
            }
        }
        panic!("element greater than list size");
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

impl<T: Ord> IntoIterator for SortedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            outer: self.lists.into_iter(),
            inner: Vec::new().into_iter(),
        }
    }
}

impl<T: Ord> Default for SortedList<T> {
    fn default() -> Self {
        SortedList::<T> {
            lists: vec![Vec::new()],
            load_factor: DEFAULT_LOAD_FACTOR,
            len: 0,
        }
    }
}

/// Does a probably O(n^2) collection from an iterator -- but it's an iterator, not a
/// collection we're sorting, so what do you expect?
///
/// Actually may not be that bad based on the performance analysis that's todo
impl<T: Ord> FromIterator<T> for SortedList<T> {
    fn from_iter<F>(iter: F) -> Self
    where
        F: IntoIterator<Item = T>,
    {
        let mut list = Self::default();
        let mut iter = iter.into_iter();
        while let Some(x) = iter.next() {
            list.add(x);
        }
        list
    }
}
