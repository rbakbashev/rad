pub struct List {
    head: Link,
}

type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: i32) {
        let new = Box::new(Node {
            elem,
            next: self.head.take()
        });

        self.head = Some(new);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut it = self.head.take();

        while let Some(mut node) = it {
            it = node.next.take();
        }
    }
}

impl std::fmt::Display for List {
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
