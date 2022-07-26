pub fn sort<T: PartialOrd>(xs: &mut [T]) {
    for i in 0..xs.len() {
        for j in i + 1..xs.len() {
            if xs[i] > xs[j] {
                xs.swap(i, j);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::{elements_match, is_sorted};
    use super::sort;

    #[quickcheck]
    fn qc_test(xs: Vec<isize>) -> bool {
        let mut xs_copy = xs.clone();
        sort(&mut xs_copy);

        is_sorted(&xs_copy) && elements_match(&xs, &xs_copy)
    }
}
