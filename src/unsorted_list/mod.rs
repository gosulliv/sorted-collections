use std::default::Default;
use std::iter::FromIterator;
use std::ops::{Index, IndexMut};

const DEFAULT_LOAD_FACTOR: usize = 1000;

#[derive(Debug)]
pub struct UnsortedList<T> {
    lists: Vec<Vec<T>>, // There is always at least one element in the outer list.
    load_factor: usize,
    len: usize,
}

impl<T> UnsortedList<T> {
    pub fn insert(&mut self, index: usize, element: T) {
        let mut sublist_index = index;
        let mut list_index = 0;

        // biases towards the earlier list.
        while sublist_index > self.lists[list_index].len() {
            sublist_index -= self.lists[list_index].len();
            list_index += 1;
        }

        self.lists[list_index].insert(sublist_index, element);
        self.len += 1;
        self.checked_expand(list_index);
    }

    /// Splits sublists that are more than double the load level.
    /// Updates the index when the sublist length is less than double the load
    /// level. This requires incrementing the nodes in a traversal from the
    /// leaf node to the root. For an example traversal see self._loc.
    fn checked_expand(&mut self, idx: usize) {
        // >= because otherwise contract can fail... better solution for this?
        if self.lists[idx].len() >= 2 * self.load_factor {
            self.do_expand(idx)
        }
    }

    fn do_expand(&mut self, idx: usize) {
        let new_list = {
            let the_list = &mut self.lists[idx];
            let split_point = the_list.len() / 2;
            the_list.split_off(split_point)
        };

        self.lists.insert(idx + 1, new_list);
    }

    // TODO: this can make lists that are too big.
    fn checked_contract(&mut self, idx: usize) {
        if self.lists.len() > 1 && self.lists[idx].len() < self.load_factor / 2 {
            self.actual_contract(idx)
        }
    }

    /// Contracts with the nearest list.
    fn actual_contract(&mut self, idx: usize) {
        assert!(self.len() > 1);
        let (low_idx, high_idx) = self.contract_idx(idx);
        let mut removed_list = self.lists.remove(high_idx);
        self.lists[low_idx].append(&mut removed_list);
    }

    fn contract_idx(&self, idx: usize) -> (usize, usize) {
        if idx == 0 {
            (0, 1)
        } else if idx == self.lists.len() {
            (self.lists.len() - 2, self.lists.len() - 1)
        } else {
            let other_list: usize = if self.lists[idx - 1].len() < self.lists[idx + 1].len() {
                idx - 1
            } else {
                idx + 1
            };
            if idx < other_list {
                (idx, other_list)
            } else {
                (other_list, idx)
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
            self.checked_contract(0);
            rv
        }
    }

    pub fn push(&mut self, element: T) {
        self.lists.last_mut().unwrap().push(element);
        self.len += 1;
        let len = self.lists.len();
        // FIXME catch with test?
        self.checked_contract(len);
    }

    pub fn pop(&mut self) -> Option<T> {
        let rv = self.lists.last_mut().and_then(|l| l.pop());
        if rv.is_some() {
            self.len -= 1;
            let len = self.lists.len();
            self.checked_contract(len);
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

    fn indices(&self, index: usize) -> (usize, usize) {
        let mut sublist_index = index;
        let mut list_index = 0;

        // biases towards the earlier list.
        while sublist_index > self.lists[list_index].len() {
            sublist_index -= self.lists[list_index].len();
            list_index += 1;
        }
        (list_index, sublist_index)
    }
}

impl<T: PartialEq> UnsortedList<T> {
    pub fn contains(&self, val: &T) -> bool {
        assert!(!self.lists.is_empty());

        self.lists.iter().any(|list| list.contains(val))
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
        (
            ll_min + cl_min,
            match (ll_max, cl_max) {
                (Some(x), Some(y)) => Some(x + y),
                _ => None,
            },
        )
    }
}

impl<'a, T: Ord> IntoIterator for UnsortedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            list_list_iter: self.lists.into_iter(),
            curr_list_iter: vec![].into_iter(),
        }
    }
}

impl<'a, T: Ord> Default for UnsortedList<T> {
    fn default() -> Self {
        UnsortedList::<T> {
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

impl<'a, T: Ord> Index<usize> for UnsortedList<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        let (list_index, sublist_index) = self.indices(index);
        &self.lists[list_index][sublist_index]
    }
}

impl<'a, T: Ord> IndexMut<usize> for UnsortedList<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        let (list_index, sublist_index) = self.indices(index);
        &mut self.lists[list_index][sublist_index]
    }
}

#[cfg(test)]
mod tests;
