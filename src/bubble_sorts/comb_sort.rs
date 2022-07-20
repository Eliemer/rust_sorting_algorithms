pub fn sort<T: PartialOrd>(xs: &mut [T]) {
    let shrink_factor = 1.3_f64;
    let mut gap = xs.len();
    let mut sorted = false;

    while !sorted {
        gap = f64::floor(gap as f64 / shrink_factor) as usize;

        if gap <= 1 {
            gap = 1;
            sorted = true;
        }

        let bound = match xs.len() {
            0 => 0,
            l => l - gap,
        };

        for i in 0..bound {
            if xs[i] > xs[i + gap] {
                xs.swap(i, i + gap);
                sorted = false;
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
