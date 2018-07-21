#![feature(test)]

extern crate rand;
extern crate sorted_collections;

mod benchmarks {
    extern crate test;

    use self::test::Bencher;
    use rand::Rng;
    use sorted_collections::SortedList;

    #[bench]
    fn empty(b: &mut Bencher) {
        b.iter(|| 1)
    }

    #[bench]
    fn insert_random_u8(b: &mut Bencher) {
        let mut list = SortedList::default();
        let mut rng = ::rand::IsaacRng::new_unseeded();
        b.iter(|| list.insert(rng.gen::<u8>()));
    }

    #[bench]
    fn insert_random_u64(b: &mut Bencher) {
        let mut list = SortedList::default();
        let mut rng = ::rand::IsaacRng::new_unseeded();
        b.iter(|| list.insert(rng.gen::<u64>()));
    }

    #[bench]
    fn insert_zero_u8(b: &mut Bencher) {
        let mut list: SortedList<u8> = SortedList::default();
        b.iter(|| list.insert(0));
    }

    #[bench]
    fn insert_zero_u64(b: &mut Bencher) {
        let mut list: SortedList<u64> = SortedList::default();
        b.iter(|| list.insert(0));
    }

    #[bench]
    fn insert_sequential_u8(b: &mut Bencher) {
        let mut list = SortedList::default();
        let mut i: u8 = 0;
        b.iter(|| {
            list.insert(i);
            i.wrapping_add(1)
        });
    }

    #[bench]
    fn insert_sequential_u64(b: &mut Bencher) {
        let mut list = SortedList::default();
        let mut i: u64 = 0;
        b.iter(|| {
            list.insert(i);
            i = i + 1
        });
    }
}
