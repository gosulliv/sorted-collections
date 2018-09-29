use super::UnsortedList;
#[test]
fn empty() {
    let mut list: UnsortedList<i32> = UnsortedList::default();
    assert_eq!(list.len(), 0);
    assert_eq!(list.first(), None);
    assert_eq!(list.first_mut(), None);
    assert_eq!(list.last(), None);
    assert_eq!(list.last_mut(), None);
    assert_eq!(list.pop(), None);
    assert_eq!(list.pop_first(), None);
}

#[test]
fn index() {
    use unsorted_list::UnsortedList;
    let mut list = UnsortedList::default();
    list.insert(0, 100);
    list.insert(0, 10);
    list.insert(1, 1);
    assert_eq!(list[0], 10);
    assert_eq!(list[1], 1);
    assert_eq!(list[2], 100);
    assert_eq!(list.pop(), Some(100));
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), Some(10));
}

#[test]
fn test_actual_contract() {
    let mut list = UnsortedList::<i32> {
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

quickcheck! {
    fn first(element: u8) -> bool {
        let mut list: UnsortedList<u8> = Some(element).into_iter().collect();
        list.push(element);

        list.first() == Some(&element)
    }

    fn first_mut(element: u8) -> bool {
        let mut list = UnsortedList::default();
        list.push(element);

        list.first_mut() == Some(&mut element.clone())
    }

    fn last(element: u8) -> bool {
        let mut list: UnsortedList<u8> = Some(element).into_iter().collect();
        list.last() == Some(&element)
    }

    fn last_mut(element: u8) -> bool {
        let mut list: UnsortedList<u8> = Some(element).into_iter().collect();
        list.last_mut() == Some(&mut element.clone())
    }

    fn pop(element: u8) -> bool {
        let mut list: UnsortedList<u8> = Some(element).into_iter().collect();
        list.pop() == Some(element)
    }

    fn pop_first(element: u8) -> bool {
        let mut list: UnsortedList<u8> = Some(element).into_iter().collect();
        list.pop_first() == Some(element)
    }

    fn from_iter(list: Vec<u32>) -> bool {
    let mut mut_list = list.clone(); // can't get mutable values from quickcheck.
    let from_iter: UnsortedList<u32> = mut_list.iter().map(|x| x.clone()).collect();
    let from_collection = {
        let mut collection = UnsortedList::default();
        for x in mut_list.iter() {
            collection.push(x.clone());
        }
        collection
    };

    from_iter.iter().eq(mut_list.iter()) && from_collection.iter().eq(mut_list.iter())
        && list.iter().eq(from_collection.iter())
    }
}
