//! Red-black tree: a self-balancing binary tree

use std::cmp::Ordering;
use std::fmt;

pub struct RbTree<T: Ord> {
    root: usize,
    nodes: Vec<Node<T>>,
}

struct Node<T> {
    red: bool,
    key: T,
    left: usize,
    right: usize,
    parent: usize,
}

const NIL: usize = usize::MAX;

impl<T: Ord> RbTree<T> {
    pub fn new() -> Self {
        Self {
            root: NIL,
            nodes: Vec::new(),
        }
    }

    pub fn insert(&mut self, key: T) {
        let z = self.allocate(key);

        let mut par = NIL;
        let mut cur = self.root;

        while cur != NIL {
            par = cur;

            cur = if self.nodes[z].key < self.nodes[cur].key {
                self.nodes[cur].left
            } else {
                self.nodes[cur].right
            };
        }

        self.nodes[z].parent = par;

        if par == NIL {
            self.root = z;
        } else if self.nodes[z].key < self.nodes[par].key {
            self.nodes[par].left = z;
        } else {
            self.nodes[par].right = z;
        }

        self.nodes[z].left = NIL;
        self.nodes[z].right = NIL;
        self.nodes[z].red = true;

        self.insert_fixup(z);
    }

    fn allocate(&mut self, key: T) -> usize {
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
}

impl<T: fmt::Display + Ord> RbTree<T> {
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

impl<T: Ord + fmt::Display> Default for RbTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord + fmt::Display> fmt::Display for RbTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.print_node(f, self.root, "", "")
    }
}

impl<T> Node<T> {
    fn new(key: T) -> Self {
        Self {
            red: false,
            key,
            left: NIL,
            right: NIL,
            parent: NIL,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NUM_NODES: u32 = 512;

    #[test]
    fn insert() {
        let mut tree = RbTree::new();

        for i in 0..NUM_NODES {
            tree.insert(i);
        }

        assert_all_present(&tree, NUM_NODES);
    }

    fn assert_all_present(tree: &RbTree<u32>, num: u32) {
        for k in 1..num {
            assert!(tree.has_key(&k));
        }

        assert!(!tree.has_key(&num));
    }
}
