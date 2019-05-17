#![feature(test)]

extern crate rand;
extern crate sorted_collections;

use std::collections::{BTreeMap, BTreeSet};
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
    let mut rng = ::rand::thread_rng();
    b.iter(|| list.add(rng.gen::<u8>()));
}

#[bench]
fn insert_random_u64(b: &mut Bencher) {
    let mut list = SortedList::default();
    let mut rng = ::rand::thread_rng();
    b.iter(|| list.add(rng.gen::<u64>()));
}

#[bench]
fn insert_zero_u8(b: &mut Bencher) {
    let mut list: SortedList<u8> = SortedList::default();
    b.iter(|| list.add(0));
}

#[bench]
fn insert_zero_u64(b: &mut Bencher) {
    let mut list: SortedList<u64> = SortedList::default();
    b.iter(|| list.add(0));
}

#[bench]
fn insert_sequential_u8(b: &mut Bencher) {
    let mut list = SortedList::default();
    let mut i: u8 = 0;
    b.iter(|| {
        list.add(i);
        i = i.wrapping_add(1)
    });
}

#[bench]
fn insert_increasing_u64(b: &mut Bencher) {
    let mut list = SortedList::default();
    let mut i: u64 = 0;
    b.iter(|| {
        list.add(i);
        i = i + 1
    });
}

#[bench]
fn insert_increasing_u64_BTreeMap(b: &mut Bencher) {
    let mut list = BTreeMap::new();
    let mut i: u64 = 0;
    b.iter(|| {
        list.insert(i, 0);
        i = i + 1
    })
}

#[bench]
fn insert_increasing_u64_BTreeSet(b: &mut Bencher) {
    let mut list = BTreeSet::new();
    let mut i: u64 = 0;
    b.iter(|| {
        list.insert(i);
        i = i + 1
    })
}

#[bench]
fn insert_decreasing_u64(b: &mut Bencher) {
    let mut list = SortedList::default();
    let mut i: u64 = std::u64::MAX;
    b.iter(|| {
        list.add(i);
        i = i - 1
    });
}

#[bench]
fn insert_decreasing_u64_BTreeMap(b: &mut Bencher) {
    let mut list = BTreeMap::new();
    let mut i: u64 = std::u64::MAX;
    b.iter(|| {
        list.insert(i, 0);
        i = i - 1
    })
}

#[bench]
fn insert_decreasing_u64_BTreeSet(b: &mut Bencher) {
    let mut list = BTreeSet::new();
    let mut i: u64 = std::u64::MAX;
    b.iter(|| {
        list.insert(i);
        i = i - 1
    })
}
