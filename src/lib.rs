//! Expandable, hopefully reasonably-cache friendly list types written entirely in safe Rust.

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

mod sorted_utils;
mod unsorted_list;

pub use unsorted_list::sorted_list::SortedList;
pub use unsorted_list::{IntoIter, Iter, UnsortedList};
