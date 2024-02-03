pub fn selection_sort<T: PartialOrd>(a: &mut [T]) {
    for i in 0..a.len() - 1 {
        let mut smallest = i;

        for j in (i + 1)..a.len() {
            if a[j] < a[smallest] {
                smallest = j;
            }
        }

        a.swap(i, smallest);
    }
}

#[test]
fn test() {
    crate::tests::test_sort(selection_sort);
}
