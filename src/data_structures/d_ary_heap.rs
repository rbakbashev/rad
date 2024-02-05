pub struct DAryHeap<T, const D: usize> {
    data: Vec<T>,
}

impl<T: Copy + PartialOrd, const D: usize> DAryHeap<T, D> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn max(&self) -> Option<&T> {
        self.data.first()
    }

    pub fn extract_max(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }

        let max = self.data.swap_remove(0);

        self.sift_down(0);

        Some(max)
    }

    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    fn parent(i: usize) -> usize {
        let max_int_f64 = 2_usize.pow(f64::MANTISSA_DIGITS) - 1;

        assert!(i <= max_int_f64);
        assert!(D <= max_int_f64);

        ((i as f64 - 1.0) / D as f64) as usize
    }

    fn kth_child(i: usize, k: usize) -> usize {
        D * i + k
    }

    fn sift_down(&mut self, mut i: usize) {
        let size = self.data.len();

        loop {
            let mut high_pr = i;

            for k in 1..=D {
                let c = Self::kth_child(i, k);

                if c < size && self.has_more_priority(c, high_pr) {
                    high_pr = c;
                }
            }

            if high_pr == i {
                break;
            }

            self.data.swap(i, high_pr);

            i = high_pr;
        }
    }

    fn has_more_priority(&self, x: usize, y: usize) -> bool {
        self.data[x] > self.data[y]
    }

    pub fn change_priority(&mut self, i: usize, new: T) {
        self.data[i] = new;

        if self.has_more_priority(i, Self::parent(i)) {
            self.sift_up(i);
        } else {
            self.sift_down(i);
        }
    }

    fn sift_up(&mut self, mut i: usize) {
        while i > 0 && self.has_more_priority(i, Self::parent(i)) {
            self.data.swap(i, Self::parent(i));
            i = Self::parent(i);
        }
    }

    pub fn insert(&mut self, new: T) {
        self.data.push(new);

        self.sift_up(self.data.len() - 1);
    }
}

impl<T: Copy + PartialOrd, const D: usize> Default for DAryHeap<T, D> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const D: usize = 5;

    #[test]
    fn simple() {
        let mut q = DAryHeap::<i32, D>::new();

        assert_eq!(None, q.max());
        assert_eq!(None, q.extract_max());

        q.insert(1);
        assert_eq!(Some(&1), q.max());

        q.insert(2);
        assert_eq!(Some(&2), q.max());

        q.insert(3);
        assert_eq!(Some(&3), q.max());

        assert_eq!(Some(3), q.extract_max());
        assert_eq!(Some(2), q.extract_max());
        assert_eq!(Some(1), q.extract_max());
        assert_eq!(None, q.extract_max());
    }

    #[test]
    fn simple_rev() {
        let mut q = DAryHeap::<i32, D>::new();

        q.insert(3);
        assert_eq!(Some(&3), q.max());

        q.insert(2);
        assert_eq!(Some(&3), q.max());

        q.insert(1);
        assert_eq!(Some(&3), q.max());

        assert_eq!(Some(3), q.extract_max());
        assert_eq!(Some(2), q.extract_max());
        assert_eq!(Some(1), q.extract_max());
        assert_eq!(None, q.extract_max());
    }

    #[test]
    fn change_priority() {
        let mut q = DAryHeap::<i32, D>::new();

        for i in 1..=5 {
            q.insert(i);
        }

        q.change_priority(2, 10);
        assert_eq!(Some(&10), q.max());

        q.change_priority(0, -1);
        assert_eq!(Some(&5), q.max());
    }
}
