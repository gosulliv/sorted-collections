#![feature(test)]

extern crate rand;
extern crate sorted_collections;

extern crate test;

use self::test::Bencher;
use rand::Rng;
use sorted_collections::SortedList;

#[bench]
fn empty(b: &mut Bencher) {
    b.iter(|| 1)
}

#[bench]
fn add_random_u8(b: &mut Bencher) {
    let mut list = SortedList::default();
    let mut rng = ::rand::thread_rng();
    b.iter(|| list.add(rng.gen::<u8>()));
}

#[bench]
fn add_random_u64(b: &mut Bencher) {
    let mut list = SortedList::default();
    let mut rng = ::rand::thread_rng();
    b.iter(|| list.add(rng.gen::<u64>()));
}

#[bench]
fn add_zero_u8(b: &mut Bencher) {
    let mut list: SortedList<u8> = SortedList::default();
    b.iter(|| list.add(0));
}

#[bench]
fn add_zero_u64(b: &mut Bencher) {
    let mut list: SortedList<u64> = SortedList::default();
    b.iter(|| list.add(0));
}

#[bench]
fn add_sequential_u8(b: &mut Bencher) {
    let mut list = SortedList::default();
    let mut i: u8 = 0;
    b.iter(|| {
        list.add(i);
        i = i.wrapping_add(1)
    });
}

#[bench]
fn add_increasing_u64(b: &mut Bencher) {
    let mut list = SortedList::default();
    let mut i: u64 = 0;
    b.iter(|| {
        list.add(i);
        i = i + 1
    });
}
