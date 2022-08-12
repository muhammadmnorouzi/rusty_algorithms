// Thanks to open source existing impls
#![allow(unused)]
use std::{collections::binary_heap, fmt::Display, mem::swap};

use super::{Child, Node};

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct BinarySearchTree<T: PartialOrd + Clone> {
    pub root: Child<T>,
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
