//! Module for a sorted list using multiple lists of varying length.
//!
//! Copied from Grant Jenks' sorted containers.

// TODO:
// make sure the index is never truly empty (should be [0] if empty).
//
// other invariants?

use std::default::Default;
use std::iter::FromIterator;

/// if the list size grows greater than the load factor, we split it.
/// If the list size shrinks below the load factor, we join two lists.
const DEFAULT_LOAD_FACTOR: usize = 1000;

fn insert_sorted<T: Ord>(vec: &mut Vec<T>, val: T) {
    match vec.binary_search(&val) {
        Ok(idx) | Err(idx) => vec.insert(idx, val)
    }
}

/// NOT generic, does not handle empty sublists except for a single empty list.
/// returns the index of the list that was inserted into.
fn insert_list<T: Ord>(list_list: &mut Vec<Vec<T>>, val: T) -> usize {
    if list_list.len() == 1 && list_list[0].len() == 0 {
        list_list[0].push(val);
        return 0;
    }
    let list_idx = match list_list
        .binary_search_by(|list| {
            val.cmp(&list.last().unwrap())
        }) {
            Ok(idx) | Err(idx) => idx
        };

    insert_sorted(&mut list_list[list_idx], val);
    list_idx
}

#[derive(Debug)]
pub struct SortedList<T: Ord> {
    lists: Vec<Vec<T>>, // There is always at least one element in this list.
    load_factor: usize,
    len: usize,
}

/// The sorted list you've all been waiting for.
///
/// It is a logic error for an item to be modified in such a way that the item's ordering relative
/// to any other item, as determined by the `Ord` trait, changes while it is in the heap. This is
/// normally only possible through `Cell`, `RefCell`, global state, I/O, or unsafe code.
impl<'a, T: Ord> SortedList<T> {
    pub fn contains(&self, val: &T) -> bool {
        assert!(!self.lists.is_empty());

        self.lists.iter().any(|list| list.contains(val))
    }

     fn insert(&mut self, new_val: T) {
         let idx_changed = insert_list(&mut self.lists, new_val);
         self.len += 1;
         self.expand(idx_changed);
    }

    /// Splits sublists that are more than double the load level.
    /// Updates the index when the sublist length is less than double the load
    /// level. This requires incrementing the nodes in a traversal from the
    /// leaf node to the root. For an example traversal see self._loc.
    fn expand(&mut self, idx: usize) {
        // >= because otherwise contract can fail... better solution for this?
        if self.lists[idx].len() >= 2 * self.load_factor {
            self.actual_expand(idx)
        }
    }

    fn actual_expand(&mut self, idx: usize) {
        let new_list = {
            let mut the_list = &mut self.lists[idx];
            let split_point = the_list.len() / 2;
            the_list.split_off(split_point)
        };

        self.lists.insert(idx + 1, new_list);
    }

    fn contract(&mut self, idx: usize) {
        if self.lists.len() > 1 &&
            self.lists[idx].len() < self.load_factor / 2 {
            self.actual_contract(idx)
        }
    }

    // TODO: this can make lists that are too big.
    /// Contracts with the nearest list.
    /// Example:
    /// ```
    /// let list = SortedList<u32> {
    ///   lists: Vec![vec![-6,-5,-3],
    ///               vec![1,2,3,4,5],
    ///               vec![99,100],],
    ///   load_factor: 2,
    ///   len: 10,
    ///   }
    ///   let new_lists = list.actual_contract(1).lists;
    ///   assert_eq!(new_lists, vec![vec![-6,-5,-3],
    ///                              vec![1,2,3,4,5,99,100],]);
    ///  ```
    fn actual_contract(&mut self, idx: usize) {
        assert!(self.len() > 1);
        let (low_idx, high_idx) =
            if idx == 0 {
                (0, 1)
            } else if idx == self.lists.len() {
                (self.lists.len() - 2, self.lists.len() - 1)
            } else {
                let other_list: usize =
                    if self.lists[idx - 1].len() < self.lists[idx + 1].len() {
                        idx - 1
                    } else {
                        idx + 1
                    };
                if idx < other_list {
                    (idx, other_list)
                } else {
                    (other_list, idx)
                }
            };

        let mut removed_list = self.lists.remove(high_idx);
        self.lists[low_idx].append(&mut removed_list);
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
        let rv = self.lists.last_mut().and_then(|l| l.pop());
        if rv.is_some() {
            self.len -= 1;
            let len = self.len;
            self.contract(len);
        }
        rv
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn iter(&self) -> Iter<T> {
        let mut ll_iter = self.lists.iter();
        let cl_iter = ll_iter.next().unwrap().iter();
        Iter {
            list_list_iter: ll_iter,
            curr_list_iter: cl_iter,
        }
    }
}

pub struct Iter<'a, T: 'a> {
    list_list_iter: ::std::slice::Iter<'a, Vec<T>>,
    curr_list_iter: ::std::slice::Iter<'a, T>,
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
            list_list_iter: self.lists.into_iter(),
            curr_list_iter: vec![].into_iter(),
        }
    }
}



impl<'a, T: Ord> Default for SortedList<T> {
    fn default() -> Self {
        SortedList::<T> {
            lists: vec![vec![]],
            load_factor: DEFAULT_LOAD_FACTOR,
            len: 0,
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

#[cfg(test)]
mod tests {
    use super::SortedList;
    #[test]
    pub fn it_builds() {
        let default = SortedList::<u8>::default();
        assert!(default.lists.len() == 1);
        assert!(default.lists[0].len() == 0);
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

        assert_eq!(1, list.lists.len());
        assert_eq!(0, list.lists[0].len());

        assert_eq!(0, list.len());

        list.insert(1);
        assert_eq!(1, list.len());
        assert_eq!(&1, list.last().unwrap());
        assert_eq!(&1, list.first().unwrap());

        list.insert(20);
        assert_eq!(&20, list.last_mut().unwrap());
    }
}

#[cfg(test)]
mod quickcheck_tests {
    use super::SortedList;

    fn prop_from_iter_sorted<T: Ord + Clone>(list: Vec<T>) -> bool {
        let mut list = list.clone(); // can't get mutable values from quickcheck.
        list.sort();
        let from_iter: SortedList<T> = list.iter().map(|x| x.clone()).collect();
        let from_collection = {
            let mut collection = SortedList::default();
            for x in list.iter() { collection.insert(x.clone()); }
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
