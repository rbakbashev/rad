use std::ptr;

// NOTE: invariant over T, should use `Option<NonNull<Node<T>>>` or `*const Node<T>` to be covariant

pub struct LinkedList<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    size: usize,
}

struct Node<T> {
    prev: *mut Node<T>,
    next: *mut Node<T>,
    data: T,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            size: 0,
        }
    }

    pub fn push_front(&mut self, data: T) {
        let new_head = Box::new(Node::new(data));
        let ptr_head = Box::into_raw(new_head);

        if self.head.is_null() {
            self.tail = ptr_head;
        } else {
            unsafe {
                (*ptr_head).next = self.head;
                (*self.head).prev = ptr_head;
            }
        }

        self.head = ptr_head;
        self.size += 1;
    }

    pub fn push_back(&mut self, data: T) {
        let new_tail = Box::new(Node::new(data));
        let ptr_tail = Box::into_raw(new_tail);

        if self.tail.is_null() {
            self.head = ptr_tail;
        } else {
            unsafe {
                (*ptr_tail).prev = self.tail;
                (*self.tail).next = ptr_tail;
            }
        }

        self.tail = ptr_tail;
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.head.is_null() {
            return None;
        }

        let head = unsafe { Box::from_raw(self.head) };

        self.head = head.next;

        match unsafe { self.head.as_mut() } {
            Some(new_head) => new_head.prev = ptr::null_mut(),
            None => self.tail = ptr::null_mut(),
        }

        self.size -= 1;

        Some(head.data)
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.tail.is_null() {
            return None;
        }

        let tail = unsafe { Box::from_raw(self.tail) };

        self.tail = tail.prev;

        match unsafe { self.tail.as_mut() } {
            Some(new_tail) => new_tail.next = ptr::null_mut(),
            None => self.head = ptr::null_mut(),
        }

        self.size -= 1;

        Some(tail.data)
    }

    pub fn head(&self) -> Option<&T> {
        unsafe { self.head.as_ref().map(|n| &n.data) }
    }

    pub fn head_mut(&mut self) -> Option<&mut T> {
        unsafe { self.head.as_mut().map(|n| &mut n.data) }
    }

    pub fn tail(&self) -> Option<&T> {
        unsafe { self.tail.as_ref().map(|n| &n.data) }
    }

    pub fn tail_mut(&mut self) -> Option<&mut T> {
        unsafe { self.tail.as_mut().map(|n| &mut n.data) }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self {
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
            data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn front() {
        let mut l = LinkedList::new();

        assert_eq!(0, l.len());
        assert_eq!(None, l.pop_front());
        assert_eq!(None, l.head());

        l.push_front(1);
        assert_eq!(1, l.len());
        assert_eq!(Some(&1), l.head());
        assert_eq!(Some(1), l.pop_front());
        assert_eq!(None, l.head());
        assert_eq!(None, l.pop_front());
        assert_eq!(0, l.len());

        l.push_front(2);
        l.push_front(3);

        assert_eq!(2, l.len());
        assert_eq!(Some(&3), l.head());
        assert_eq!(Some(3), l.pop_front());
        assert_eq!(1, l.len());
        assert_eq!(Some(&2), l.head());
        assert_eq!(Some(2), l.pop_front());
        assert_eq!(0, l.len());
        assert_eq!(None, l.head());
        assert_eq!(None, l.pop_front());
        assert_eq!(0, l.len());

        l.push_front(4);
        l.push_front(5);
        l.push_front(6);

        l.head_mut().map(|x| *x *= 2);

        assert_eq!(Some(&12), l.head());
        assert_eq!(Some(12), l.pop_front());
        assert_eq!(Some(5), l.pop_front());
        assert_eq!(Some(4), l.pop_front());
        assert_eq!(None, l.head());
        assert_eq!(None, l.pop_front());
    }

    #[test]
    fn back() {
        let mut l = LinkedList::new();

        assert_eq!(0, l.len());
        assert_eq!(None, l.pop_back());
        assert_eq!(None, l.tail());

        l.push_back(1);
        assert_eq!(1, l.len());
        assert_eq!(Some(&1), l.tail());
        assert_eq!(Some(1), l.pop_back());
        assert_eq!(None, l.tail());
        assert_eq!(None, l.pop_back());
        assert_eq!(0, l.len());

        l.push_back(2);
        l.push_back(3);

        assert_eq!(2, l.len());
        assert_eq!(Some(&3), l.tail());
        assert_eq!(Some(3), l.pop_back());
        assert_eq!(1, l.len());
        assert_eq!(Some(&2), l.tail());
        assert_eq!(Some(2), l.pop_back());
        assert_eq!(0, l.len());
        assert_eq!(None, l.tail());
        assert_eq!(None, l.pop_back());
        assert_eq!(0, l.len());

        l.push_back(4);
        l.push_back(5);
        l.push_back(6);

        l.tail_mut().map(|x| *x *= 2);

        assert_eq!(Some(&12), l.tail());
        assert_eq!(Some(12), l.pop_back());
        assert_eq!(Some(5), l.pop_back());
        assert_eq!(Some(4), l.pop_back());
        assert_eq!(None, l.tail());
        assert_eq!(None, l.pop_front());
    }

    #[test]
    fn drop() {
        let nodes = if cfg!(miri) { 5000 } else { 100000 };
        let mut l = LinkedList::new();

        for i in 0..nodes {
            l.push_front(i);
        }
    }
}
