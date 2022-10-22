pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    pub fn push(&mut self, elem: T) {
        let new = Box::new(Node {
            elem,
            next: self.head.take(),
        });

        self.head = Some(new);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut it = self.head.take();

        while let Some(mut node) = it {
            it = node.next.take();
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

impl<T: std::fmt::Display> std::fmt::Display for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut it = &self.head;

        while let Some(node) = it {
            it = &node.next;
            write!(f, "{} → ", node.elem)?;
        }

        write!(f, "nil")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut list = List::new();

        list.push(3);
        list.push(2);
        list.push(1);

        println!("{}", list);

        assert!(list.pop() == Some(1));
        assert!(list.pop() == Some(2));
        assert!(list.pop() == Some(3));
    }

    #[test]
    fn mutate() {
        let mut list = List::new();

        list.push(3);
        list.push(2);
        list.push(1);

        list.peek_mut().map(|value| *value = 100);

        println!("{}", list);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();

        list.push(3);
        list.push(2);
        list.push(1);

        for item in list.iter_mut() {
            *item *= 2;
        }

        println!("{}", list);
    }
}
