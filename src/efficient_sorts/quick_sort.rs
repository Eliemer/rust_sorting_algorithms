pub fn sort<T: PartialOrd>(xs: &mut [T]) {
    let hi = match xs.len() {
        0 => 0,
        l => l - 1,
    };
    quicksort(xs, 0, hi);
}

// hi is max(xs.len() - 1, 0)
fn quicksort<T: PartialOrd>(xs: &mut [T], lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }

    let pivot_idx = partition(xs, lo, hi);

    let left_hi = if pivot_idx == 0 { 0 } else { pivot_idx - 1 };
    let right_lo = pivot_idx + 1;

    quicksort(xs, lo, left_hi);
    quicksort(xs, right_lo, hi);
}

fn partition<T: PartialOrd>(xs: &mut [T], lo: usize, hi: usize) -> usize {
    let mut i = if lo == 0 { 0 } else { lo - 1 };
    for j in lo..hi {
        if xs[j] <= xs[hi] {
            xs.swap(i, j);
            i += 1;
        }
    }

    xs.swap(i, hi);
    return i;
}

#[cfg(test)]
mod tests {
    use super::super::super::helpers::{elements_match, is_sorted};
    use super::{partition, sort};

    #[test]
    fn basic_test() {
        let mut xs = [0, -1];
        sort(&mut xs);
        assert!(is_sorted(&mut xs));
    }

    #[test]
    fn test_partition() {
        let arr = [1, 5, 2, 4, 3];
        let i = partition(&mut arr.clone(), 0, 4);
        assert!(i == 2);
    }

    #[quickcheck]
    fn qc_test(xs: Vec<isize>) -> bool {
        let mut xs_copy = xs.clone();
        sort(&mut xs_copy);

        is_sorted(&xs_copy) && elements_match(&xs, &xs_copy)
    }
}
