pub fn sort<T: PartialEq + PartialOrd>(xs: &mut [T]) {
    let size = xs.len();
    for i in 0..size {
        for j in 0..size - i - 1 {
            if xs[j] > xs[j + 1] {
                xs.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::helpers::{elements_match, is_sorted};
    use super::sort;

    #[quickcheck]
    fn qc_test(xs: Vec<isize>) -> bool {
        let mut xs_copy = xs.clone();
        sort(&mut xs_copy);

        is_sorted(&xs_copy) && elements_match(&xs, &xs_copy)
    }
}
