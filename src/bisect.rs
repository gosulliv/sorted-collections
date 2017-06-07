
//type DoubleIndex = (usize, usize);

///
/// Locate the insertion point for x in a to maintain sorted order. The
/// parameters lo and hi may be used to specify a subset of the list which
/// should be considered; by default the entire list is used. If x is already
/// present in a, the insertion point will be before (to the left of) any
/// existing entries. The return value is suitable for use as the first
/// parameter to list.insert() assuming that a is already sorted.
///
/// The returned insertion point i partitions the array a into two halves so
/// that all(val < x for val in a[lo:i]) for the left side and all(val >= x for
/// val in a[i:hi]) for the right side.
///
/// Examples:
/// ```
/// assert_eq!(bisect_left(vec![1,2,4,8],3),2);
///
/// assert_eq!(bisect_left(vec![2,3,5,7,11],7),3);
///
/// assert_eq!(bisect_left(vec![1,2,4,8],2),2);
/// ```
///
pub fn bisect_left<T: PartialOrd>(a: &[&mut T], x: &T) -> usize {
    // naive implementation.
    for i in 0..a.len() {
        if a[i] >= *x {
            return i;
        }
    }
    a.len() // ok, 0 if 0
}

/// Bisect, but on the right.
///
/// Examples:
/// ```
/// assert_eq!(bisect_right(vec![1,2,4,8],3),2);
///
/// assert_eq!(bisect_right(vec![2,3,5,7,11],7),4);
///
/// assert_eq!(bisect_right(vec![1,2,4,8],2),3);
/// ```
///
pub fn bisect_right<T: PartialOrd>(a: &Vec<T>, x: &T) -> usize {
    for i in (0..a.len()).rev() {
        if a[i] <= *x {
            return i + 1;
        }
    }
    0
}

//pub fn bisect_right<T>(x: T, a: Vec<T>, lo: usize, hi: usize) -> usize {
//    unimplemented!()
//}
//
//pub fn bisect<T>(x: &T, a: &Vec<T>) -> usize {
//unimplemented!()
//}

//pub fn bisect<T>(x: T, a: Vec<T>, lo: usize, hi: usize) -> usize {
//    unimplemented!()
//}
//
//

/// Examples:
/// ```
/// assert_eq!(insort_left(vec![1,4,5],3),vec![1,3,4,5]);
/// ```
pub fn insort_left<T: PartialOrd>(a: &mut Vec<T>, x: T) {
    let foo = bisect_left(a, &x);
    a.insert(foo, x);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn simple_bisects() {
        assert_eq!(bisect_left(&vec![], &1), 0);
        assert_eq!(bisect_left(&vec![], &0), 0);
        assert_eq!(bisect_left(&vec![1], &0), 0);
        assert_eq!(bisect_left(&vec![1], &1), 0);
        assert_eq!(bisect_left(&vec![1], &2), 1);

        assert_eq!(bisect_right(&vec![], &1), 0);
        assert_eq!(bisect_right(&vec![], &0), 0);
        assert_eq!(bisect_right(&vec![1], &0), 0);
        assert_eq!(bisect_right(&vec![1], &1), 1);
        assert_eq!(bisect_right(&vec![1], &2), 1);
    }
}
