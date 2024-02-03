use crate::data_structures::heap::BorrowingHeap;

pub fn heap_sort<T: Ord + Copy + std::fmt::Debug>(a: &mut [T]) {
    let h = BorrowingHeap::from_slice(a);
    h.sort();
}

#[cfg(test)]
#[test]
fn test() {
    crate::tests::test_sort(heap_sort);
}
