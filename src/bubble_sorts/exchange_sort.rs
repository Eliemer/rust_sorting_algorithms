pub fn sort<T: PartialOrd>(xs: &mut [T]){
    for i in 0..xs.len() {
        for j in i+1 .. xs.len() {
            if xs[i] > xs[j] {
                xs.swap(i, j);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::sort;
    use super::super::super::helpers::{is_sorted, elements_match};

    quickcheck! {
        fn test(xs: Vec<isize>) -> bool {
            let mut xs_copy = xs.clone();
            sort(&mut xs_copy);

            is_sorted(&xs_copy) && elements_match(&xs, &xs_copy)
        }
    }
}