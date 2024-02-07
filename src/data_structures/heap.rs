pub struct BorrowingHeap<'d, T: PartialOrd> {
    len: usize,
    data: &'d mut [T],
}

impl<'d, T: PartialOrd + Copy> BorrowingHeap<'d, T> {
    pub fn from_slice<'s: 'd>(slice: &'s mut [T]) -> Self {
        let slice_len = slice.len();

        let mut heap = Self {
            len: slice_len,
            data: slice,
        };

        for i in (0..=(slice_len / 2)).rev() {
            heap.sift_down(i);
        }

        heap
    }

    fn has_more_priority(&self, x: usize, y: usize) -> bool {
        self.data[x] > self.data[y]
    }

    #[allow(clippy::useless_let_if_seq)]
    fn sift_down(&mut self, mut i: usize) {
        loop {
            let l = left_child(i);
            let r = right_child(i);

            let mut high_pr = i;

            if l < self.len && self.has_more_priority(l, i) {
                high_pr = l;
            }

            if r < self.len && self.has_more_priority(r, high_pr) {
                high_pr = r;
            }

            if high_pr == i {
                break;
            }

            self.data.swap(i, high_pr);

            i = high_pr;
        }
    }

    pub fn sort(mut self) {
        for i in (1..self.len).rev() {
            self.data.swap(0, i);

            self.len -= 1;
            self.sift_down(0);
        }
    }
}

fn left_child(i: usize) -> usize {
    2 * i
}

fn right_child(i: usize) -> usize {
    2 * i + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests;

    fn parent(i: usize) -> usize {
        i / 2
    }

    fn test_single(mut a: Vec<u64>) {
        let heap = BorrowingHeap::from_slice(&mut a);

        for i in 1..heap.data.len() {
            assert!(heap.data[parent(i)] >= heap.data[i]);
        }
    }

    #[test]
    fn heap_property() {
        let len = 1000;
        let arrays = tests::generate_test_arrays(len);

        for (desc, arr) in arrays {
            println!("Array: {}", desc);
            test_single(arr);
        }
    }
}
