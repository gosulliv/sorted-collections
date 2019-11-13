//! Module for a sorted list using multiple lists of varying length.
//!
//! Adapted from Grant Jenks' sorted containers.
//!
//! # Example usage
//! ```
//! use sorted_collections::SortedList;
//! let mut list: SortedList<i32> = SortedList::new();
//! assert_eq!(0, list.len());
//!
//! list.add(3);
//!
//! assert!(list.contains(&3));
//! assert!(!list.contains(&13));
//! assert_eq!(Some(&3), list.first());
//! assert_eq!(Some(&3), list.last());
//!
//! list.add(13);
//!
//! assert_eq!(2, list.len());
//! assert!(list.contains(&3));
//! assert!(list.contains(&13));
//! assert!(!list.contains(&1));
//! ```

#[cfg(test)]
mod tests;

use super::{IntoIter, Iter};
use crate::sorted_utils::insert_list_of_lists;
use std::default::Default;
use std::iter::FromIterator;
use std::ops::{Index, IndexMut};
use unsorted_list::UnsortedList;

/// A sorted list with no `unsafe` code.
///
/// It is a logic error for an item to be modified in such a way that the item's ordering relative
/// to any other item, as determined by the `Ord` trait, changes while it is in the heap (similar
/// to the standard library collections). This is normally only possible through `Cell`, `RefCell`,
/// global state, I/O, or unsafe code.
#[derive(Debug)]
pub struct SortedList<T: Ord>(UnsortedList<T>);

impl<T: Ord> SortedList<T> {
    pub fn new() -> Self {
        Self(UnsortedList::new())
    }

    // TODO: test contract is called here.
    pub fn add(&mut self, new_val: T) {
        let list_index_changed = insert_list_of_lists(&mut self.0.lists, new_val);
        self.0.len += 1;
        self.0.contract(list_index_changed);
    }

    // TODO: this can be sped up by using knowledge of the list being sorted.
    pub fn contains(&self, val: &T) -> bool {
        self.0.contains(val)
    }

    pub fn first(&self) -> Option<&T> {
        self.0.first()
    }

    pub fn first_mut(&mut self) -> Option<&mut T> {
        self.0.first_mut()
    }

    pub fn last(&mut self) -> Option<&T> {
        self.0.last()
    }

    pub fn last_mut(&mut self) -> Option<&mut T> {
        self.0.last_mut()
    }

    pub fn pop_first(&mut self) -> Option<T> {
        self.0.pop_first()
    }

    pub fn pop_last(&mut self) -> Option<T> {
        self.0.pop_last()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> Iter<T> {
        self.0.iter()
    }
}

impl<T: Ord> Index<usize> for SortedList<T> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        self.0.index(i)
    }
}

impl<T: Ord> IndexMut<usize> for SortedList<T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        self.0.index_mut(i)
    }
}

impl<T: Ord> IntoIterator for SortedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        self.0.into_iter()
    }
}

impl<T: Ord> Default for SortedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a SortedList from an Iterator.
///
/// The runtime of this function should be approximately `O(n * log(n))`.
impl<T: Ord> FromIterator<T> for SortedList<T> {
    fn from_iter<F>(iter: F) -> Self
    where
        F: IntoIterator<Item = T>,
    {
        let mut list = Self::new();
        for x in iter {
            list.add(x);
        }
        list
    }
}
