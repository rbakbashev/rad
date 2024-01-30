fn insertion_sort<T: PartialOrd + Copy>(a: &mut [T]) {
    for j in 2..=a.len() {
        let key = a[j - 1];
        let mut i = j - 1;

        while i >= 1 && a[i - 1] > key {
            a[i] = a[i - 1];
            i -= 1;
        }

        a[i] = key;
    }
}

// Same as above except uses 0-indexing
fn insertion_sort_2<T: PartialOrd + Copy>(a: &mut [T]) {
    'outer: for j in 1..a.len() {
        let key = a[j];
        let mut i = j - 1;

        while a[i] > key {
            a[i + 1] = a[i];

            if i == 0 {
                a[i] = key;
                continue 'outer;
            }

            i -= 1;
        }

        a[i + 1] = key;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal() {
        crate::tests::test_sort(insertion_sort);
    }

    #[test]
    fn alternative() {
        crate::tests::test_sort(insertion_sort_2);
    }
}
