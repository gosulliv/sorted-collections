#![feature(test)]

extern crate rand;
extern crate sorted_collections;

mod benchmarks {
    extern crate test;

    use self::test::Bencher;
    use rand::Rng;
    use sorted_collections::UnsortedList;

    #[bench]
    fn empty(b: &mut Bencher) {
        b.iter(|| 1)
    }

    #[bench]
    fn push_random_u8(b: &mut Bencher) {
        let mut list = UnsortedList::default();
        let mut rng = ::rand::IsaacRng::new_unseeded();
        b.iter(|| list.push(rng.gen::<u8>()));
    }

    #[bench]
    fn push_random_u64(b: &mut Bencher) {
        let mut list = UnsortedList::default();
        let mut rng = ::rand::IsaacRng::new_unseeded();
        b.iter(|| list.push(rng.gen::<u64>()));
    }

    #[bench]
    fn push_zero_u8(b: &mut Bencher) {
        let mut list: UnsortedList<u8> = UnsortedList::default();
        b.iter(|| list.push(0));
    }

    #[bench]
    fn push_zero_u64(b: &mut Bencher) {
        let mut list: UnsortedList<u64> = UnsortedList::default();
        b.iter(|| list.push(0));
    }

    #[bench]
    fn push_sequential_u8(b: &mut Bencher) {
        let mut list = UnsortedList::default();
        let mut i: u8 = 0;
        b.iter(|| {
            list.push(i);
            i = i.wrapping_add(1)
        });
    }

    #[bench]
    fn push_sequential_u64(b: &mut Bencher) {
        let mut list = UnsortedList::default();
        let mut i: u64 = 0;
        b.iter(|| {
            list.push(i);
            i = i + 1
        });
    }

    #[bench]
    fn insert_first_i32(b: &mut Bencher) {
        let mut list = UnsortedList::default();
        let mut i: i32 = 0;
        b.iter(|| {
            list.insert(0, i);
            i = i.wrapping_add(1);
        })
    }

    #[bench]
    fn insert_first_i32_vec(b: &mut Bencher) {
        let mut list = Vec::default();
        let mut i: i32 = 0;
        b.iter(|| {
            list.insert(0, i);
            i = i.wrapping_add(1);
        })
    }

    #[bench]
    fn insert_first_i32_large(b: &mut Bencher) {
        let mut list = UnsortedList::default();
        let mut i: i32 = 2 << 30 - 1;
        b.iter(|| {
            list.insert(0, i);
            i += 1;
        })
    }

    #[bench]
    fn insert_first_i32_large_vec(b: &mut Bencher) {
        let mut list = Vec::default();
        let mut i: i32 = 2 << 30 - 1;
        b.iter(|| {
            list.insert(0, i);
            i += 1;
        })
    }

    // I think this is not working right.
    //    #[bench]
    //    fn remove_all(b: &mut Bencher) {
    //        let mut list = UnsortedList::default();
    //        let mut rng = ::rand::IsaacRng::new_unseeded();
    //        for i in 1..100000 {
    //            list.push(rng.gen::<i32>);
    //        b.iter(|| {
    //            rng.choose(list);
    //        });
    //        }
    //    }

}
