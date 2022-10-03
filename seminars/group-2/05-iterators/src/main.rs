use std::collections::VecDeque;

enum Node<T> {
    Leaf(T),
    Children(Vec<Node<T>>),
}

impl<T> Node<T> {
    fn iter<'a>(&'a self) -> NodeIter<'a, T> {
        NodeIter {
            parent: None,
            children: VecDeque::from([self]),
        }
    }
}

impl<'a, T> IntoIterator for &'a Node<T> {
    type Item = &'a T;

    type IntoIter = NodeIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

struct NodeIter<'a, T> {
    parent: Option<Box<NodeIter<'a, T>>>,
    children: VecDeque<&'a Node<T>>,
}

impl<'a, T> Default for NodeIter<'a, T> {
    fn default() -> Self {
        Self { parent: Default::default(), children: Default::default() }
    }
}

impl<'a, T> Iterator for NodeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.children.pop_front() {
            None => {
                match self.parent.take() {
                    None => None,
                    Some(parent) => {
                        *self = *parent;
                        self.next()
                    }
                }
            }
            Some(Node::Leaf(value)) => Some(value),
            Some(Node::Children(children)) => {
                *self = NodeIter {
                    parent: Some(Box::new(std::mem::take(self))),
                    children: VecDeque::from_iter(children.iter()),
                };
                self.next()
            }
        }
    }
}

fn main() {
    let root = Node::Children(vec![
        Node::Children(vec![
            Node::Leaf(1),
            Node::Leaf(2),
            Node::Leaf(3),
        ]),
        Node::Leaf(4),
        Node::Children(vec![
            Node::Children(vec![
                Node::Leaf(5),
                Node::Leaf(6),
            ]),
            Node::Leaf(7),
        ]),
    ]);

    for elem in &root {
        println!("{}", elem);
    }
}
