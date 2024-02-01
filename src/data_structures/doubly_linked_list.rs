use std::{fmt, mem};

pub struct List<T: Default> {
    head: usize,
    tail: usize,
    nodes: Vec<Node<T>>,
    recycled: Vec<usize>,
}

struct Node<T> {
    elem: T,
    next: usize,
    prev: usize,
}

pub struct ListIntoIter<T: Default>(List<T>);

pub struct ListIterator<'n, T> {
    nodes: &'n [Node<T>],
    next: usize,
}

const NIL: usize = usize::MAX;

impl<T: Default> List<T> {
    pub fn new() -> Self {
        Self {
            head: NIL,
            tail: NIL,
            nodes: vec![],
            recycled: vec![],
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let old_head = self.head;
        let new_head = self.allocate_node(elem);

        if old_head == NIL {
            self.head = new_head;
            self.tail = new_head;
            return;
        }

        self.nodes[old_head].prev = new_head;
        self.nodes[new_head].next = old_head;

        self.head = new_head;
    }

    fn allocate_node(&mut self, elem: T) -> usize {
        let new = Node::new(elem);

        if let Some(idx) = self.recycled.pop() {
            self.nodes[idx] = new;
            idx
        } else {
            self.nodes.push(new);
            self.nodes.len() - 1
        }
    }

    pub fn push_back(&mut self, elem: T) {
        let old_tail = self.tail;
        let new_tail = self.allocate_node(elem);

        if old_tail == NIL {
            self.head = new_tail;
            self.tail = new_tail;
            return;
        }

        self.nodes[old_tail].next = new_tail;
        self.nodes[new_tail].prev = old_tail;

        self.tail = new_tail;
    }

    pub fn front(&self) -> Option<&T> {
        match self.head {
            NIL => None,
            idx => Some(&self.nodes[idx].elem),
        }
    }

    pub fn back(&self) -> Option<&T> {
        match self.tail {
            NIL => None,
            idx => Some(&self.nodes[idx].elem),
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let old_head = self.head;

        if old_head == NIL {
            return None;
        }

        let sentinel = Node::new(T::default());
        let node = mem::replace(&mut self.nodes[old_head], sentinel);

        self.recycled.push(old_head);

        self.head = node.next;

        if node.next == NIL {
            self.tail = NIL;
        }

        Some(node.elem)
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let old_tail = self.tail;

        if old_tail == NIL {
            return None;
        }

        let sentinel = Node::new(T::default());
        let node = mem::replace(&mut self.nodes[old_tail], sentinel);

        self.recycled.push(old_tail);

        self.tail = node.prev;

        if node.prev == NIL {
            self.head = NIL;
        }

        Some(node.elem)
    }

    pub fn iter(&self) -> ListIterator<T> {
        ListIterator {
            nodes: self.nodes.as_slice(),
            next: self.head,
        }
    }
}

impl<T: Default> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Default> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = ListIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIntoIter(self)
    }
}

impl<'n, T: Default> IntoIterator for &'n List<T> {
    type Item = &'n T;
    type IntoIter = ListIterator<'n, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: fmt::Display + Default> fmt::Display for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for elem in self {
            write!(f, "{} → ", elem)?;
        }

        write!(f, "nil")
    }
}

impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Self {
            elem,
            next: NIL,
            prev: NIL,
        }
    }
}

impl<T: Default> Iterator for ListIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<'n, T> Iterator for ListIterator<'n, T> {
    type Item = &'n T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            NIL => None,
            idx => {
                self.next = self.nodes[idx].next;
                Some(&self.nodes[idx].elem)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn push_front() {
        let mut list = List::new();

        assert_eq!(list.front(), None);

        list.push_front(1);
        assert_eq!(list.front(), Some(&1));
        list.push_front(2);
        assert_eq!(list.front(), Some(&2));
        list.push_front(3);
        assert_eq!(list.front(), Some(&3));

        let collection = list.iter().map(|elem| *elem).collect::<Vec<_>>();
        assert_eq!(collection, vec![3, 2, 1]);
    }

    #[test]
    fn push_back() {
        let mut list = List::new();

        assert_eq!(list.back(), None);

        list.push_back(1);
        assert_eq!(list.back(), Some(&1));
        list.push_back(2);
        assert_eq!(list.back(), Some(&2));
        list.push_back(3);
        assert_eq!(list.back(), Some(&3));

        let collection = list.iter().map(|elem| *elem).collect::<Vec<_>>();
        assert_eq!(collection, vec![1, 2, 3]);
    }

    #[test]
    fn pop_front() {
        let mut list = List::new();

        assert_eq!(list.pop_front(), None);

        list.push_front(1);
        assert_eq!(list.pop_front(), Some(1));
        list.push_front(2);
        assert_eq!(list.pop_front(), Some(2));
        list.push_front(3);
        assert_eq!(list.pop_front(), Some(3));

        assert_eq!(list.iter().next(), None);
    }

    #[test]
    fn pop_back() {
        let mut list = List::new();

        assert_eq!(list.pop_back(), None);

        list.push_back(1);
        assert_eq!(list.pop_back(), Some(1));
        list.push_back(2);
        assert_eq!(list.pop_back(), Some(2));
        list.push_back(3);
        assert_eq!(list.pop_back(), Some(3));

        assert_eq!(list.iter().next(), None);
    }

    #[test]
    fn drop() {
        let mut list = List::new();

        for i in 0..1000000 {
            list.push_back(i);
        }
    }

    #[test]
    fn recycle() {
        let mut list = List::new();

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        list.push_back(5);

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));

        list.push_back(6);
        list.push_back(7);

        let iter = list.iter().map(|elem| *elem).collect::<Vec<_>>();
        let data = list.nodes.iter().map(|node| node.elem).collect::<Vec<_>>();

        assert_eq!(iter, vec![3, 4, 5, 6, 7]);
        assert_eq!(data, vec![7, 6, 3, 4, 5]);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let mut iter = list.into_iter();

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }
}
