pub fn sort<T: PartialOrd>(xs: &mut [T]){
    let length = xs.len();

    // to avoid "0 - 1", which stackoverflows with usize
    let i_len = match length {
        0 => 0,
        n => n -1
    };

    for i in 0..i_len {

        let mut min_idx = i;
        for j in i+1..length {
            if xs[j] < xs[min_idx]{
                min_idx = j;
            }
        }

        xs.swap(i, min_idx);
    }
}

#[cfg(test)]
mod tests {
    use super::*; 
    // lol
    use super::super::super::helpers::{is_sorted, elements_match};

    quickcheck! {
        fn test(xs: Vec<isize>) -> bool {
            let mut xs_copy = xs.clone();
            sort(&mut xs_copy);

            is_sorted(&xs_copy) && elements_match(&xs, &xs_copy)
        }
    }
}