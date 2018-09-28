use std::cmp::Ordering;

/// if the list size grows greater than the load factor, we split it.
/// If the list size shrinks below the load factor, we join two lists.
pub const DEFAULT_LOAD_FACTOR: usize = 1000;

/// Inserts into a list while maintaining a preexisting ordering.
pub fn insert_sorted<T: Ord>(vec: &mut Vec<T>, val: T) {
    match vec.binary_search(&val) {
        Ok(idx) | Err(idx) => vec.insert(idx, val),
    }
}

/// Inserts a value into a list of lists, as in SortedList.
///
/// Does not handle empty sublists except for a single empty list.
/// returns the index of the list that was inserted into.
pub fn insert_list_of_lists<T: Ord>(list_list: &mut Vec<Vec<T>>, val: T) -> usize {
    if list_list.len() == 1 && list_list[0].len() == 0 {
        list_list[0].push(val);
        return 0;
    }
    let list_idx = match list_list.binary_search_by(|list| {
        let first = list.first().unwrap();
        let last = list.last().unwrap();
        if last < &val {
            Ordering::Less
        } else if first > &val {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }) {
        Ok(idx) => idx,
        Err(idx) => match idx {
            // TODO how fair is this?
            0 => 0,
            n => n - 1,
        },
    };

    insert_sorted(&mut list_list[list_idx], val);
    list_idx
}
