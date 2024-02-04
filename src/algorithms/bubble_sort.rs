pub fn bubble_sort<T: PartialOrd>(a: &mut [T]) {
    if a.len() <= 1 {
        return;
    }

    for i in 0..a.len() - 1 {
        for j in ((i + 1)..a.len()).rev() {
            if a[j] < a[j - 1] {
                a.swap(j, j - 1);
            }
        }
    }
}

#[test]
fn test() {
    crate::tests::test_sort(bubble_sort);
}
