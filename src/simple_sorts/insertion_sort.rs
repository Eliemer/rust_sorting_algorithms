pub fn sort<T: PartialOrd>(xs: &mut [T]){
    for i in 1..xs.len(){
        for j in (1..=i).rev() {
            if xs[j] >= xs[j-1] { break }
            xs.swap(j, j-1);
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