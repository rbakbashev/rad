#![allow(clippy::module_name_repetitions)]

use std::mem;

#[derive(Debug)]
pub struct BorrowingHeap<'d, T>
where
    T: Ord,
{
    len: usize,
    data: &'d mut [T],
}

impl<'d, T> BorrowingHeap<'d, T>
where
    T: Ord + Copy,
{
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

    fn parent(i: usize) -> usize {
        i / 2
    }

    fn left_child(i: usize) -> usize {
        2 * i
    }

    fn right_child(i: usize) -> usize {
        2 * i + 1
    }

    fn has_more_priority(&self, x: usize, y: usize) -> bool {
        self.data[x] > self.data[y]
    }

    fn sift_down(&mut self, mut i: usize) {
        loop {
            let l = Self::left_child(i);
            let r = Self::right_child(i);

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

    fn sift_up(&mut self, mut i: usize) {
        let mut p = Self::parent(i);

        while i > 0 && self.has_more_priority(i, p) {
            self.data.swap(i, p);
            i = p;
            p = Self::parent(i);
        }
    }

    pub fn inc_key_priority(&mut self, i: usize, new: T) {
        self.data[i] = new;
        self.sift_up(i);
    }

    pub fn insert(&mut self, new: T) {
        self.len += 1;
        self.inc_key_priority(self.len - 1, new);
    }

    pub fn remove(&mut self, i: usize) -> T {
        let elem = self.data[i];
        self.data.swap(i, self.len - 1);
        self.len -= 1;

        if self.has_more_priority(i, Self::parent(i)) {
            self.sift_up(i);
        } else {
            self.sift_down(i);
        }

        elem
    }

    pub fn sort(mut self) {
        for i in (1..self.len).rev() {
            self.data.swap(0, i);

            self.len -= 1;
            self.sift_down(0);
        }
    }
}
