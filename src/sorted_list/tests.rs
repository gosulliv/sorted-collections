use super::sorted_utils::insert_sorted;
use super::SortedList;

#[test]
fn it_builds() {
    let default = SortedList::<u8>::default();
    assert!(default.lists.len() == 1);
    assert!(default.lists[0].len() == 0);
}

#[test]
fn test_insert() {
    let mut vec = vec![];
    insert_sorted(&mut vec, 22);
    assert_eq!(vec![22], vec);
    insert_sorted(&mut vec, -1000);
    assert_eq!(vec![-1000, 22], vec);
}

#[test]
fn basic_test() {
    let mut list: SortedList<i32> = SortedList::default();
    assert_eq!(0, list.len());

    list.add(3);

    assert!(list.contains(&3));
    assert!(!list.contains(&13));
    assert_eq!(Some(3), list.first());
    assert_eq!(Some(3), list.last());

    list.add(13);

    assert_eq!(2, list.len());
    assert!(list.contains(&3));
    assert!(list.contains(&13));
    assert!(!list.contains(&1));
    assert_eq!(Some(3), list.first());
    assert_eq!(Some(13), list.last());

    assert_eq!(13, list.pop_last().unwrap());

    assert!(list.contains(&3));
    assert!(!list.contains(&13));
    assert_eq!(Some(3), list.last());

    assert_eq!(3, list.pop_first().unwrap());

    assert!(!list.contains(&3));

    assert_eq!(1, list.lists.len());
    assert_eq!(0, list.lists[0].len());

    assert_eq!(0, list.len());

    list.add(1);
    assert_eq!(1, list.len());
    assert_eq!(Some(1), list.last());
    assert_eq!(Some(1), list.first());

    list.add(20);
    assert_eq!(Some(20), list.last_mut());
}

#[test]
fn sequence() {
    let mut list = SortedList::default();
    for i in 0..15000 {
        assert_eq!(i, list.len());
        list.add(i);
    }

    for i in 0..15000 {
        assert_eq!(i, list[i]);
    }
}

#[test]
fn zeroes() {
    let mut list = SortedList::default();

    for i in 0..15000 {
        assert_eq!(i, list.len());
        list.add(0);
    }

    for i in 0..15000 {
        assert_eq!(0, list[i]);
    }
}

#[test]
fn ones() {
    let mut list = SortedList::default();

    for i in 0..15000 {
        assert_eq!(i, list.len());
        list.add(1);
    }

    for i in 0..15000 {
        assert_eq!(1, list[i]);
    }
}

#[test]
#[should_panic]
fn out_of_bounds_panics() {
    let list: SortedList<i32> = SortedList::default();
    list[0];
}

#[test]
fn test_actual_contract() {
    let mut list = SortedList::<i32> {
        lists: vec![vec![-6, -5, -3], vec![1, 2, 3, 4, 5], vec![99, 100]],
        load_factor: 2,
        len: 10,
    };
    list.actual_contract(1);
    assert_eq!(
        list.lists,
        vec![vec![-6, -5, -3], vec![1, 2, 3, 4, 5, 99, 100]]
    );
}

fn prop_from_iter_sorted<T: Ord + Clone>(list: Vec<T>) -> bool {
    let mut list = list.clone(); // can't get mutable values from quickcheck.
    list.sort();
    let from_iter: SortedList<T> = list.iter().map(|x| x.clone()).collect();
    let from_collection = {
        let mut collection = SortedList::default();
        for x in list.iter() {
            collection.add(x.clone());
        }
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
