//! Unfinished x-fast and y-fast tries from Stanford CS166 archives

use std::collections::HashMap;

/// A threaded binary trie where leaves are stored in a doubly-linked list, and all nodes in each
/// level are stored in a hash table.
///
/// Each non-leaf node has a thread pointer to the in-order predecessor or successor of the node,
/// if either 0-pointer (left child) or 1-pointer (right child) is missing, respectively.
///
/// ```text
/// level  prefix
///   0     xxxx                        (_)
///                           0╭────────╯ ╰───────────╮1
///   1     Bxxx              (_)                    (_)
///                     0╭────╯ ╰──────╮1       0╭───╯ ╰───╮1
///   2     BBxx   nil ⇐(_)           (_)     ╔══(_)       (_)╗
///                      │1       0╭──╯ ╰──╮1 ║   │1       0│ ║
///   3     BBBx        (_)     ╔═(_) ╔═══(_) ║╔═(_) ╔═════(_)║
///                  ╭──╯ ╰──╮  ║  │  ║    │  ║║  │  ║      │ ║
///                 0│       │1 ║  │1 ║    │1 ║║  │1 ║      │1║
///                  │       │  ║  │  ║    │  ║║  │  ║      │ ║
///   4     BBBB    (_)     (_) ║ (_) ║   (_) ║║ (_) ║     (_)║
///                  │       │  ║  │  ║    │  ║║  │  ║      │ ║
/// leaves          (x)←───→(x)←╜→(x)←╜──→(x)←╜╜→(x)←╜────→(x)╜
/// ```
pub struct XFastTrie {
    levels: Vec<HashMap<u32, usize>>,
    nodes: Vec<XNode>,
    leaves: Vec<XLeaf>,
    root: usize,
}

struct XNode {
    left: usize,
    right: usize,
    thread: usize,
    parent: usize,
    leaf: usize,
}

struct XLeaf {
    key: u32,
    prev: usize,
    next: usize,
    parent: usize,
}

const NIL: usize = usize::MAX;

impl XFastTrie {
    /// Construct a new x-fast trie that can hold 2^`exp` items.
    pub fn new(exp: u32) -> Self {
        assert!(exp > 0);

        let mut levels = Vec::with_capacity(exp as usize);

        for level in 0..=exp {
            let capacity = 2_usize.pow(level);
            let map = HashMap::with_capacity(capacity);
            levels.push(map);
        }

        Self {
            levels,
            nodes: Vec::new(),
            leaves: Vec::new(),
            root: NIL,
        }
    }

    pub fn successor(&self, key: u32) -> u32 {
        let leaf = self.find_successor(key);

        if leaf == NIL {
            return key;
        }

        self.leaves[leaf].key
    }

    fn find_successor(&self, key: u32) -> usize {
        let (idx, is_leaf) = self.find_longest_prefix(key);

        if is_leaf {
            let lidx = self.nodes[idx].leaf;
            let leaf = &self.leaves[lidx];
            return leaf.next;
        }

        if idx == NIL {
            return NIL;
        }

        let node = &self.nodes[idx];
        let tptr = node.thread;

        if tptr == NIL {
            return NIL;
        }

        let leaf = &self.leaves[tptr];

        if leaf.key < key {
            return leaf.next;
        }

        tptr
    }

    // TODO: this should be done through a binary search of levels
    fn find_longest_prefix(&self, key: u32) -> (usize, bool) {
        let num_levels = self.levels.len() - 1;
        let mut level = 1;
        let mut node_idx = NIL;
        let mut complete = false;

        while level <= num_levels {
            let prefix = key & construct_prefix_mask(level, num_levels);
            let node = self.levels[level].get(&prefix);

            match node {
                Some(idx) => {
                    node_idx = *idx;
                }
                None => break,
            }

            level += 1;
        }

        if level > num_levels {
            complete = true;
        }

        (node_idx, complete)
    }

    pub fn has_key(&self, key: u32) -> bool {
        self.levels
            .last()
            .expect("self.levels should not be empty")
            .contains_key(&key)
    }

    pub fn insert(&mut self, key: u32) {
        if self.has_key(key) {
            return;
        }

        let succ = self.find_successor(key);
        let leaf = self.add_to_trie(key);

        self.connect_to_linked_list(leaf, succ);
        self.update_thread_pointers(leaf, succ);
    }

    fn add_to_trie(&mut self, key: u32) -> usize {
        if self.root == NIL {
            self.root = self.alloc_node();
        }

        let num_levels = self.levels.len() - 1;
        let mut level = 1;
        let mut prev = self.root;

        while level <= num_levels {
            let prefix = key & construct_prefix_mask(level, num_levels);
            let bit = key & construct_bit_mask(level, num_levels);
            let node = self.levels[level].get(&prefix);

            if let Some(idx) = node {
                prev = *idx;
            } else {
                let new = self.alloc_node();

                if bit == 0 {
                    self.nodes[prev].left = new;
                } else {
                    self.nodes[prev].right = new;
                }

                self.nodes[new].parent = prev;
                self.levels[level].insert(prefix, new);

                prev = new;
            }

            level += 1;
        }

        let leaf = self.alloc_leaf(key);

        self.nodes[prev].leaf = leaf;
        self.leaves[leaf].parent = prev;

        leaf
    }

    fn alloc_node(&mut self) -> usize {
        self.nodes.push(XNode::new());
        self.nodes.len() - 1
    }

    fn alloc_leaf(&mut self, key: u32) -> usize {
        self.leaves.push(XLeaf::new(key));
        self.leaves.len() - 1
    }

    fn connect_to_linked_list(&mut self, leaf: usize, succ: usize) {
        if succ == NIL {
            return;
        }

        let prev = self.leaves[succ].prev;

        // prev ⇄ succ
        // prev ⇄ leaf ⇄ succ

        self.leaves[prev].next = leaf;
        self.leaves[leaf].prev = prev;
        self.leaves[leaf].next = succ;
        self.leaves[succ].prev = leaf;
    }

    fn update_thread_pointers(&mut self, leaf: usize, succ: usize) {
        let prev = self.leaves[leaf].prev;

        self.update_thread_ptr_prev(prev, leaf);
        self.update_thread_ptr_leaf(prev, leaf, succ);
        self.update_thread_ptr_succ(leaf, succ);
    }

    fn update_thread_ptr_prev(&mut self, prev: usize, leaf: usize) {
        if prev == NIL {
            return;
        }

        let mut it = self.leaves[prev].parent;

        while it != NIL {
            if self.nodes[it].right == NIL {
                self.nodes[it].thread = leaf;
            }

            it = self.nodes[it].parent;
        }
    }

    fn update_thread_ptr_leaf(&mut self, prev: usize, leaf: usize, succ: usize) {
        let mut it = self.leaves[leaf].parent;

        while it != NIL {
            let leaf = &mut self.nodes[it];

            if leaf.left == NIL {
                leaf.thread = prev;
            } else if leaf.right == NIL {
                leaf.thread = succ;
            }

            it = leaf.parent;
        }
    }

    fn update_thread_ptr_succ(&mut self, leaf: usize, succ: usize) {
        if succ == NIL {
            return;
        }

        let mut it = self.leaves[succ].parent;

        while it != NIL {
            if self.nodes[it].left == NIL {
                self.nodes[it].thread = leaf;
            }

            it = self.nodes[it].parent;
        }
    }

    pub fn dump_as_graphviz(&self) -> String {
        let mut t = String::new();

        t.push_str("digraph g {\n");
        t.push_str("graph [ rankdir = \"TB\" ]\n");

        t.push_str(&format!("{} [ label = \"nil\" ]\n", NIL));

        for (i, node) in self.nodes.iter().enumerate() {
            t.push_str(&format!("{i}\n"));

            if node.left != NIL {
                t.push_str(&format!("{i} -> {} [label = 0]\n", node.left));
            }

            if node.right != NIL {
                t.push_str(&format!("{i} -> {} [label = 1]\n", node.right));
            }

            if node.thread != NIL {
                t.push_str(&format!("{i} -> {}\n", node.thread));
            }

            if node.leaf != NIL {
                t.push_str(&format!("{i} -> leaf{}\n", node.leaf));
            }
        }

        for (i, leaf) in self.leaves.iter().enumerate() {
            t.push_str(&format!(
                "leaf{i} [ label = \"{:04b}\" shape=box ]\n",
                leaf.key
            ));
            t.push_str(&format!("leaf{i} -> {}\n", leaf.prev));
            t.push_str(&format!("leaf{i} -> {}\n", leaf.next));
        }

        t.push('}');

        t
    }
}

const fn construct_prefix_mask(mut len: usize, max: usize) -> u32 {
    let mut mask = 0;

    while len != 0 {
        mask |= 1 << (max - len);
        len -= 1;
    }

    mask
}

const fn construct_bit_mask(pos: usize, max: usize) -> u32 {
    if pos == 0 {
        return 0;
    }

    1 << (max - pos)
}

impl XNode {
    fn new() -> Self {
        Self {
            left: NIL,
            right: NIL,
            thread: NIL,
            parent: NIL,
            leaf: NIL,
        }
    }
}

impl XLeaf {
    fn new(key: u32) -> Self {
        Self {
            key,
            prev: NIL,
            next: NIL,
            parent: NIL,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn masks() {
        assert_eq!(0b0000, construct_prefix_mask(0, 4));
        assert_eq!(0b1000, construct_prefix_mask(1, 4));
        assert_eq!(0b1100, construct_prefix_mask(2, 4));
        assert_eq!(0b1111, construct_prefix_mask(4, 4));

        assert_eq!(0b0000, construct_bit_mask(0, 4));
        assert_eq!(0b1000, construct_bit_mask(1, 4));
        assert_eq!(0b0100, construct_bit_mask(2, 4));
        assert_eq!(0b0001, construct_bit_mask(4, 4));
    }

    #[test]
    fn simple() {
        let mut x = XFastTrie::new(4);

        x.insert(0b1101);
        x.insert(0b0101);
        x.insert(0b1010);
        x.insert(0b1011);
        x.insert(0b1000);
        x.insert(0b1000);
    }
}
