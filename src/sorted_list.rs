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

/// if the list size grows greater than the load factor, we split it.
/// If the list size shrinks below the load factor, we join two lists.
const DEFAULT_LOAD_FACTOR: usize = 1000;

// todo: make not copy.
#[derive(Debug)]
struct SortedList<T: Ord + Copy> {
    value_lists: Vec<Vec<T>>,
    maxes: Vec<T>,
    index: JenksIndex,
    load_factor: usize,
    twice_load_factor: usize, // cached for speed I guess?
}

/// The sorted list you've all been waiting for.
/// Hopefully it has really good performance.
impl<T: Ord + Copy> SortedList<T> {
    fn update_jenks_index(&mut self) {
        self.index = JenksIndex::from_value_lists(&self.value_lists);
    }

    pub fn contains(&self, val: &T) -> bool {
        let pos = bisect_left(&self.maxes, val);
        if pos >= self.value_lists.len() {
            return false;
        }
        self.value_lists[pos].contains(val)
    }

    pub fn insert(&mut self, val: T) {
        let mut which_list;
        if self.maxes.is_empty() {
            assert!(!self.value_lists.is_empty());
            self.value_lists[0].push(val);
            self.maxes.push(val);
            which_list = 0;
        } else {
            // TODO: I think this results in an extra list if we keep inserting the same value. But
            // bisect_left would do the same? No, it would put it in the list with its equal. But
            // we should push to the right if we can...... think about this later.
            which_list = bisect_right(&self.maxes, &val);

            if which_list == self.maxes.len() {
                which_list -= 1;
                self.value_lists[which_list].push(val);
                self.maxes[which_list] = val;
            } else {
                insort_left(&mut self.value_lists[which_list], val);
            }
        }
        self.expand(which_list);
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
        self.maxes[pos] = *self.value_lists[pos].last().unwrap();
        self.maxes.insert(pos + 1, new_list.last().unwrap().clone());
        self.value_lists.insert(pos + 1, new_list);
    }

    pub fn first(&self) -> Option<&T> {
        match self.value_lists.first() {
            Some(l) => l.first(),
            None => None,
        }
    }

    /// Returns a reference to the last (maximum) value in the list.
    pub fn last(&mut self) -> Option<&T> {
        match self.value_lists.last() {
            Some(l) => l.last(),
            None => None,
        }
    }

    pub fn pop_first(&mut self) -> Option<T> {
        if self.index.head() == 0 {
            None
        } else {
            Some(self.value_lists
                     .first_mut()
                     .unwrap()
                     .remove(0))
        }
    }

    pub fn pop_last(&mut self) -> Option<T> {
        let rv = match self.value_lists.last_mut() {
            Some(l) => l.pop(),
            None => None,
        };
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

impl<T: Ord + Copy> SortedList<T> {
    pub fn iter(&self) -> Iter<T> {
        let mut ll_iter = self.value_lists.iter();
        let mut cl_iter = ll_iter.next().unwrap().iter(); // TODO
        Iter {
            list_list_iter: ll_iter,
            curr_list_iter: cl_iter,
        }
    }
}

impl<'a, T: Ord + Copy> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.curr_list_iter.next() {
            Some(x) => Some(x),
            None => {
                self.list_list_iter.next().and_then(|x| { 
                    self.curr_list_iter = x.into_iter();
                    self.next()
                })
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (ll_min, ll_max) = self.list_list_iter.size_hint();
        let (cl_min, cl_max) = self.curr_list_iter.size_hint();
        (ll_min + cl_min, match (ll_max, cl_max) {
            (Some(x), Some(y)) => Some(x + y),
            _ => None,
        })
    }
}

pub struct IntoIter<T> {
    list_list_iter: ::std::vec::IntoIter<Vec<T>>,
    curr_list_iter: ::std::vec::IntoIter<T>,
}

impl<T: Ord + Copy> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.curr_list_iter.next() {
            Some(x) => Some(x),
            None => {
                match self.list_list_iter.next() {
                    Some(x) => {
                        self.curr_list_iter = x.into_iter();
                        self.next()
                    }
                    None => None,
                }
            }
        }
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

impl<'a, T: Ord + Copy> IntoIterator for SortedList<T> {
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
impl<T: Copy + Ord> FromIterator<T> for SortedList<T> {
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

//impl<'a, T> From<&'a [T]> for SortedList<T> where T: Clone {
//    fn from(s: &'a [T]) -> SortedList<T> {
//        let starting_size = DEFAULT_LOAD_FACTOR + DEFAULT_LOAD_FACTOR/2;
//        let value_lists = s.sorted().iter().chunks(starting_size).map(|x| x.collect()).collect();
//
//        SortedList{
//            value_lists: value_lists,
//            maxes: value_lists.map(|x| x.last().unwrap()),
//            index: JenksIndex::from_value_lists(value_lists),
//            load_factor: DEFAULT_LOAD_FACTOR,
//            twice_load_factor: DEFAULT_LOAD_FACTOR*2,
//        }
//    }
//}

// todo: Index<Range<usize>>
//impl<T> Index<usize> for SortedList<T> {
//    fn index(&self, index: usize) -> &T {
//        let mut currindex = index;
//        let mut
//        self.index[0]
//    }
//}

impl<T: Ord + Copy> Default for SortedList<T> {
    fn default() -> Self {
        SortedList::<T> {
            value_lists: vec![vec![]],
            maxes: vec![],
            index: JenksIndex { index: vec![] },
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
        assert!(default.maxes.len() == 0);
    }

    #[test]
    pub fn test_calculate_jenks_index() {
        let list: SortedList<u8> = SortedList::default();
        let index = JenksIndex::from_value_lists(&list.value_lists);
        assert_eq!(index, JenksIndex { index: vec![0] });

        let list: SortedList<u64> = SortedList {
            value_lists: vec![vec![1, 2, 3, 4, 5]],
            maxes: vec![5],
            index: JenksIndex { index: vec![5] },
            load_factor: DEFAULT_LOAD_FACTOR,
            twice_load_factor: DEFAULT_LOAD_FACTOR * 2,
        };
        let index = JenksIndex::from_value_lists(&list.value_lists);
        assert_eq!(JenksIndex { index: vec![5] }, index);
    }

    #[test]
    pub fn basic_list_test() {
        let mut list = SortedList::default();
        list.insert(3);
        assert!(list.contains(&3));
        assert!(!list.contains(&13));
        assert_eq!(&3, list.first().unwrap());
        assert_eq!(&3, list.last().unwrap());
        list.insert(13);
        assert!(list.contains(&3));
        assert!(list.contains(&13));
        assert!(!list.contains(&1));
        assert_eq!(&3, list.first().unwrap());
        assert_eq!(&13, list.last().unwrap());
        assert_eq!(13, list.pop_last().unwrap());
        assert!(list.contains(&3));
        assert!(!list.contains(&13));
        assert_eq!(&3, list.last().unwrap());
        list.insert(1);
        assert_eq!(&3, list.last().unwrap());
        list.insert(20);
        assert_eq!(&20, list.last().unwrap());
    }

}

#[cfg(test)]
mod quickcheck_tests {
    use super::*;
    use std::collections::BTreeMap;

    fn iters_equal<T: PartialEq, A: Iterator<Item = T>, B: Iterator<Item = T>>(a: &mut A, b: &mut B) -> bool{
        a.all(|alpha| match b.next() {
                Some(beta) => alpha == beta,
                None => false
        }) && b.next() == None
    }

//    quickcheck! {
//        // TODO: make this clone instead of copy.
//        // Also, PartialOrd?
//        //fn prop_from_iter_sorted<T: Copy + Ord>(list: &mut [T]) -> bool {
//        //    let from_iter: SortedList<T> = list.iter().collect();
//        //    let from_collection = {
//        //        let mut collection = SortedList<T>::default();
//        //        for x in list { collection.insert(X); }
//        //        collection
//        //    }
//        //    from_iter == list.sort()
//        //}
//        fn prop_from_iter_sorted_u8(list: Vec<u8>) -> bool {
//            let from_iter: SortedList<u8> = list.into_iter().collect();
//            let from_collection = {
//                let mut collection = SortedList::default();
//                for x in list { collection.insert(x); }
//                collection
//            };
//
//            //iters_equal(&mut from_iter.iter(), &mut list.iter())
//        }
//    }
}
