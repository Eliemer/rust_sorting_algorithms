mod simple_sorts;
mod helpers;
pub mod bubble_sorts;

use simple_sorts::{insertion_sort, selection_sort};
use bubble_sorts::{bubble_sort};
use helpers::is_sorted;

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck;


fn main() {
    let mut xs = [1,2,3,4];

    xs.swap(0, 2);
    println!("{:?}", xs);

    let ys = [0, -1, 1];
    println!("unsorted: {:?}", &ys);
    
    let mut ys_copy = ys.clone();
    insertion_sort::sort(&mut ys_copy);
    assert!(is_sorted(&ys_copy), "insertion sort is not ordered\n{:?}", &ys_copy);

    let mut ys_copy = ys.clone();
    selection_sort::sort(&mut ys_copy);
    assert!(is_sorted(&ys_copy), "insertion sort is not ordered\n{:?}", &ys_copy);

    ys_copy = ys.clone();
    bubble_sort::sort(&mut ys_copy);
    assert!(is_sorted(&ys_copy), "bubble sort is not ordered");
}
