//! Order statistic tree based on a Red-Black tree

use std::cmp::Ordering;
use std::fmt;

pub struct OsTree<T: Ord + Default> {
    root: usize,
    nodes: Vec<Node<T>>,
    recycled: Vec<usize>,
}

struct Node<T> {
    red: bool,
    key: T,
    size: usize,
    left: usize,
    right: usize,
    parent: usize,
}

const NIL: usize = 0;

impl<T: Ord + Default> OsTree<T> {
    pub fn new() -> Self {
        let sentinel = Node::new(T::default());
        let nodes = vec![sentinel];
        let root = NIL;
        let recycled = Vec::new();

        Self {
            root,
            nodes,
            recycled,
        }
    }

    pub fn insert(&mut self, key: T) -> usize {
        let x = self.allocate(key);

        let mut par = NIL;
        let mut cur = self.root;

        while cur != NIL {
            par = cur;

            self.nodes[cur].size += 1;

            cur = if self.nodes[x].key < self.nodes[cur].key {
                self.nodes[cur].left
            } else {
                self.nodes[cur].right
            };
        }

        self.nodes[x].red = true;
        self.nodes[x].parent = par;
        self.nodes[x].size = 1;

        if par == NIL {
            self.root = x;
        } else if self.nodes[x].key < self.nodes[par].key {
            self.nodes[par].left = x;
        } else {
            self.nodes[par].right = x;
        }

        self.insert_fixup(x);

        x
    }

    fn allocate(&mut self, key: T) -> usize {
        if let Some(idx) = self.recycled.pop() {
            self.nodes[idx] = Node::new(key);
            return idx;
        }

        self.nodes.push(Node::new(key));
        self.nodes.len() - 1
    }

    fn insert_fixup(&mut self, mut x: usize) {
        loop {
            let xp = self.nodes[x].parent;

            if xp == NIL || !self.nodes[xp].red {
                break;
            }

            let xpp = self.nodes[xp].parent;

            if xp == self.nodes[xpp].left {
                let y = self.nodes[xpp].right;

                if y != NIL && self.nodes[y].red {
                    self.nodes[xp].red = false;
                    self.nodes[y].red = false;
                    self.nodes[xpp].red = true;
                    x = xpp;
                    continue;
                }

                if x == self.nodes[xp].right {
                    x = xp;
                    self.left_rotate(x);
                }

                let xp = self.nodes[x].parent;
                let xpp = self.nodes[xp].parent;

                self.nodes[xp].red = false;
                self.nodes[xpp].red = true;
                self.right_rotate(xpp);
            } else {
                let y = self.nodes[xpp].left;

                if y != NIL && self.nodes[y].red {
                    self.nodes[xp].red = false;
                    self.nodes[y].red = false;
                    self.nodes[xpp].red = true;
                    x = xpp;
                    continue;
                }

                if x == self.nodes[xp].left {
                    x = xp;
                    self.right_rotate(x);
                }

                let xp = self.nodes[x].parent;
                let xpp = self.nodes[xp].parent;

                self.nodes[xp].red = false;
                self.nodes[xpp].red = true;
                self.left_rotate(xpp);
            }
        }

        self.nodes[self.root].red = false;
    }

    fn left_rotate(&mut self, x: usize) {
        let y = self.nodes[x].right;
        let yl = self.nodes[y].left;

        self.nodes[x].right = yl;

        if yl != NIL {
            self.nodes[yl].parent = x;
        }

        self.nodes[y].parent = self.nodes[x].parent;

        let xp = self.nodes[x].parent;

        if xp == NIL {
            self.root = y;
        } else if x == self.nodes[xp].left {
            self.nodes[xp].left = y;
        } else {
            self.nodes[xp].right = y;
        }

        self.nodes[y].left = x;
        self.nodes[x].parent = y;

        self.nodes[y].size = self.nodes[x].size;

        let ls = self.nodes[self.nodes[x].left].size;
        let rs = self.nodes[self.nodes[x].right].size;

        self.nodes[x].size = ls + rs + 1;
    }

    fn right_rotate(&mut self, x: usize) {
        let y = self.nodes[x].left;
        let yr = self.nodes[y].right;

        self.nodes[x].left = yr;

        if yr != NIL {
            self.nodes[yr].parent = x;
        }

        self.nodes[y].parent = self.nodes[x].parent;

        let xp = self.nodes[x].parent;

        if xp == NIL {
            self.root = y;
        } else if x == self.nodes[xp].right {
            self.nodes[xp].right = y;
        } else {
            self.nodes[xp].left = y;
        }

        self.nodes[y].right = x;
        self.nodes[x].parent = y;

        self.nodes[y].size = self.nodes[x].size;

        let ls = self.nodes[self.nodes[x].left].size;
        let rs = self.nodes[self.nodes[x].right].size;

        self.nodes[x].size = ls + rs + 1;
    }

    pub fn has_key(&self, key: &T) -> bool {
        let mut i = self.root;

        while i != NIL {
            match key.cmp(&self.nodes[i].key) {
                Ordering::Less => i = self.nodes[i].left,
                Ordering::Equal => return true,
                Ordering::Greater => i = self.nodes[i].right,
            }
        }

        false
    }

    pub fn delete(&mut self, node: usize) {
        let z = node;
        let y = z;
        let x;

        let mut r = self.nodes[y].red;

        if self.nodes[z].left == NIL {
            x = self.nodes[z].right;
            self.decr_ancestor_sizes(y);
            self.transplant(z, x);
        } else if self.nodes[z].right == NIL {
            x = self.nodes[z].left;
            self.decr_ancestor_sizes(y);
            self.transplant(z, x);
        } else {
            let y = self.min_node_from(self.nodes[z].right);

            self.decr_ancestor_sizes(y);

            r = self.nodes[y].red;
            x = self.nodes[y].right;

            if self.nodes[y].parent == z {
                self.nodes[x].parent = y;
            } else {
                self.transplant(y, self.nodes[y].right);

                self.nodes[y].right = self.nodes[z].right;
                let yr = self.nodes[y].right;
                self.nodes[yr].parent = y;
            }

            self.transplant(z, y);

            self.nodes[y].left = self.nodes[z].left;
            let yl = self.nodes[y].left;
            self.nodes[yl].parent = y;

            self.nodes[y].red = self.nodes[z].red;
        }

        if !r {
            self.delete_fixup(x);
        }

        self.recycled.push(node);
    }

    fn decr_ancestor_sizes(&mut self, y: usize) {
        let mut w = y;

        while w != NIL {
            self.nodes[w].size -= 1;
            w = self.nodes[w].parent;
        }
    }

    fn transplant(&mut self, u: usize, v: usize) {
        let up = self.nodes[u].parent;

        if up == NIL {
            self.root = v;
        } else if u == self.nodes[up].left {
            self.nodes[up].left = v;
        } else {
            self.nodes[up].right = v;
        }

        self.nodes[v].size = self.nodes[u].size;
        self.nodes[v].parent = up;
    }

    pub fn minimum(&self) -> Option<&T> {
        let u = self.min_node_from(self.root);

        if u == NIL {
            None
        } else {
            Some(&self.nodes[u].key)
        }
    }

    fn min_node_from(&self, mut i: usize) -> usize {
        loop {
            if i == NIL || self.nodes[i].left == NIL {
                break;
            }

            i = self.nodes[i].left;
        }

        i
    }

    fn delete_fixup(&mut self, mut x: usize) {
        while x != self.root && !self.nodes[x].red {
            if x == self.nodes[self.nodes[x].parent].left {
                x = self.delete_fixup_left(x);
            } else {
                x = self.delete_fixup_right(x);
            }
        }

        self.nodes[x].red = false;
    }

    fn delete_fixup_left(&mut self, x: usize) -> usize {
        let mut xp = self.nodes[x].parent;
        let mut w = self.nodes[xp].right;

        if self.nodes[w].red {
            self.nodes[w].red = false;
            self.nodes[xp].red = true;

            self.left_rotate(xp);

            xp = self.nodes[x].parent;
            w = self.nodes[xp].right;
        }

        let mut wr = self.nodes[w].right;
        let wl = self.nodes[w].left;

        if !self.nodes[wl].red && !self.nodes[wr].red {
            self.nodes[w].red = true;

            xp
        } else {
            if !self.nodes[wr].red {
                self.nodes[wl].red = false;
                self.nodes[w].red = true;

                self.right_rotate(w);

                xp = self.nodes[x].parent;
                w = self.nodes[xp].right;
                wr = self.nodes[w].right;
            }

            self.nodes[w].red = self.nodes[xp].red;
            self.nodes[xp].red = false;
            self.nodes[wr].red = false;

            self.left_rotate(xp);

            self.root
        }
    }

    fn delete_fixup_right(&mut self, x: usize) -> usize {
        let mut xp = self.nodes[x].parent;
        let mut w = self.nodes[xp].left;

        if self.nodes[w].red {
            self.nodes[w].red = false;
            self.nodes[xp].red = true;

            self.right_rotate(xp);

            xp = self.nodes[x].parent;
            w = self.nodes[xp].left;
        }

        let mut wl = self.nodes[w].left;
        let wr = self.nodes[w].right;

        if !self.nodes[wr].red && !self.nodes[wl].red {
            self.nodes[w].red = true;

            xp
        } else {
            if !self.nodes[wl].red {
                self.nodes[wr].red = false;
                self.nodes[w].red = true;

                self.left_rotate(w);

                xp = self.nodes[x].parent;
                w = self.nodes[xp].left;
                wl = self.nodes[w].left;
            }

            self.nodes[w].red = self.nodes[xp].red;
            self.nodes[xp].red = false;
            self.nodes[wl].red = false;

            self.right_rotate(xp);

            self.root
        }
    }

    pub fn is_empty(&self) -> bool {
        self.root == NIL
    }

    pub fn select(&self, mut k: usize) -> Option<&T> {
        if k == NIL {
            return None;
        }

        let mut x = self.root;
        let mut r = self.nodes[self.nodes[x].left].size + 1;

        while k != r {
            if k > r {
                x = self.nodes[x].right;
                k -= r;
            } else {
                x = self.nodes[x].left;
            }

            r = self.nodes[self.nodes[x].left].size + 1;
        }

        Some(&self.nodes[x].key)
    }

    pub fn get_rank(&self, node: usize) -> usize {
        let mut r = self.nodes[self.nodes[node].left].size + 1;
        let mut y = node;
        let mut yp = self.nodes[y].parent;

        while y != self.root {
            if y == self.nodes[yp].right {
                r += self.nodes[self.nodes[yp].left].size + 1;
            }

            y = yp;
            yp = self.nodes[y].parent;
        }

        r
    }
}

impl<T: fmt::Display + Ord + Default> OsTree<T> {
    fn print_node(
        &self,
        f: &mut fmt::Formatter<'_>,
        x: usize,
        num_prefix: &str,
        ind_prefix: &str,
    ) -> fmt::Result {
        write!(f, "{}", num_prefix)?;

        if x == NIL {
            return writeln!(f, "nil");
        }

        writeln!(f, "{}:{}", self.nodes[x].key, self.nodes[x].size)?;

        let rc_prefix = ind_prefix.to_string() + "├── ";
        let lc_prefix = ind_prefix.to_string() + "└── ";

        let r_ind_prefix = ind_prefix.to_string() + "│   ";
        let l_ind_prefix = ind_prefix.to_string() + "    ";

        let r = self.nodes[x].right;
        let l = self.nodes[x].left;

        if l != NIL && r != NIL {
            self.print_node(f, r, &rc_prefix, &r_ind_prefix)?;
            self.print_node(f, l, &lc_prefix, &l_ind_prefix)?;
        } else if r != NIL {
            self.print_node(f, r, &lc_prefix, &l_ind_prefix)?;
        } else if l != NIL {
            self.print_node(f, l, &lc_prefix, &l_ind_prefix)?;
        }

        Ok(())
    }
}

impl<T: Ord + fmt::Display + Default> Default for OsTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord + fmt::Display + Default> fmt::Display for OsTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.print_node(f, self.root, "", "")
    }
}

impl<T> Node<T> {
    fn new(key: T) -> Self {
        Self {
            red: false,
            key,
            size: 0,
            left: NIL,
            right: NIL,
            parent: NIL,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests;
    use std::collections::HashMap;

    const NUM_NODES: usize = 512;

    #[test]
    #[allow(clippy::needless_range_loop)]
    fn simple() {
        let mut tree = OsTree::new();
        let mut keys = vec![(0, 0); NUM_NODES + 1];

        for i in 1..=NUM_NODES {
            let key = i * 10;
            let idx = tree.insert(key);
            keys[i] = (key, idx);
        }

        for k in 1..=NUM_NODES {
            assert_eq!(Some(k * 10).as_ref(), tree.select(k));
        }

        for (key, idx) in keys.iter().skip(1) {
            let k = tree.get_rank(*idx);
            assert_eq!(k * 10, *key);
        }
    }

    #[test]
    fn randomized() {
        let keys = tests::generate_array_shuffled::<u64>(NUM_NODES);
        let mut tree = OsTree::new();

        for key in keys {
            tree.insert(key + 1);
        }

        for k in 1..=NUM_NODES {
            assert_eq!(Some(k as u64).as_ref(), tree.select(k));
        }
    }

    #[test]
    fn half_deleted() {
        let keys = tests::generate_array_shuffled::<u64>(NUM_NODES);
        let mut tree = OsTree::new();
        let mut idxs = HashMap::new();

        for key in &keys {
            let idx = tree.insert(*key);
            idxs.insert(key, idx);
        }

        let (deleted, retained) = keys.split_at(NUM_NODES / 2);

        for key in deleted {
            let idx = idxs.get(key).expect("element should be present");
            tree.delete(*idx);
        }

        let mut retained = retained.to_vec();
        retained.sort_unstable();

        for (k, r) in retained.iter().enumerate() {
            assert_eq!(Some(r), tree.select(k + 1));
        }

        for key in deleted {
            assert!(!tree.has_key(key));
        }

        for key in retained {
            assert!(tree.has_key(&key));
        }
    }
}
