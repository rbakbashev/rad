fn insertion_sort<T: PartialOrd + Copy>(a: &mut [T]) {
    for i in 1..a.len() {
        let key = a[i];
        let mut j = i as isize - 1;

        while j >= 0 && a[j as usize] > key {
            a[j as usize + 1] = a[j as usize];
            j -= 1;
        }

        a[(j + 1) as usize] = key;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    const LEN: usize = 100;

    #[test]
    fn id() {
        let mut a = utils::generate_array_identical(LEN, 1);
        insertion_sort(&mut a);
        utils::assert_sorted(&a);
    }

    #[test]
    fn ascending() {
        let mut a: Vec<u64> = utils::generate_array_ascending(LEN);
        insertion_sort(&mut a);
        utils::assert_sorted(&a);
    }

    #[test]
    fn descending() {
        let mut a: Vec<u64> = utils::generate_array_descending(LEN);
        insertion_sort(&mut a);
        utils::assert_sorted(&a);
    }

    #[test]
    fn random() {
        let mut a = utils::generate_array_random(LEN, 1, LEN as u64);
        insertion_sort(&mut a);
        utils::assert_sorted(&a);
    }

    #[test]
    fn permutation() {
        let mut a: Vec<u64> = utils::generate_array_permuation(LEN);
        insertion_sort(&mut a);
        utils::assert_sorted(&a);
    }
}
