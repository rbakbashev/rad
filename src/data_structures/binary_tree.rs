//! A very simple, non-self-balancing binary tree

#[derive(Default)]
pub struct BinaryTree<K: Ord + Copy, V> {
    nodes: Vec<Node<K, V>>,
    root: usize,
}

struct Node<K: Ord + Copy, V> {
    left: usize,
    right: usize,
    key: K,
    value: V,
}

const NIL: usize = usize::MAX;

impl<K: Ord + Copy, V> Node<K, V> {
    fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
            left: NIL,
            right: NIL,
        }
    }
}

impl<K: Ord + Copy, V> BinaryTree<K, V> {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            root: NIL,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let new_idx = self.allocate(key, value);

        if self.root == NIL {
            self.root = new_idx;
            return;
        }

        let mut idx = self.root;

        loop {
            let current = &mut self.nodes[idx];

            if key <= current.key {
                if current.left == NIL {
                    current.left = new_idx;
                    break;
                }

                idx = current.left;
            } else {
                if current.right == NIL {
                    current.right = new_idx;
                    break;
                }

                idx = current.right;
            }
        }
    }

    fn allocate(&mut self, key: K, value: V) -> usize {
        let node = Node::new(key, value);

        self.nodes.push(node);

        self.nodes.len() - 1
    }

    pub fn get(&self, key: K) -> Option<&V> {
        let mut idx = self.root;

        loop {
            if idx == NIL {
                return None;
            }

            let current = &self.nodes[idx];

            if key == current.key {
                return Some(&current.value);
            }

            if key <= current.key {
                idx = current.left;
            } else {
                idx = current.right;
            }
        }
    }
}

#[test]
fn simple() {
    let mut tree = BinaryTree::new();

    tree.insert(3, "Three");
    tree.insert(5, "Five");
    tree.insert(7, "Seven");

    assert_eq!(Some(&"Three"), tree.get(3));
    assert_eq!(Some(&"Five"), tree.get(5));
    assert_eq!(Some(&"Seven"), tree.get(7));

    assert_eq!(None, tree.get(2).as_ref());
    assert_eq!(None, tree.get(4).as_ref());
    assert_eq!(None, tree.get(6).as_ref());
}

#[test]
fn all_present() {
    let values = crate::tests::generate_array_shuffled::<u64>(32);
    let mut tree = BinaryTree::new();

    for value in &values {
        tree.insert(value, value);
    }

    for value in &values {
        assert_eq!(Some(&value), tree.get(value));
    }
}
