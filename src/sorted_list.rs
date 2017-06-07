//! Module for a sorted list using multiple lists of varying length.
//!
//! Copied from Grant Jenks' sorted containers.

// TODO:
// make sure the index is never truly empty (should be [0] if empty).
//
// other invariants?

use bisect::*;
use jenks_index::JenksIndex;
use std::default::Default;
use std::iter::FromIterator;
use std::collections::binary_heap::BinaryHeap;

/// if the list size grows greater than the load factor, we split it.
/// If the list size shrinks below the load factor, we join two lists.
const DEFAULT_LOAD_FACTOR: usize = 1000;

// todo: make not copy.
#[derive(Debug)]
struct SortedList<T: Ord> {
    // XXX : why do I have to specify a lifetime for T here?
    // It's for maxes... But how do I specify that they will be refs into value_lists?
    value_lists: Vec<Vec<T>>, // There is always at least one vec in this list.
    //maxes: Vec<DoubleIndex>,
    index: JenksIndex,
    load_factor: usize,
    twice_load_factor: usize, // cached for speed I guess?
}

/// The sorted list you've all been waiting for.
///
/// It is a logic error for an item to be modified in such a way that the item's ordering relative
/// to any other item, as determined by the `Ord` trait, changes while it is in the heap. This is
/// normally only possible through `Cell`, `RefCell`, global state, I/O, or unsafe code.
impl<'a, T: Ord> SortedList<T> {
    fn update_jenks_index(&mut self) {
        self.index = JenksIndex::from_value_lists(&self.value_lists);
    }

    pub fn contains(&self, val: &T) -> bool {
        // TODO make this faster?
        self.value_lists.iter().find(|list| list.last() >= val && list.contains(val))
    }

    fn insert(&mut self, x: T) {
        assert!(!self.value_lists.is_empty());

        let mut iter = self.value_lists.iter_mut().enumerate();

        // unwrap is safe because the list is not empty. But the initial is kind of garbage.
        let (list_idx, &mut list) = find_or_last(iter, |(idx, list)| list.last() >= *x);

        insort_left(list, x);
        self.expand(list_idx);
    }

    /// Splits sublists that are more than double the load level.
    /// Updates the index when the sublist length is less than double the load
    /// level. This requires incrementing the nodes in a traversal from the
    /// leaf node to the root. For an example traversal see self._loc.
    fn expand(&mut self, pos: usize) {
        if self.value_lists[pos].len() > self.twice_load_factor {
            self.split_sublist(pos);
            // TODO: update index better.
            self.update_jenks_index();
        } else {
            // TODO
            //self.index.increment_above_leaf(pos);
            self.update_jenks_index();
        }
    }

    /// Assumes the list is not empty.
    fn split_sublist(&mut self, pos: usize) {
        let new_list = self.value_lists[pos].split_off(self.load_factor);
        self.value_lists.insert(pos + 1, new_list);
    }

    pub fn first(&self) -> Option<&T> {
        self.value_lists.first().and_then(|l| l.first())
    }

    /// Returns a reference to the last (maximum) value in the list.
    pub fn last(&mut self) -> Option<&T> {
        self.value_lists.last().and_then(|l| l.last())
    }

    pub fn last_mut(&mut self) -> Option<&mut T> {
        self.value_lists.last().and_then(|l| l.last_mut())
    }

    pub fn pop_first(&mut self) -> Option<T> {
        if self.len() == 0 {
            return None
        }

        let rv = Some(self.value_lists.first_mut().unwrap().remove(0));
        self.update_jenks_index();
        rv
    }

    pub fn pop_last(&mut self) -> Option<T> {
        let rv = self.value_lists.last_mut().and_then(|l| l.pop());
        // TODO: expand?
        self.update_jenks_index();
        rv
    }

    pub fn len(&self) -> usize {
        return self.index.head();
    }
}

pub struct Iter<'a, T: 'a> {
    list_list_iter: ::std::slice::Iter<'a, Vec<T>>,
    curr_list_iter: ::std::slice::Iter<'a, T>,
}

impl<'a, T: Ord> SortedList<T> {
    pub fn iter(&self) -> Iter<T> {
        let mut ll_iter = self.value_lists.iter();
        let cl_iter = ll_iter.next().unwrap().iter();
        Iter {
            list_list_iter: ll_iter,
            curr_list_iter: cl_iter,
        }
    }
}

impl<'a, T: Ord> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.curr_list_iter.next().or_else(|| {
            self.list_list_iter.next().and_then(|x| {
                self.curr_list_iter = x.into_iter();
                self.next()
            })
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (ll_min, ll_max) = self.list_list_iter.size_hint();
        let (cl_min, cl_max) = self.curr_list_iter.size_hint();
        (ll_min + cl_min,
         match (ll_max, cl_max) {
            (Some(x), Some(y)) => Some(x + y),
            _ => None,
        })
    }
}

pub struct IntoIter<T> {
    list_list_iter: ::std::vec::IntoIter<Vec<T>>,
    curr_list_iter: ::std::vec::IntoIter<T>,
}

impl<T: Ord> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.curr_list_iter.next().or_else(|| {
            self.list_list_iter.next().and_then(|x| {
                self.curr_list_iter = x.into_iter();
                self.next()
            })
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (ll_min, ll_max) = self.list_list_iter.size_hint();
        let (cl_min, cl_max) = self.curr_list_iter.size_hint();
        (ll_min + cl_min,
         match (ll_max, cl_max) {
             (Some(x), Some(y)) => Some(x + y),
             _ => None,
         }
        )
    }
}

impl<'a, T: Ord> IntoIterator for SortedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            list_list_iter: self.value_lists.into_iter(),
            curr_list_iter: vec![].into_iter(),
        }
    }
}

/// Does a probably O(n^2) collection from an iterator -- but it's an iterator, not a
/// collection we're sorting, so what do you expect?
///
/// Actually may not be that bad based on the performance analysis that's todo
impl<'a, T: Ord> FromIterator<T> for SortedList<T> {
    fn from_iter<F>(iter: F) -> Self
        where F: IntoIterator<Item = T>
    {
        let mut list = Self::default();
        let mut iter = iter.into_iter();
        while let Some(x) = iter.next() {
            list.insert(x);
        }
        list
    }
}

impl<'a, T: Ord> Default for SortedList<T> {
    fn default() -> Self {
        SortedList::<T> {
            value_lists: vec![vec![]],
            index: JenksIndex { index: vec![0] },
            load_factor: DEFAULT_LOAD_FACTOR,
            twice_load_factor: DEFAULT_LOAD_FACTOR * 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn it_builds() {
        let default = SortedList::<u8>::default();
        assert!(default.value_lists.len() == 1);
        assert!(default.value_lists[0].len() == 0);
    }

    #[test]
    pub fn test_calculate_jenks_index() {
        let list: SortedList<u8> = SortedList::default();
        let index = JenksIndex::from_value_lists(&list.value_lists);
        assert_eq!(index, JenksIndex { index: vec![0] });

        let list: SortedList<u64> = SortedList {
            value_lists: vec![vec![1, 2, 3, 4, 5]],
            index: JenksIndex { index: vec![5] },
            load_factor: DEFAULT_LOAD_FACTOR,
            twice_load_factor: DEFAULT_LOAD_FACTOR * 2,
        };
        let index = JenksIndex::from_value_lists(&list.value_lists);
        assert_eq!(JenksIndex { index: vec![5] }, index);
    }

    #[test]
    pub fn basic_test() {
        let mut list: SortedList<i32> = SortedList::default();
        assert_eq!(0, list.len());

        list.insert(3);

        assert!(list.contains(&3));
        assert!(!list.contains(&13));
        assert_eq!(&3, list.first().unwrap());
        assert_eq!(&3, list.last().unwrap());

        list.insert(13);

        assert_eq!(2, list.len());
        assert!(list.contains(&3));
        assert!(list.contains(&13));
        assert!(!list.contains(&1));
        assert_eq!(&3, list.first().unwrap());
        assert_eq!(&13, list.last().unwrap());

        assert_eq!(13, list.pop_last().unwrap());

        assert!(list.contains(&3));
        assert!(!list.contains(&13));
        assert_eq!(&3, list.last().unwrap());

        assert_eq!(3, list.pop_first().unwrap());

        assert!(!list.contains(&3));

        assert_eq!(1, list.value_lists.len());
        assert_eq!(0, list.value_lists[0].len());

        assert_eq!(0, list.len());

        list.insert(1);
        assert_eq!(1, list.len());
        assert_eq!(&1, list.last().unwrap());
        assert_eq!(&1, list.first().unwrap());

        list.insert(20);
        assert_eq!(&20, list.last().unwrap());
    }
}

// returns None iff the first call to iter returns None.
fn find_or_last<T, P>(iter: Iterator<Item = T>, predicate: P) -> Option<T>
where P: FnMut(T,T) {
    let mut state = None;

    while let mut st = iter.next() {
        state = st;
        if predicate(state) {
            return state;
        }
    }
    state
}



#[cfg(test)]
mod quickcheck_tests {
    use super::SortedList;

    fn prop_from_iter_sorted<T: Ord>(list: Vec<T>) -> bool {
        let mut list = list.clone(); // can't get mutable values from quickcheck.
        list.sort();
        let from_iter: SortedList<'a, T> = list.iter().map(|x| x.clone()).collect();
        let from_collection = {
            let mut collection = SortedList::default();
            for x in list.iter() { collection.insert(*x); }
            collection
        };

        from_iter.iter().eq(list.iter()) && from_collection.iter().eq(list.iter())
    }

    quickcheck! {
        fn prop_from_iter_sorted_u8(list: Vec<u8>) -> bool {
            prop_from_iter_sorted(list)
        }

        fn prop_from_iter_sorted_u16(list: Vec<u16>) -> bool {
            prop_from_iter_sorted(list)
        }

        fn prop_from_iter_sorted_u32(list: Vec<u32>) -> bool {
            prop_from_iter_sorted(list)
        }

        fn prop_from_iter_sorted_u64(list: Vec<u64>) -> bool {
            prop_from_iter_sorted(list)
        }

        fn prop_from_iter_sorted_i8(list: Vec<i8>) -> bool {
            prop_from_iter_sorted(list)
        }

        fn prop_from_iter_sorted_i32(list: Vec<i32>) -> bool {
            prop_from_iter_sorted(list)
        }

        fn prop_from_iter_sorted_i64(list: Vec<i64>) -> bool {
            prop_from_iter_sorted(list)
        }
    }
}
