//! Module for a sorted list using multiple lists of varying length.
//!
//! Copied from Grant Jenks' sorted containers.

//impl SortedList<T> {
//    pub fn newSortedList(Vec<T>) -> SortedList<T> {
//        SortedList<T> {
//            size: v.len(),
//            ValueLists: Vec<Vec<T>>,
//        }
//    }
//}

use bisect::*;
use jenks_index::JenksIndex;
use std::default::Default;

/// if the list size grows greater than the load factor, we split it.
/// If the list size shrinks below the load factor, we join two lists.
const DEFAULT_LOAD_FACTOR: usize = 1000;

// todo? make this a: Iterable or something.

    //match a.iter().partition::<Vec<usize>,_>(|x| {even = !even; even})
    //{
        //(even, odd) => even.into_iter()
            //.zip(odd.into_iter())
            //.map(|tuple| match tuple {
                //(l,r) => l + r
            //})
            //.collect()
    //}

// todo: make a better derivation of PartialEq, Eq.
#[derive(Debug)]
struct SortedList<T> {
    total_elements: usize,
    value_lists: Vec<Vec<T>>,
    maxes: Vec<usize>,
    index: JenksIndex,
    load_factor: usize,
    twice_load_factor: usize, // cached for speed I guess?
}

impl<T: PartialOrd> SortedList<T> {
    fn update_jenks_index(&mut self) {
        self.jenks_index = self.JenksIndex.from_value_lists(&value_lists);
    }

    pub fn contains(&self, val: T) -> bool {
        let pos = bisect_left(self.maxes, val);
        let idx = bisect_left(self.lists[pos], val);
        self.lists[pos][idx] == val
    }

    pub fn add(&mut self, val: T) {
        if self.maxes.is_empty() {
            self.lists.push(vec![val]);
            self.maxes.push(val);
        } else {
            let mut pos = bisect_right(self.maxes, val);
            
            if pos == self.maxes.len() {
                pos -= 1;
                self.lists[pos].append(val);
                self.maxes[pos] = val;
            } else {
                insort_left(lists[pos], val);
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
        if self.lists[pos].len() > self.twice_load_factor {
            self.split_list(pos);
        } else if !self.index.is_empty() {
            let child = self.offset + pos;
 
            // TODO: get jenks index working.
     //else:
     //    if _index:
     //        child = self._offset + pos
     //        while child:
     //        _index[child] += 1
     //        child = (child - 1) >> 1
     //        child = (child - 1) / 2
        }
    }
    /// Assumes the list is not empty.
    fn split_list(&mut self, pos: usize) {
        let mut new_list = self.value_lists[pos].split_off(self.load_factor);
        self.maxes[pos] = self.value_lists[pos].last().unwrap();
        self.lists.insert(pos + 1, new_list);
        self.maxes.insert(pos + 1, new_list.last().unwrap());
    }
        

    //fn jenks_lookup(self, idx: usize) {
       //let mut idx = idx;
       //while (idx 
    //}
}

//impl IntoIterator for SortedList<T> {
//    type Item = T;
//    fn into_iter(self) -> Self::IntoIter;
//}
impl<'a, T> From<&'a [T]> for SortedList<T> where T: Clone {
    fn from(s: &'a [T]) -> SortedList<T> {
        unimplemented!()
    }
}

// todo: Index<Range<usize>>
//impl<T> Index<usize> for SortedList<T> {
//    fn index(&self, index: usize) -> &T {
//        let mut currindex = index;
//        let mut 
//        self.index[0]
//    }
//}

impl<T> Default for SortedList<T> {
    fn default() -> Self {
        SortedList::<T> {
            total_elements: 0,
            value_lists: vec!(Vec::default()),
            maxes: Vec::default(),
            index: Vec::default(),
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
        let index = JenksIndex::fromValueLists(list.value_lists);
        assert_eq!(index, vec![]);

        let list: SortedList<u64> = SortedList{
            total_elements: 5,
            value_lists: vec![vec![1,2,3,4,5]],
            maxes: vec![5],
            index: vec![vec![5]],
            load_factor: DEFAULT_LOAD_FACTOR,
            twice_load_factor: DEFAULT_LOAD_FACTOR * 2,
        };
        let index = JenksIndex::fromValueLists(list.value_lists);
        assert_eq!(vec![5], index);
    }

}

