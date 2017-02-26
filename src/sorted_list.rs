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

use bisect;
use std::default::Default;

/// if the list size grows greater than the load factor, we split it.
/// If the list size shrinks below the load factor, we join two lists.
const LOAD_FACTOR: usize = 1000;

struct SortedList<T> {
    totalElements: usize,
    valueLists: Vec<Vec<T>>,
    maxes: Vec<usize>,
}

enum Twocumulator<T: Copy> {
    Even(Vec<T>),
    Odd(Vec<T>,T),
}

/// Returns an empty Twocumulator.
impl<T: Copy> Default for Twocumulator<T> {
    fn default() -> Self {
        Twocumulator::Even(Vec::default())
    }
}
fn pair_sum(a: Vec<usize>) -> Vec<usize> {
    match a.into_iter()
           .fold(Twocumulator::default(),|acc, val|
                 match acc {
                     Twocumulator::Even(vec) => Twocumulator::Odd(vec, val),
                     Twocumulator::Odd(vec,saved) => Twocumulator::Even({vec.push(val + saved);vec})
                 }
                ) {
        Twocumulator::Even(vec) => vec,
        Twocumulator::Odd(vec,v) => {vec.push(v);vec},
    }
}

 

// todo.
//impl<T> into<Vec<T>> for Twocumulator<T> {
    //
//}

impl<T> SortedList<T> {
           

//        let step = match lengths.into_iter() {
//            mut iter => loop {
//                match iter.next() {
//                    Some(i) => a,
//                    None => break
//                }
//            }
//        }

/// _lists = [
///  [0,1,2,3],
///  [4,5,6],
///  [7,8,9,10,11,12],
///  [13,14,15,16,17]]
///  _maxes = [3,6,12,17]

/// lengths = [ 4,3,6,5 ]
/// pair_wise_sums1 = [ 7,11 ]
///pair_wise_sums2  = [18]
/// _index = [18, 7 ,11, 4, 3, 6 ,5 ]
/// _offset = 3
    fn jenks_index(self) {
        let lengths = self.valueLists.iter()
            .map(|l| l.len())
            .collect();
        pair_sum(lengths)
    }
    pub fn insert(value: T) {
        
    }

    //pub fn jenks_lookup(self, idx: usize) {
        //let mut idx = idx;
    //}
}


impl<T> Default for SortedList<T> {
    fn default() -> Self {
        SortedList::<T> {
            totalElements: 0,
            valueLists: vec!(Vec::default()),
            maxes: Vec::default(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn it_builds() {
        let default = SortedList::<u8>::default();
        assert!(default.totalElements == 0);
        assert!(default.totalElements == 0);
        assert!(default.valueLists.len() == 1);
        assert!(default.valueLists[0].len() == 0);
        assert!(default.maxes.len() == 0);
    }
}

