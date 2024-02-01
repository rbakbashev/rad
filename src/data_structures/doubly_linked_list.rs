pub struct List<T> {
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

pub struct ListIterator<'n, T> {
    nodes: &'n [Node<T>],
    next: usize,
}

const NIL: usize = usize::MAX;

impl<T> List<T> {
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

    pub fn iter(&self) -> ListIterator<T> {
        ListIterator {
            nodes: self.nodes.as_slice(),
            next: self.head,
        }
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'n, T> IntoIterator for &'n List<T> {
    type Item = &'n T;
    type IntoIter = ListIterator<'n, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
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
    fn drop() {
        let mut list = List::new();

        for i in 0..1000000 {
            list.push_back(i);
        }
    }
}
