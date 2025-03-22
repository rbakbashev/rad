use crate::data_structures::heap::BorrowingHeap;

pub fn heap_sort<T: Ord + Copy>(a: &mut [T]) {
    let heap = BorrowingHeap::from_slice(a);

    heap.sort();
}

#[test]
fn test() {
    crate::tests::test_sort(heap_sort);
}
