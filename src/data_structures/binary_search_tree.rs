// Thanks to open source existing impls
#![allow(unused)]
use std::{collections::binary_heap, fmt::Display, mem::swap};

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct Node<T> {
    data: T,
    left: Child<T>,
    right: Child<T>,
}

type Child<T> = Option<Box<Node<T>>>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct BinarySearchTree<T: PartialOrd + Clone> {
    root: Child<T>,
}

impl<T: PartialOrd + Clone + Display> Node<T> {
    pub fn new(data: T) -> Node<T> {
        Self {
            data: data,
            left: None,
            right: None,
        }
    }

    pub fn data(&self) -> T {
        self.data.clone()
    }

    pub fn min(&self) -> T {
        match &self.left {
            Some(left) => Node::min(&left),
            None => self.data(),
        }
    }

    pub fn max(&self) -> T {
        match &self.right {
            Some(right) => Node::max(&right),
            None => self.data(),
        }
    }

    pub fn find(&self, data: &T) -> Option<Box<&Node<T>>> {
        if data > &self.data {
            match self.right {
                Some(ref right) => right.find(data),
                None => None,
            }
        } else if data < &self.data {
            match self.left {
                Some(ref left) => left.find(data),
                None => None,
            }
        } else {
            Some(Box::from(self))
        }
    }

    pub fn insert(&mut self, data: T) {
        if data > self.data {
            match self.right {
                Some(ref mut right) => right.insert(data),
                None => swap(&mut self.right, &mut Some(Box::from(Node::new(data)))),
            }
        } else if data < self.data {
            match self.left {
                Some(ref mut left) => left.insert(data),
                None => swap(&mut self.left, &mut Some(Box::from(Node::new(data)))),
            }
        }
    }

    pub fn traverse_in_order<F>(&self, visit: &F)
    where
        F: Fn(T),
    {
        if let Some(ref left) = self.left {
            left.traverse_in_order(visit);
        }

        visit(self.data());

        if let Some(ref right) = self.right {
            right.traverse_in_order(visit);
        }
    }

    pub fn traverse_pre_order<F>(&self, visit: &F)
    where
        F: Fn(T),
    {
        visit(self.data());

        if let Some(ref left) = self.left {
            left.traverse_in_order(visit);
        }

        if let Some(ref right) = self.right {
            right.traverse_in_order(visit);
        }
    }

    pub fn traverse_post_order<F>(&self, visit: &F)
    where
        F: Fn(T),
    {
        if let Some(ref left) = self.left {
            left.traverse_in_order(visit);
        }

        if let Some(ref right) = self.right {
            right.traverse_in_order(visit);
        }

        visit(self.data());
    }
}

impl<T: PartialOrd + Clone + Display> BinarySearchTree<T> {
    pub fn new() -> BinarySearchTree<T> {
        BinarySearchTree { root: None }
    }

    pub fn insert(&mut self, data: T) -> Box<&mut BinarySearchTree<T>> {
        match self.root {
            Some(ref mut root) => root.insert(data),
            None => swap(&mut self.root, &mut Some(Box::from(Node::new(data)))),
        };

        Box::new(self)
    }

    pub fn min(&self) -> Option<T> {
        match self.root {
            Some(ref root) => Some(root.min()),
            None => None,
        }
    }

    pub fn max(&self) -> Option<T> {
        match self.root {
            Some(ref root) => Some(root.max()),
            None => None,
        }
    }

    pub fn find(&self, data: &T) -> Option<Box<&Node<T>>> {
        match self.root {
            Some(ref root) => root.find(data),
            None => None,
        }
    }

    pub fn exists(&self, data: &T) -> bool {
        match self.find(data) {
            _ => true,
            None => false,
        }
    }

    pub fn traverse_in_order<F>(&self, visit: &F)
    where
        F: Fn(T),
    {
        match self.root {
            Some(ref root) => root.traverse_in_order(visit),
            None => println!("Tree is empty.Nothing to traverse!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_new_should_have_data_with_none_branches() {
        let data = 5;
        let node = Node::new(data);

        assert_eq!(node.data(), node.data);
        assert_eq!(node.data(), data);
        assert_eq!(node.left, None);
        assert_eq!(node.right, None);
    }

    #[test]
    fn node_min_should_return_minimum_value() {
        let mut node: Node<i32> = Node::new(10);
        node.insert(7);
        node.insert(15);
        node.insert(3);
        node.insert(20);
        node.insert(10);

        assert_eq!(node.min(), 3);
    }

    #[test]
    fn node_max_should_return_maximum_value() {
        let mut node: Node<i32> = Node::new(10);
        node.insert(7);
        node.insert(15);
        node.insert(3);
        node.insert(20);
        node.insert(10);

        assert_eq!(node.max(), 20);
    }

    #[test]
    fn node_insert_should_put_data_in_proper_place() {
        let mut node: Node<i32> = Node::new(10);
        node.insert(7);
        node.insert(15);
        node.insert(3);
        node.insert(20);
        node.insert(10);

        assert_eq!(
            node,
            Node {
                data: 10,
                left: Some(Box::from(Node {
                    data: 7,
                    left: Some(Box::from(Node {
                        data: 3,
                        left: None,
                        right: None
                    })),
                    right: None,
                })),
                right: Some(Box::from(Node {
                    data: 15,
                    left: None,
                    right: Some(Box::from(Node {
                        data: 20,
                        left: None,
                        right: None
                    }))
                }))
            }
        )
    }
}
