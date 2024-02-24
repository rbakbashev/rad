//! Unsafe singly-linked queue, also adapted from the book [Learning Rust With Entirely Too Many
//! Linked Lists](https://rust-unofficial.github.io/too-many-lists/index.html).

use std::ptr;

pub struct Queue<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
}

struct Node<T> {
    data: T,
    next: *mut Node<T>,
}

pub struct QueueIntoIter<T>(Queue<T>);

pub struct QueueIterator<'q, T> {
    next: Option<&'q Node<T>>,
}

pub struct QueueIterMut<'q, T> {
    next: Option<&'q mut Node<T>>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    pub fn push(&mut self, data: T) {
        let new_tail = Box::new(Node::new(data));
        let ptr_tail = Box::into_raw(new_tail);

        match unsafe { self.tail.as_mut() } {
            Some(t) => t.next = ptr_tail,
            None => self.head = ptr_tail,
        }

        self.tail = ptr_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_null() {
            return None;
        }

        let head = unsafe { Box::from_raw(self.head) };

        self.head = head.next;

        if self.head.is_null() {
            self.tail = ptr::null_mut();
        }

        Some(head.data)
    }

    pub fn peek(&self) -> Option<&T> {
        unsafe { self.tail.as_ref().map(|node| &node.data) }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe { self.tail.as_mut().map(|node| &mut node.data) }
    }

    pub fn iter(&self) -> QueueIterator<'_, T> {
        unsafe {
            QueueIterator {
                next: self.head.as_ref(),
            }
        }
    }

    pub fn iter_mut(&mut self) -> QueueIterMut<'_, T> {
        unsafe {
            QueueIterMut {
                next: self.head.as_mut(),
            }
        }
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> IntoIterator for Queue<T> {
    type Item = T;
    type IntoIter = QueueIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        QueueIntoIter(self)
    }
}

impl<'a, T> IntoIterator for &'a Queue<T> {
    type Item = &'a T;
    type IntoIter = QueueIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Queue<T> {
    type Item = &'a mut T;
    type IntoIter = QueueIterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T> Iterator for QueueIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for QueueIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = unsafe { node.next.as_ref() };
            &node.data
        })
    }
}

impl<'a, T> Iterator for QueueIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = unsafe { node.next.as_mut() };
            &mut node.data
        })
    }
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self {
            data,
            next: ptr::null_mut(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DROPPED_NODES: usize = if cfg!(miri) { 5000 } else { 100_000 };

    #[test]
    fn simple() {
        let mut q = Queue::new();

        assert_eq!(None, q.peek());
        assert_eq!(None, q.pop());

        q.push(1);
        q.push(2);

        assert_eq!(Some(&2), q.peek());

        q.push(3);

        assert_eq!(Some(&3), q.peek());
        assert_eq!(Some(1), q.pop());
        assert_eq!(Some(2), q.pop());

        q.push(4);
        q.push(5);

        assert_eq!(Some(&5), q.peek());
        assert_eq!(Some(3), q.pop());
        assert_eq!(Some(4), q.pop());
        assert_eq!(Some(5), q.pop());
        assert_eq!(None, q.pop());
        assert_eq!(None, q.peek());

        q.push(6);
        q.push(7);

        assert_eq!(Some(6), q.pop());
        assert_eq!(Some(&7), q.peek());
        assert_eq!(Some(7), q.pop());
        assert_eq!(None, q.pop());
        assert_eq!(None, q.peek());
    }

    #[test]
    fn drop() {
        let mut q = Queue::new();

        for i in 0..DROPPED_NODES {
            q.push(i);
        }
    }

    #[test]
    fn into_iters() {
        let mut q = Queue::new();

        q.push(1);
        q.push(2);
        q.push(3);

        let mut iter = (&q).into_iter();

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);

        let mut iter = (&mut q).into_iter();

        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);

        let mut iter = q.into_iter();

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iters() {
        let mut q = Queue::new();

        q.push(1);
        q.push(2);
        q.push(3);

        let mut iter = q.iter();

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);

        let mut iter = q.iter_mut();

        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);

        for item in q.iter_mut() {
            *item *= 2;
        }

        iter = q.iter_mut();

        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 4));
        assert_eq!(iter.next(), Some(&mut 6));
        assert_eq!(iter.next(), None);

        let mut q = Queue::new();

        q.push(1);
        q.push(2);
        q.push(3);

        q.peek_mut().map(|t| *t *= 2);

        assert_eq!(&[1, 2, 6], q.into_iter().collect::<Vec<i32>>().as_slice());
    }
}
