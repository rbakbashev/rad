pub struct PriorityQueue<T> {
    data: Vec<T>,
}

impl<T: Copy + PartialOrd> PriorityQueue<T> {
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

    #[allow(clippy::useless_let_if_seq)]
    fn sift_down(&mut self, mut i: usize) {
        let size = self.data.len();

        loop {
            let l = left_child(i);
            let r = right_child(i);
            let mut high_pr = i;

            if l < size && self.has_more_priority(l, i) {
                high_pr = l;
            }

            if r < size && self.has_more_priority(r, high_pr) {
                high_pr = r;
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

        if self.has_more_priority(i, parent(i)) {
            self.sift_up(i);
        } else {
            self.sift_down(i);
        }
    }

    fn sift_up(&mut self, mut i: usize) {
        while i > 0 && self.has_more_priority(i, parent(i)) {
            self.data.swap(i, parent(i));
            i = parent(i);
        }
    }

    pub fn insert(&mut self, new: T) {
        self.data.push(new);

        self.sift_up(self.data.len() - 1);
    }
}

impl<T: Copy + PartialOrd> Default for PriorityQueue<T> {
    fn default() -> Self {
        Self::new()
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let mut q = PriorityQueue::new();

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
        let mut q = PriorityQueue::new();

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
        let mut q = PriorityQueue::new();

        for i in 1..=5 {
            q.insert(i);
        }

        q.change_priority(2, 10);
        assert_eq!(Some(&10), q.max());

        q.change_priority(0, -1);
        assert_eq!(Some(&5), q.max());
    }
}
