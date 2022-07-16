use std::collections::HashMap;
use std::hash::Hash;

/// helpers
pub fn is_sorted<T: PartialOrd>(xs: &[T]) -> bool{
    let length = match xs.len() {
        0 => 0,
        x => x - 1,
    };

    for idx in 0..length {
        if xs[idx] > xs[idx + 1]{
            return false;
        }
    }
    return true;
}

pub fn elements_match<T: Eq + Hash>(xs: &[T], ys: &[T]) -> bool {
    let mut count_xs = HashMap::<&T, i32>::new();
    let mut count_ys = HashMap::<&T, i32>::new();

    for x in xs.iter() {
        let counter = count_xs.entry(x).or_insert(0);
        *counter += 1;
    }

    for y in ys.iter() {
        let counter = count_ys.entry(y).or_insert(0);
        *counter += 1;
    }

    count_xs == count_ys
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_is_sorted() {
        assert!(is_sorted(&[1,2,3,4]));
        assert!(!is_sorted(&[4,3,2,1]));
        assert!(is_sorted(&[0i32; 0]));
    }

    #[test]
    fn verify_elements_match() {
        assert!(elements_match(&[1,2,3], &[1,2,3]));
        assert!(elements_match(&[1,2,3], &[1,3,2]));
        assert!(!elements_match(&[2,2,3], &[1,2,1]));
        assert!(elements_match(&[3,3,1,2,3], &[1,2,3,3,3]));
    }
}