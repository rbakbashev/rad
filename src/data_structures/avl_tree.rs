use std::cmp::Ordering;
use std::fmt;

struct Node<T> {
    key: T,
    left: usize,
    right: usize,
    height: i32,
}

pub struct AvlTree<T> {
    root: usize,
    nodes: Vec<Node<T>>,
}

impl<T> Node<T> {
    fn new(key: T) -> Self {
        Self {
            key,
            left: usize::MAX,
            right: usize::MAX,
            height: 1,
        }
    }
}

impl<T: Ord + fmt::Display> AvlTree<T> {
    pub fn new() -> Self {
        Self {
            root: usize::MAX,
            nodes: Vec::new(),
        }
    }

    pub fn insert(&mut self, key: T) {
        self.root = self.insert_at(self.root, key);
    }

    pub fn has_key(&self, key: T) -> bool {
        self.has_key_at(key, self.root)
    }

    fn insert_at(&mut self, idx: usize, key: T) -> usize {
        if idx == usize::MAX {
            let new = self.nodes.len();
            self.nodes.push(Node::new(key));
            return new;
        }

        if key < self.nodes[idx].key {
            self.nodes[idx].left = self.insert_at(self.nodes[idx].left, key);
        } else {
            self.nodes[idx].right = self.insert_at(self.nodes[idx].right, key);
        }

        self.balance(idx)
    }

    fn balance(&mut self, x: usize) -> usize {
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
            self.update_height(x);

            x
        }
    }

    fn right_rotate(&mut self, x: usize) -> usize {
        let y = self.nodes[x].left;

        self.nodes[x].left = self.nodes[y].right;
        self.nodes[y].right = x;

        self.update_height(x);

        y
    }

    fn left_rotate(&mut self, x: usize) -> usize {
        let y = self.nodes[x].right;

        self.nodes[x].right = self.nodes[y].left;
        self.nodes[y].left = x;

        self.update_height(x);

        y
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

    fn has_key_at(&self, key: T, x: usize) -> bool {
        if x == usize::MAX {
            return false;
        }

        match key.cmp(&self.nodes[x].key) {
            Ordering::Less => self.has_key_at(key, self.nodes[x].left),
            Ordering::Equal => true,
            Ordering::Greater => self.has_key_at(key, self.nodes[x].right),
        }
    }

    fn print_node(
        &self,
        f: &mut fmt::Formatter<'_>,
        x: usize,
        num_prefix: &str,
        ind_prefix: &str,
    ) -> fmt::Result {
        write!(f, "{}", num_prefix)?;

        if x == usize::MAX {
            return writeln!(f, "nil");
        }

        writeln!(f, "{}", self.nodes[x].key)?;

        let rc_prefix = ind_prefix.to_string() + "├── ";
        let lc_prefix = ind_prefix.to_string() + "└── ";

        let r_ind_prefix = ind_prefix.to_string() + "│   ";
        let l_ind_prefix = ind_prefix.to_string() + "    ";

        let r = self.nodes[x].right;
        let l = self.nodes[x].left;

        if l != usize::MAX && r != usize::MAX {
            self.print_node(f, r, &rc_prefix, &r_ind_prefix)?;
            self.print_node(f, l, &lc_prefix, &l_ind_prefix)?;
        } else if r != usize::MAX {
            self.print_node(f, r, &lc_prefix, &l_ind_prefix)?;
        } else if l != usize::MAX {
            self.print_node(f, l, &lc_prefix, &l_ind_prefix)?;
        }

        Ok(())
    }
}

impl<T: Ord + fmt::Display> Default for AvlTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord + fmt::Display> fmt::Display for AvlTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.print_node(f, self.root, "", "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NUM_NODES: u32 = 512;
    const EXP_DEPTH: usize = NUM_NODES.ilog2() as usize;

    #[test]
    fn balanced() {
        let mut tree = AvlTree::new();

        for i in 1..NUM_NODES {
            tree.insert(i);
        }

        assert_eq!(calc_height(&tree, tree.root), EXP_DEPTH);
    }

    #[test]
    fn balanced_rev() {
        let mut tree = AvlTree::new();

        for i in (1..NUM_NODES).rev() {
            tree.insert(i);
        }

        assert_eq!(calc_height(&tree, tree.root), EXP_DEPTH);
    }

    #[test]
    fn balanced_rand() {
        let mut rand = crate::rand::Wyhash64RNG::from_seed(123);
        let mut tree = AvlTree::new();

        for _ in 1..NUM_NODES {
            let num = rand.gen_in_range(0..NUM_NODES.into());
            tree.insert(num);
        }

        assert!(calc_height(&tree, tree.root).abs_diff(EXP_DEPTH) <= 2);
    }

    fn calc_height<T>(tree: &AvlTree<T>, x: usize) -> usize {
        if x == usize::MAX {
            return 0;
        }

        let lh = calc_height(tree, tree.nodes[x].left);
        let rh = calc_height(tree, tree.nodes[x].right);

        1 + usize::max(lh, rh)
    }

    #[test]
    fn all_present() {
        let num = 512;
        let mut tree = AvlTree::new();

        for i in 1..num {
            tree.insert(i);
        }

        for k in 1..num {
            assert!(tree.has_key(k));
        }

        assert!(!tree.has_key(num));
    }

    #[test]
    fn print() {
        let mut tree = AvlTree::default();

        for i in 1..=16 {
            tree.insert(i);
        }

        let printout = format!("{}", tree);
        let expected = "\
8
├── 12
│   ├── 14
│   │   ├── 15
│   │   │   └── 16
│   │   └── 13
│   └── 10
│       ├── 11
│       └── 9
└── 4
    ├── 6
    │   ├── 7
    │   └── 5
    └── 2
        ├── 3
        └── 1
";

        assert_eq!(printout, expected);
    }

    #[test]
    fn print_empty() {
        let tree: AvlTree<u8> = AvlTree::default();

        let printout = format!("{}", tree);
        let expected = "nil\n";

        assert_eq!(printout, expected);
    }

    #[test]
    fn print_single() {
        let mut tree = AvlTree::default();

        tree.insert(5);
        tree.insert(3);

        println!("{tree}");

        let printout = format!("{}", tree);
        let expected = "\
5
└── 3
";

        assert_eq!(printout, expected);
    }
}
