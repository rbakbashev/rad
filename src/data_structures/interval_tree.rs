use std::cmp::max;
use std::fmt::Display;

struct Node<T> {
    low: T,
    high: T,
    max: T,
    left: usize,
    right: usize,
    height: i32,
}

pub struct IntervalTree<T> {
    root: usize,
    nodes: Vec<Node<T>>,
}

impl<T: Copy> Node<T> {
    fn new(low: T, high: T) -> Self {
        Self {
            low,
            high,
            max: high,
            left: usize::MAX,
            right: usize::MAX,
            height: 1,
        }
    }
}

impl<T: Ord + Copy + Display> IntervalTree<T> {
    pub fn new() -> Self {
        Self {
            root: usize::MAX,
            nodes: Vec::new(),
        }
    }

    pub fn insert(&mut self, low: T, high: T) {
        self.root = self.insert_at(self.root, low, high);
    }

    fn insert_at(&mut self, idx: usize, low: T, high: T) -> usize {
        if idx == usize::MAX {
            let new = self.nodes.len();
            self.nodes.push(Node::new(low, high));
            return new;
        }

        if low < self.nodes[idx].low {
            self.nodes[idx].left = self.insert_at(self.nodes[idx].left, low, high);
        } else {
            self.nodes[idx].right = self.insert_at(self.nodes[idx].right, low, high);
        }

        self.balance(idx)
    }

    fn balance(&mut self, x: usize) -> usize {
        self.update_height(x);

        let d = self.diff(x);

        if d > 1 {
            if self.diff(self.nodes[x].right) < 0 {
                self.nodes[x].right = self.right_rotate(self.nodes[x].right);
            }

            self.left_rotate(x)
        } else if d < -1 {
            if self.diff(self.nodes[x].left) > 0 {
                self.nodes[x].left = self.left_rotate(self.nodes[x].left);
            }

            self.right_rotate(x)
        } else {
            x
        }
    }

    fn right_rotate(&mut self, x: usize) -> usize {
        let y = self.nodes[x].left;

        self.nodes[x].left = self.nodes[y].right;
        self.nodes[y].right = x;

        self.update_height(x);
        self.update_max(x, y);

        y
    }

    fn left_rotate(&mut self, x: usize) -> usize {
        let y = self.nodes[x].right;

        self.nodes[x].right = self.nodes[y].left;
        self.nodes[y].left = x;

        self.update_height(x);
        self.update_max(x, y);

        y
    }

    fn update_max(&mut self, x: usize, y: usize) {
        self.nodes[y].max = self.nodes[x].max;

        let l = self.nodes[x].left;
        let r = self.nodes[x].right;

        if l != usize::MAX {
            self.nodes[x].max = max(self.nodes[x].high, self.nodes[l].max);
        }

        if r != usize::MAX {
            self.nodes[x].max = max(self.nodes[x].high, self.nodes[r].max);
        }
    }

    fn update_height(&mut self, x: usize) {
        let lh = self.height(self.nodes[x].left);
        let rh = self.height(self.nodes[x].right);

        self.nodes[x].height = 1 + i32::max(lh, rh);
    }

    fn height(&self, x: usize) -> i32 {
        if x == usize::MAX {
            return 0;
        }

        self.nodes[x].height
    }

    fn diff(&self, x: usize) -> i32 {
        self.height(self.nodes[x].right) - self.height(self.nodes[x].left)
    }

    pub fn search(&self, low: T, high: T) -> Option<(T, T)> {
        let mut x = self.root;

        while x != usize::MAX
            && !intervals_overlap(low, high, self.nodes[x].low, self.nodes[x].high)
        {
            let left = self.nodes[x].left;

            if left != usize::MAX && self.nodes[left].max >= low {
                x = left;
            } else {
                x = self.nodes[x].right;
            }
        }

        if x == usize::MAX {
            None
        } else {
            Some((self.nodes[x].low, self.nodes[x].high))
        }
    }
}

impl<T: Ord + Copy + Display> Default for IntervalTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

fn intervals_overlap<T: Ord + Copy>(x0: T, x1: T, y0: T, y1: T) -> bool {
    x0 <= y1 && y0 <= x1
}

#[test]
fn find_intervals() {
    let mut tree = IntervalTree::new();

    tree.insert(26, 26);
    tree.insert(25, 30);
    tree.insert(19, 20);
    tree.insert(17, 19);
    tree.insert(16, 21);
    tree.insert(15, 23);
    tree.insert(8, 9);
    tree.insert(6, 10);
    tree.insert(5, 8);
    tree.insert(0, 3);

    assert_eq!(tree.search(19, 19), Some((17, 19)));
    assert_eq!(tree.search(27, 29), Some((25, 30)));
    assert_eq!(tree.search(15, 15), Some((15, 23)));
    assert_eq!(tree.search(6, 9), Some((6, 10)));
    assert_eq!(tree.search(24, 24), None);
    assert_eq!(tree.search(11, 14), None);
}
