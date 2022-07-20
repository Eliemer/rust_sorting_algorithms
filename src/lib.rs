pub mod bubble_sorts;
pub mod helpers;
pub mod simple_sorts;

#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;
