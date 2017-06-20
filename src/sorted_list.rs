//! Module for a sorted list using multiple lists of varying length.
//!
//! Copied from Grant Jenks' sorted containers.

// TODO:
// make sure the index is never truly empty (should be [0] if empty).
//
// other invariants?

use std::default::Default;
use std::iter::FromIterator;
use std::cmp::Ordering;
use std::mem;

/// if the list size grows greater than the load factor, we split it.
/// If the list size shrinks below the load factor, we join two lists.
const DEFAULT_LOAD_FACTOR: usize = 1000;
const MINIMUM_LOAD_FACTOR: usize = 4;

fn insert_sorted<T: Ord>(vec: &mut Vec<T>, val: T) {
    match vec.binary_search(&val) {
        Ok(idx) | Err(idx) => vec.insert(idx, val)
    }
}

fn insert_list<T>(list_list: &mut Vec<ListWithSeparateMax<T>>, val: T) {
    let list_idx = match list_list.binary_search_by(|list_with_max| {
        x.cmp(&list_with_max.max)
    }) {
        Ok(idx) | Err(idx) => idx
    };
    list_list[list_idx].insert(val)
}

/// Always has at least the max set.
/// It is a logic error to remove the one remaining element. Take it out consuming the structure instead.
#[derive(Debug)]
struct ListWithSeparateMax<T>{
    vec: Vec<T>,
    max: T,
}

impl<T: Ord> ListWithSeparateMax<T> {
    fn new(x: T, size_hint: usize) -> ListWithSeparateMax<T> {
        ListWithSeparateMax{ vec: vec::with_capacity(size_hint), max: x }
    }

    fn max(&self) -> &T {
        &self.max
    }

    fn insert(&mut self, x: T) {
        let x = if self.max < x { mem::replace(&mut self.max, x) } else { x };
        insert_sorted(&mut self.vec, x)
    }

    fn contains(&self, x: &T) -> bool {
        &self.max == x ||
            self.vec.binary_search(x).is_ok()
    }

    fn binary_search(&self, x: &T) -> Result<usize, usize> {
        match x.cmp(&self.max) {
            Ordering::Equal => Ok(self.len() - 1),
            Ordering::Greater => Err(self.len()),
            Ordering::Less => self.vec.binary_search(x),
        }
    }

    fn remove(&mut self, idx: usize) -> T {
        assert!(idx < self.len());
        assert!(self.len() > 1);

        if idx == self.vec.len() {
            mem::replace(&mut self.max, self.vec.pop().unwrap())
        } else {
            self.vec.remove(idx)
        }
    }

    fn len(&self) -> usize {
        self.vec.len() + 1
    }
}

#[derive(Debug)]
pub struct SortedList<T: Ord> {
    lists: Vec<ListWithSeparateMax<T>>, // There is always at least one element in this list.
    //index: JenksIndex,
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
        assert_eq!(self.lists.len(), self.maxes.len());

        self.lists.contains(val)
    }

     fn insert(&mut self, new_val: T) {
         insert_list(&mut self.lists, new_val)
    }

    /// Splits sublists that are more than double the load level.
    /// Updates the index when the sublist length is less than double the load
    /// level. This requires incrementing the nodes in a traversal from the
    /// leaf node to the root. For an example traversal see self._loc.
    fn expand(&mut self, idx: usize) {
        // >= because otherwise contract can fail... better solution for this?
        if lists[idx].len() >= 2 * self.load_factor {
            actual_expand(idx)
        }
    }

    fn actual_expand(&mut self, idx: usize) {
        let split_point = the_list.len() / 2;
        let new_list = the_list.list.split_off(split_point); // high half, except max.
        self.lists.insert(idx, ListWithSeparateMax{
            list: new_list,
            max: mem::replace(the_list.max, the_list.pop()),
        });
    }


    fn contract(&mut self, idx: usize) {
        if self.lists[idx].len() < self.load_factor / 2 {
            actual_contract(idx)
        }
    }

    // FIXME
    fn actual_contract(&mut self, idx: usize) {
        let left_idx = idx - 1;
        let right_idx = idx + 1;
        let left_len = self[left_idx].len();
        let right_len = self[right_idx].len();

        if left_len + right_len + self[idx].len() > 2 * load_factor {
            return;
        } else {
            let mut old_list_iter = self.lists.remove(idx).into_iter();

            let left_list = &mut self.lists[left_idx];
            let right_list = &mut self.lists[right_idx - 1];

            for x in old_list_iter {
                if (left_list.len() < self.load_factor * 2) {
                    left_list.append(x);
                } else {
                    right_list.push(x);
                }
            }
        }
    }

    /// Assumes the list is not empty.
    //fn split_sublist(&mut self, pos: usize) {
    //    let new_list = self.lists[pos].split_off(self.load_factor);
    //    self.lists.insert(pos + 1, new_list);
    //}

    pub fn first(&self) -> Option<&T> {
        self.lists.first()
    }

    /// Returns a reference to the last (maximum) value in the list.
    pub fn last(&mut self) -> Option<&T> {
        self.lists.last()
    }

    pub fn last_mut(&mut self) -> Option<&mut T> {
        self.lists.last_mut()
    }

    pub fn pop_first(&mut self) -> Option<T> {
        if self.len() == 0 {
            return None
        }

        self.lists[0].pop_first()
        let rv = Some(self.lists.first_mut().unwrap().remove(0));
        self.removed_from(0);
        rv
    }

    pub fn pop_last(&mut self) -> Option<T> {
        let rv = self.lists.last_mut().and_then(|l| l.pop());
        // TODO: expand?
        let last_idx = self.lists.len() - 1;
        self.removed_from(last_idx);
        rv
    }

    pub fn len(&self) -> usize {
        //return self.index.head();
        return self.len;
    }
}

pub struct Iter<'a, T: 'a> {
    list_list_iter: ::std::slice::Iter<'a, Vec<T>>,
    curr_list_iter: ::std::slice::Iter<'a, T>,
}

impl<'a, T: Ord> SortedList<T> {
    pub fn iter(&self) -> Iter<T> {
        let mut ll_iter = self.lists.iter();
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
            list_list_iter: self.lists.into_iter(),
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
            lists: vec![vec![]],
            maxes: vec![],
            //index: JenksIndex { index: vec![0] },
            load_factor: DEFAULT_LOAD_FACTOR,
            len: 0,
        }
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

    //#[test]
    //pub fn test_calculate_jenks_index() {
    //    let list: SortedList<u8> = SortedList::default();
    //    let index = JenksIndex::from_lists(&list.lists);
    //    assert_eq!(index, JenksIndex { index: vec![0] });

    //    let list: SortedList<u64> = SortedList {
    //        lists: vec![vec![1, 2, 3, 4, 5]],
    //        maxes: vec![6],
    //        index: JenksIndex { index: vec![5] },
    //        load_factor: DEFAULT_LOAD_FACTOR,
    //    };
    //    let index = JenksIndex::from_lists(&list.lists);
    //    assert_eq!(JenksIndex { index: vec![5] }, index);
    //}

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
//
//#[cfg(test)]
//mod quickcheck_tests {
//    use super::SortedList;
//
//    fn prop_from_iter_sorted<T: Ord>(list: Vec<T>) -> bool {
//        let mut list = list.clone(); // can't get mutable values from quickcheck.
//        list.sort();
//        let from_iter: SortedList<T> = list.iter().map(|x| x.clone()).collect();
//        let from_collection = {
//            let mut collection = SortedList::default();
//            for x in list.iter() { collection.insert(x.clone()); }
//            collection
//        };
//
//        from_iter.iter().eq(list.iter()) && from_collection.iter().eq(list.iter())
//    }
//
//    quickcheck! {
//        fn prop_from_iter_sorted_u8(list: Vec<u8>) -> bool {
//            prop_from_iter_sorted(list)
//        }
//
//        fn prop_from_iter_sorted_u16(list: Vec<u16>) -> bool {
//            prop_from_iter_sorted(list)
//        }
//
//        fn prop_from_iter_sorted_u32(list: Vec<u32>) -> bool {
//            prop_from_iter_sorted(list)
//        }
//
//        fn prop_from_iter_sorted_u64(list: Vec<u64>) -> bool {
//            prop_from_iter_sorted(list)
//        }
//
//        fn prop_from_iter_sorted_i8(list: Vec<i8>) -> bool {
//            prop_from_iter_sorted(list)
//        }
//
//        fn prop_from_iter_sorted_i32(list: Vec<i32>) -> bool {
//            prop_from_iter_sorted(list)
//        }
//
//        fn prop_from_iter_sorted_i64(list: Vec<i64>) -> bool {
//            prop_from_iter_sorted(list)
//        }
//    }
//}
