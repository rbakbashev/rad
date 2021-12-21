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

    #[test]
    fn test() {
        crate::utils::test_sort(insertion_sort);
    }
}
