use std::fmt::Debug;
use std::{mem, ptr};

// NOTE: invariant over T, should use `Option<NonNull<Node<T>>>` or `*const Node<T>` to be covariant

pub struct FibHeap<T: PartialOrd> {
    min: *mut Node<T>,
    all_nodes: usize,
    root_list: usize,
}

struct Node<T> {
    parent: *const Node<T>,
    left: *mut Node<T>,
    right: *mut Node<T>,
    children: *mut Node<T>,
    num_children: usize,
    lost_child: bool,
    data: T,
}

impl<T: PartialOrd> FibHeap<T> {
    pub fn new() -> Self {
        Self {
            min: ptr::null_mut(),
            all_nodes: 0,
            root_list: 0,
        }
    }

    pub fn insert(&mut self, data: T) {
        let node = Box::new(Node::new(data));
        let ptr = Box::into_raw(node);

        self.insert_ptr(ptr);

        self.all_nodes += 1;
    }

    fn insert_ptr(&mut self, node: *mut Node<T>) {
        if self.min.is_null() {
            self.min = node;

            unsafe {
                (*node).left = node;
                (*node).right = node;
            }

            self.root_list = 1;
            return;
        }

        self.insert_to_root_list(node);

        unsafe {
            if (*node).data < (*self.min).data {
                self.min = node;
            }
        }
    }

    fn insert_to_root_list(&mut self, node: *mut Node<T>) {
        let list = self.min;

        Node::insert_to_list(list, node);

        self.root_list += 1;
    }

    pub fn minimum(&self) -> Option<&T> {
        unsafe { self.min.as_ref().map(|n| &n.data) }
    }

    pub fn merge(&mut self, mut other: Self) {
        if self.min.is_null() {
            self.min = other.min;
            self.all_nodes = other.all_nodes;
            self.root_list = other.root_list;
            return;
        }

        if other.min.is_null() {
            return;
        }

        let this = self.min;
        let conc = other.min;

        // ↶ this ⇄ oldr ↷ + ↶ oldl ⇄ conc ↷ =
        //
        // ↶ this  oldr ↷
        //    ⇅      ⇅
        // ↶ conc  oldl ↷
        unsafe {
            let oldr = (*this).right;
            let oldl = (*conc).left;

            (*this).right = conc;
            (*conc).left = this;
            (*oldr).left = oldl;
            (*oldl).right = oldr;

            if (*conc).data < (*this).data {
                self.min = conc;
            }
        }

        self.all_nodes += other.all_nodes;
        self.root_list += other.root_list;

        other.min = ptr::null_mut();
        other.all_nodes = 0;
        other.root_list = 0;
    }

    pub fn extract_min(&mut self) -> Option<T> {
        let x = self.min;

        if x.is_null() {
            return None;
        }

        self.move_children_to_root(x);
        self.remove_from_root_list(x);

        let x_right = unsafe { (*x).right };

        if x == x_right {
            self.min = ptr::null_mut();
        } else {
            self.min = x_right;

            self.consolidate();
        }

        self.all_nodes -= 1;

        let node = unsafe { Box::from_raw(x) };

        Some(node.data)
    }

    fn move_children_to_root(&mut self, x: *mut Node<T>) {
        unsafe {
            let num_children = (*x).num_children;
            let mut y = (*x).children;

            for _ in 0..num_children {
                let next = (*y).right;

                self.insert_to_root_list(y);

                (*y).parent = ptr::null_mut();

                y = next;
            }
        }
    }

    fn remove_from_root_list(&mut self, x: *mut Node<T>) {
        unsafe {
            let left = (*x).left;
            let right = (*x).right;

            (*left).right = right;
            (*right).left = left;
        }

        self.root_list -= 1;
    }

    fn consolidate(&mut self) {
        let mut a = vec![ptr::null_mut::<Node<T>>(); self.all_nodes + 1];
        let mut w = self.min;

        for _ in 0..self.root_list {
            let mut x = w;
            let mut c = unsafe { (*x).num_children };
            let next = unsafe { (*w).right };

            while !a[c].is_null() {
                let mut y = a[c];

                if unsafe { (*x).data > (*y).data } {
                    mem::swap(&mut x, &mut y);
                }

                self.link(y, x);

                a[c] = ptr::null_mut();
                c += 1;
            }

            a[c] = x;

            w = next;
        }

        self.min = ptr::null_mut();

        for node in a {
            if node.is_null() {
                continue;
            }

            self.insert_ptr(node);
        }
    }

    fn link(&mut self, y: *mut Node<T>, x: *mut Node<T>) {
        self.remove_from_root_list(y);

        Node::move_to_child_list(x, y);

        unsafe {
            (*y).lost_child = false;
        }
    }
}

impl<T: PartialOrd + Debug> FibHeap<T> {
    #[allow(unused)]
    fn print_list(&self) {
        Node::print_list(self.min, self.root_list, true);
    }
}

impl<T: PartialOrd> Default for FibHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: PartialOrd> Drop for FibHeap<T> {
    fn drop(&mut self) {
        while self.extract_min().is_some() {}
    }
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self {
            parent: ptr::null_mut(),
            left: ptr::null_mut(),
            right: ptr::null_mut(),
            children: ptr::null_mut(),
            num_children: 0,
            lost_child: false,
            data,
        }
    }

    fn insert_to_list(list: *mut Self, node: *mut Self) {
        // [↶ list ⇄ oldr ↷] to [↶ list ⇄ node ⇄ oldr ↷]
        // or [⮎ list ⮌] to [↶ list ⇄ node ↷]
        unsafe {
            let oldr = (*list).right;

            (*list).right = node;
            (*node).right = oldr;
            (*node).left = list;
            (*oldr).left = node;
        }
    }

    fn move_to_child_list(x: *mut Self, y: *mut Self) {
        unsafe {
            if (*x).children.is_null() {
                (*x).children = y;

                (*y).left = y;
                (*y).right = y;
            } else {
                Self::insert_to_list((*x).children, y);
            }

            (*x).num_children += 1;
            (*y).parent = x;
        }
    }
}

impl<T: Debug> Node<T> {
    fn print_list(mut x: *mut Self, len: usize, root: bool) {
        if x.is_null() {
            print!("null");
            return;
        }

        let mut i = 0;

        loop {
            if i >= len {
                break;
            }

            Self::print_node(x);

            x = unsafe { (*x).right };

            i += 1;

            if i < len {
                print!(" ");
            }
        }

        if root {
            println!();
        }
    }

    fn print_node(x: *mut Self) {
        unsafe {
            print!("{:?}", (*x).data);

            if (*x).num_children != 0 {
                print!("[");
                Self::print_list((*x).children, (*x).num_children, false);
                print!("]");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        println!();

        let mut h = FibHeap::new();

        h.insert(4);
        h.insert(2);
        h.insert(5);
        h.insert(1);
        h.insert(3);

        assert_eq!(Some(&1), h.minimum());
        assert_eq!(Some(1), h.extract_min());
        assert_eq!(Some(2), h.extract_min());
        assert_eq!(Some(&3), h.minimum());
        assert_eq!(Some(3), h.extract_min());
        assert_eq!(Some(4), h.extract_min());
        assert_eq!(Some(&5), h.minimum());
        assert_eq!(Some(5), h.extract_min());
        assert_eq!(None, h.minimum());
        assert_eq!(None, h.extract_min());
    }

    #[test]
    fn with_merge() {
        println!();

        let mut h = FibHeap::new();

        h.insert(5);
        h.insert(3);
        h.insert(6);
        h.insert(2);
        h.insert(4);

        assert_eq!(Some(&2), h.minimum());

        let mut h2 = FibHeap::new();

        h2.insert(7);
        h2.insert(1);
        h2.insert(8);

        assert_eq!(Some(&1), h2.minimum());

        h.merge(h2);

        assert_eq!(Some(&1), h.minimum());

        for x in 1..=8 {
            assert_eq!(Some(x), h.extract_min());
        }

        assert_eq!(None, h.minimum());
        assert_eq!(None, h.extract_min());
    }

    #[test]
    fn drop() {
        let nodes = if cfg!(miri) { 500 } else { 10000 };
        let mut l = FibHeap::new();

        for i in 0..nodes {
            l.insert(i);
        }
    }
}
