#[cfg(test)]
#[macro_use]
extern crate quickcheck;

//pub use bisect::*;
pub mod sorted_list;
pub mod unsorted_list;
pub mod bisect;
mod jenks_index;

pub use sorted_list::SortedList;
pub use unsorted_list::UnsortedList;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(0 == 0);
    }
}
