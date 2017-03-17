
/// A flattened tree structure, represented by a Vec of lengths.
///
/// We want to make index methods like leftChild()
///
/// The algorithm works something like this. Given the lists:
/// _lists = [
///  [0,1,2,3],
///  [4,5,6],
///  [7,8,9,10,11,12],
///  [13,14,15,16,17]]
///  _maxes = [3,6,12,17]
///
/// lengths = [ 4,3,6,5 ]
/// pair_wise_sums1 = [ 7,11 ]
/// pair_wise_sums2  = [ 18 ]
/// _index = [ 18, 7, 11, 4, 3, 6 ,5 ]
/// _offset = 3
///
/// the index is the lenghts, followed by their pairwise sums, followed by their pairwise sums,
/// etc.
///
/// The index is used for positional indexing.
/// To find the nth value, we look at the left child of the current node.
/// If position is less than the left child, go left.
///
/// If position is more than the left child, subtract the left child's number
/// (by going to the right, we have passed that many elements over),
/// and go to the right.
///
/// In either case, continue until we're at a leaf node, then index into that array by what's left.

#[derive(Debug,PartialEq,Eq)]
pub struct JenksIndex {
    pub index: Vec<usize>,
}

//impl From<Vec<usize>> for JenksIndex {
    //fn from(v: Vec<usize>) -> Self {
        //JenksIndex{index: v}
    //}
//}

pub struct JenksIterator<'a> {
    pos: usize,
    index: &'a Vec<usize>,
}

impl JenksIndex {
/// Calculate the "Jenks Index" of the set, which is basically a heap-like lookup tree.
///
/// The algorithm works something like this. Given the lists:
/// _lists = [
///  [0,1,2,3],
///  [4,5,6],
///  [7,8,9,10,11,12],
///  [13,14,15,16,17]]
///  _maxes = [3,6,12,17]
///
/// lengths = [ 4,3,6,5 ]
/// pair_wise_sums1 = [ 7,11 ]
/// pair_wise_sums2  = [ 18 ]
/// _index = [ 18, 7, 11, 4, 3, 6 ,5 ]
/// _offset = 3
///
/// ```
/// let from_lists = JenksIndex::from_value_lists(
///   vec![vec![1,2,3],vec![4,18],vec![37,38,4]]);
/// let from_flattened = JenksIndex{vec![vec![8],vec![5,3],vec![3,2,3]].flatten()};
/// let flattened = [8,5,3,3,2,3];
/// assert_eq!(flattened,from_lists);
/// assert_eq!(from_flattened,from_lists);
///
///
    pub fn from_value_lists<T>(value_lists: &Vec<Vec<T>>) -> JenksIndex {
        let lengths = value_lists.iter().map(|l| l.len()).collect();
        // triangular number... 1+2+3+4+...+n = n*n/2
        //let mut index = Vec::with_capacity(lengths.len().pow(2)/2);
        let mut steps: Vec<Vec<usize>> = Vec::with_capacity(value_lists.len()); // n/2 + n/4 + ...
        steps.push(lengths);
        while steps.last().unwrap().len() > 1 {
            let next = pair_sum(steps.last().unwrap());
            steps.push(next);
        }
        steps.reverse();
        JenksIndex{
            index: steps.iter()
            .flat_map(|x| x.iter())
            .map(|x| x.clone()) // to satisfy the FromIterator trait. sigh.
            .collect()
        }
    }

    /// Returns the left child, or None if this is a leaf node.
    ///
    /// ```
    /// let empty_index = JenksIndex{vec![]};
    /// let single_index = JenksIndex{vec![0]};
    /// assert_eq!(empty_index.left_child(0), None);
    /// assert_eq!(single_index.left_child(0), None);
    ///
    /// let several_index = JenksIndex{vec![3,1,2]};
    /// assert_eq!(several_index.left_child(0), 1);
    /// ```
    pub fn left_child(&self, pos: usize) -> Option<usize> {
        //  [ 0,
        //  1, 2,
        //3,4,5,6, ]
        let lchild = pos * 2 + 1;
        if lchild > self.index.len() {
            None
        } else {
            Some(lchild)
        }
    }
    /// ```
    /// let empty_index = JenksIndex{vec![]};
    /// let single_index = JenksIndex{vec![0]};
    /// assert_eq!(empty_index.right_child(0), None);
    /// assert_eq!(single_index.right_child(0), None);
    ///
    /// let several_index = JenksIndex{vec![3,1,2]};
    /// assert_eq!(several_index.right_child(0), 2);
    /// ```
    /// 
    pub fn right_child(&self, pos: usize) -> Option<usize> {
        let rchild = pos * 2 + 2;
        if rchild > self.index.len() {
            None
        } else {
            Some(rchild)
        }
    }

    /// ```
    /// let mut j = JenksIndex{vec![]};
    /// assert_eq!(j.parent(0), None);
    /// assert_eq!(j.parent(1), None);
    /// assert_eq!(j.parent(2), None);
    /// assert_eq!(j.parent(3), None);
    /// j.index.push(20);
    /// assert_eq!(j.parent(0), None);
    /// assert_eq!(j.parent(1), None);
    /// assert_eq!(j.parent(2), None);
    /// j.index.push(40);
    /// assert_eq!(j.parent(0), None);
    /// assert_eq!(j.parent(1), Some(0));
    /// assert_eq!(j.parent(2), None);
    /// j.index.push(1000);
    /// assert_eq!(j.parent(0), None);
    /// assert_eq!(j.parent(1), Some(0));
    /// assert_eq!(j.parent(2), Some(0));
    /// assert_eq!(j.parent(3), None);
    /// j.index.push(1);
    /// assert_eq!(j.parent(0), None);
    /// assert_eq!(j.parent(1), Some(0));
    /// assert_eq!(j.parent(2), Some(0));
    /// assert_eq!(j.parent(3), Some(1));
    /// j.index.push(34);
    /// j.index.push(55);
    /// assert_eq!(j.parent(0), None);
    /// assert_eq!(j.parent(1), Some(0));
    /// assert_eq!(j.parent(2), Some(0));
    /// assert_eq!(j.parent(3), Some(1));
    /// assert_eq!(j.parent(4), Some(1));
    /// assert_eq!(j.parent(5), Some(2));
    /// assert_eq!(j.parent(6), Some(2));
    /// assert_eq!(j.parent(7), None);
    pub fn parent(&self, pos: usize) -> Option<usize> {
        if pos == 0 || pos > self.index.len() {
            None
        } else {
            Some((pos - 1) / 2)
        }
    }

    pub fn increment_above_leaf(&mut self, pos: usize) {
        self.index[pos] += 1;

    }
}

fn pair_sum(a: &Vec<usize>) -> Vec<usize> {
    let mut even = false; //false so we return the first
    a.chunks(2).map(|pair| pair.iter().fold(0,|x,y| x+y)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    pub fn test_pair_sum() {
        let empty: Vec<usize> = Vec::default();
        assert_eq!(empty, pair_sum(&empty));

        let single: Vec<usize> = vec![0];
        assert_eq!(single, pair_sum(&single));

        let single_2: Vec<usize> = vec![1010];
        assert_eq!(single, pair_sum(&single));

        let a: Vec<usize> = vec![1,2,3,4];
        assert_eq!(vec![3,7], pair_sum(&a)); 

        let b: Vec<usize> = vec![1000,220,2];
        assert_eq!(vec![1220,2], pair_sum(&b)); 
    }
}
