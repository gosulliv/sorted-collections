//! Module for a sorted list using multiple lists of varying length.
//!
//! Copied from Grant Jenks' sorted containers.

use bisect::*;
use jenks_index::JenksIndex;
use std::default::Default;
use std::iter::FromIterator;

/// if the list size grows greater than the load factor, we split it.
/// If the list size shrinks below the load factor, we join two lists.
const DEFAULT_LOAD_FACTOR: usize = 1000;

// todo: make not copy.
#[derive(Debug)]
struct SortedList<T: PartialOrd + Copy> {
    total_elements: usize,
    value_lists: Vec<Vec<T>>,
    maxes: Vec<T>,
    index: JenksIndex,
    load_factor: usize,
    twice_load_factor: usize, // cached for speed I guess?
}

/// The sorted list you've all been waiting for.
/// Hopefully it has really good performance.
///
/// Example usage:
///
impl<T: PartialOrd + Copy> SortedList<T> {
    fn update_jenks_index(&mut self) {
        self.index = JenksIndex::from_value_lists(&self.value_lists);
    }

    pub fn contains(&self, val: T) -> bool {
        let pos = bisect_left(&self.maxes, &val);
        let idx = bisect_left(&self.value_lists[pos], &val);
        self.value_lists[pos][idx] == val
    }

    pub fn add(&mut self, val: T) {
        if self.maxes.is_empty() {
            self.value_lists.push(vec![val]);
            self.maxes.push(val);
        } else {
            let mut pos = bisect_right(&self.maxes, &val);

            if pos == self.maxes.len() {
                pos -= 1;
                self.value_lists[pos].push(val);
                self.maxes[pos] = val;
            } else {
                insort_left(&mut self.value_lists[pos], val);
            }
            self.expand(pos);
        }
        self.total_elements += 1;
    }

    /// Splits sublists that are more than double the load level.
    /// Updates the index when the sublist length is less than double the load
    /// level. This requires incrementing the nodes in a traversal from the
    /// leaf node to the root. For an example traversal see self._loc.
    fn expand(&mut self, pos: usize) {
        if self.value_lists[pos].len() > self.twice_load_factor {
            self.split_list(pos);
            // TODO: update index better.
            self.update_jenks_index();
        } else {
            self.index.increment_above_leaf(pos);
        }
    }

    /// Assumes the list is not empty.
    fn split_list(&mut self, pos: usize) {
        let mut new_list = self.value_lists[pos].split_off(self.load_factor);
        self.maxes[pos] = *self.value_lists[pos].last().unwrap();
        self.maxes.insert(pos + 1, new_list.last().unwrap().clone());
        self.value_lists.insert(pos + 1, new_list);
    }


    //fn jenks_lookup(self, idx: usize) {
    //let mut idx = idx;
    //while (idx
    //}
}

pub struct SortedListIter<T> {
    list_list_iter: ::std::vec::IntoIter<Vec<T>>,
    curr_list_iter: ::std::vec::IntoIter<T>,
}
impl<'a, T: PartialOrd + Copy> Iterator for SortedListIter<T> {
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
}

impl<'a, T: PartialOrd + Copy> IntoIterator for SortedList<T> {
    type Item = T;
    type IntoIter = SortedListIter<T>;

    fn into_iter(self) -> SortedListIter<T> {
        SortedListIter {
            list_list_iter: self.value_lists.into_iter(),
            curr_list_iter: vec![].into_iter(),
        }
    }
}

/// Does a probably O(n^2) collection from an iterator -- but it's an iterator, not a
/// collection we're sorting, so what do you expect?
///
/// Actually may not be that bad based on the performance analysis that's todo
impl<T: Copy + PartialOrd> FromIterator<T> for SortedList<T> {
    fn from_iter<F>(iter: F) -> Self
        where F: IntoIterator<Item = T>
    {
        let mut list = Self::default();
        let mut iter = iter.into_iter();
        while let Some(x) = iter.next() {
            list.add(x);
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
//            total_elements: s.len(),
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

impl<T: PartialOrd + Copy> Default for SortedList<T> {
    fn default() -> Self {
        SortedList::<T> {
            total_elements: 0,
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
        assert!(default.total_elements == 0);
        assert!(default.total_elements == 0);
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
            total_elements: 5,
            value_lists: vec![vec![1, 2, 3, 4, 5]],
            maxes: vec![5],
            index: JenksIndex { index: vec![5] },
            load_factor: DEFAULT_LOAD_FACTOR,
            twice_load_factor: DEFAULT_LOAD_FACTOR * 2,
        };
        let index = JenksIndex::from_value_lists(&list.value_lists);
        assert_eq!(JenksIndex { index: vec![5] }, index);
    }

}
