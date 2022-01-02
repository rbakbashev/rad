pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new = Box::new(Node {
            elem,
            next: std::mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            },
            Link::Empty => None
        }
    }
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut it = &self.head;

        loop {
            match it {
                Link::More(node) => {
                    it = &node.next;
                    write!(f, "{} → ", node.elem)?;
                }
                Link::Empty => {
                    write!(f, "nil")?;
                    break;
                }
            }
        }

        Ok(())
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
