pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new = Box::new(Node {
            elem,
            next: self.head.take()
        });

        self.head = Some(new);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
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
}
