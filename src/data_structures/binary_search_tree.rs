// used from https://github.com/TheAlgorithms/Rust/blob/master/src/data_structures/binary_search_tree.rs

use std::clone;
use std::cmp::Ordering;
use std::collections::binary_heap;
use std::ops::Deref;

/// This struct implements as Binary Search Tree (BST), which is a
/// simple data structure for storing sorted data
pub struct BinarySearchTree<T>
where
    T: Ord,
{
    value: Option<T>,
    left: Child<T>,
    right: Child<T>,
}

pub type Child<T> = Option<Box<BinarySearchTree<T>>>;

impl<T> Default for BinarySearchTree<T>
where
    T: Ord + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord + Clone,
{
    /// Create a new, empty BST
    pub fn new() -> BinarySearchTree<T> {
        BinarySearchTree {
            value: None,
            left: None,
            right: None,
        }
    }

    pub fn get_value(&self) -> Option<T> {
        self.value.clone()
    }

    pub fn get_left(&self) -> &Child<T> {
        &self.left
    }

    pub fn get_right(&self) -> &Child<T> {
        &self.right
    }

    pub fn is_leaf(&self) -> bool {
        if let (None, None) = (&self.left, &self.right) {
            return true;
        }

        false
    }

    /// Find a value in this tree. Returns True iff value is in this
    /// tree, and false otherwise
    pub fn search(&self, value: &T) -> bool {
        match &self.value {
            Some(key) => {
                match key.cmp(value) {
                    Ordering::Equal => {
                        // key == value
                        true
                    }
                    Ordering::Greater => {
                        // key > value
                        match &self.left {
                            Some(node) => node.search(value),
                            None => false,
                        }
                    }
                    Ordering::Less => {
                        // key < value
                        match &self.right {
                            Some(node) => node.search(value),
                            None => false,
                        }
                    }
                }
            }
            None => false,
        }
    }

    /// Returns a new iterator which iterates over this tree in order
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        BinarySearchTreeIter::new(self)
    }

    /// Insert a value into the appropriate location in this tree.
    pub fn insert(&mut self, value: T) -> &mut Self {
        if self.value.is_none() {
            self.value = Some(value);
        } else {
            match &self.value {
                None => (),
                Some(key) => {
                    let target_node = if value < *key {
                        &mut self.left
                    } else {
                        &mut self.right
                    };
                    match target_node {
                        Some(ref mut node) => {
                            node.insert(value);
                        }
                        None => {
                            let mut node = BinarySearchTree::new();
                            node.insert(value);
                            *target_node = Some(Box::new(node));
                        }
                    }
                }
            }
        }

        self
    }

    /// Returns the smallest value in this tree
    pub fn minimum(&self) -> Option<&T> {
        match &self.left {
            Some(node) => node.minimum(),
            None => self.value.as_ref(),
        }
    }

    /// Returns the largest value in this tree
    pub fn maximum(&self) -> Option<&T> {
        match &self.right {
            Some(node) => node.maximum(),
            None => self.value.as_ref(),
        }
    }

    /// Returns the largest value in this tree smaller than value
    pub fn floor(&self, value: &T) -> Option<&T> {
        match &self.value {
            Some(key) => {
                match key.cmp(value) {
                    Ordering::Greater => {
                        // key > value
                        match &self.left {
                            Some(node) => node.floor(value),
                            None => None,
                        }
                    }
                    Ordering::Less => {
                        // key < value
                        match &self.right {
                            Some(node) => {
                                let val = node.floor(value);
                                match val {
                                    Some(_) => val,
                                    None => Some(key),
                                }
                            }
                            None => Some(key),
                        }
                    }
                    Ordering::Equal => Some(key),
                }
            }
            None => None,
        }
    }

    /// Returns the smallest value in this tree larger than value
    pub fn ceil(&self, value: &T) -> Option<&T> {
        match &self.value {
            Some(key) => {
                match key.cmp(value) {
                    Ordering::Less => {
                        // key < value
                        match &self.right {
                            Some(node) => node.ceil(value),
                            None => None,
                        }
                    }
                    Ordering::Greater => {
                        // key > value
                        match &self.left {
                            Some(node) => {
                                let val = node.ceil(value);
                                match val {
                                    Some(_) => val,
                                    None => Some(key),
                                }
                            }
                            None => Some(key),
                        }
                    }
                    Ordering::Equal => {
                        // key == value
                        Some(key)
                    }
                }
            }
            None => None,
        }
    }
}

struct BinarySearchTreeIter<'a, T>
where
    T: Ord,
{
    stack: Vec<&'a BinarySearchTree<T>>,
}

impl<'a, T> BinarySearchTreeIter<'a, T>
where
    T: Ord,
{
    pub fn new(tree: &BinarySearchTree<T>) -> BinarySearchTreeIter<T> {
        let mut iter = BinarySearchTreeIter { stack: vec![tree] };
        iter.stack_push_left();
        iter
    }

    fn stack_push_left(&mut self) {
        while let Some(child) = &self.stack.last().unwrap().left {
            self.stack.push(child);
        }
    }
}

impl<'a, T> Iterator for BinarySearchTreeIter<'a, T>
where
    T: Ord,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.stack.is_empty() {
            None
        } else {
            let node = self.stack.pop().unwrap();
            if node.right.is_some() {
                self.stack.push(node.right.as_ref().unwrap().deref());
                self.stack_push_left();
            }
            node.value.as_ref()
        }
    }
}

#[cfg(test)]
mod test {
    use super::BinarySearchTree;

    fn prequel_memes_tree() -> BinarySearchTree<&'static str> {
        let mut tree = BinarySearchTree::new();

        tree.insert("hello there")
            .insert("general kenobi")
            .insert("you are a bold one")
            .insert("kill him")
            .insert("back away...I will deal with this jedi slime myself")
            .insert("your move")
            .insert("you fool");

        tree
    }

    #[test]
    fn test_get_value() {
        let mut tree: BinarySearchTree<i32> = BinarySearchTree::new();
        assert_eq!(tree.get_value(), None);

        tree.insert(5);
        assert_eq!(tree.get_value(), Some(5));
    }

    #[test]
    fn test_is_leaf() {
        let mut tree: BinarySearchTree<i32> = BinarySearchTree::new();
        assert!(tree.is_leaf());

        tree.insert(5).insert(7);

        assert_eq!(tree.is_leaf(), false);
    }

    #[test]
    fn test_get_left() {
        let mut tree: BinarySearchTree<i32> = BinarySearchTree::new();
        match tree.get_left() {
            Some(_) => assert!(false),
            None => assert!(true),
        }

        tree.insert(5).insert(3);

        match tree.get_left() {
            Some(tree) => assert_eq!(tree.get_value(), Some(3)),
            None => assert!(false),
        }
    }

    #[test]
    fn test_get_right() {
        let mut tree: BinarySearchTree<i32> = BinarySearchTree::new();
        match tree.get_right() {
            Some(_) => assert!(false),
            None => assert!(true),
        }

        tree.insert(5).insert(3).insert(7);

        match tree.get_right() {
            Some(tree) => assert_eq!(tree.get_value(), Some(7)),
            None => assert!(false),
        }
    }

    #[test]
    fn test_search() {
        let tree = prequel_memes_tree();
        assert!(tree.search(&"hello there"));
        assert!(tree.search(&"you are a bold one"));
        assert!(tree.search(&"general kenobi"));
        assert!(tree.search(&"you fool"));
        assert!(tree.search(&"kill him"));
        assert!(
            !tree.search(&"but i was going to tosche station to pick up some power converters",)
        );
        assert!(!tree.search(&"only a sith deals in absolutes"));
        assert!(!tree.search(&"you underestimate my power"));
    }

    #[test]
    fn test_maximum_and_minimum() {
        let tree = prequel_memes_tree();
        assert_eq!(*tree.maximum().unwrap(), "your move");
        assert_eq!(
            *tree.minimum().unwrap(),
            "back away...I will deal with this jedi slime myself"
        );
        let mut tree2: BinarySearchTree<i32> = BinarySearchTree::new();
        assert!(tree2.maximum().is_none());
        assert!(tree2.minimum().is_none());
        tree2.insert(0);
        assert_eq!(*tree2.minimum().unwrap(), 0);
        assert_eq!(*tree2.maximum().unwrap(), 0);
        tree2.insert(-5);
        assert_eq!(*tree2.minimum().unwrap(), -5);
        assert_eq!(*tree2.maximum().unwrap(), 0);
        tree2.insert(5);
        assert_eq!(*tree2.minimum().unwrap(), -5);
        assert_eq!(*tree2.maximum().unwrap(), 5);
    }

    #[test]
    fn test_floor_and_ceil() {
        let tree = prequel_memes_tree();
        assert_eq!(*tree.floor(&"hello there").unwrap(), "hello there");
        assert_eq!(
            *tree
                .floor(&"these are not the droids you're looking for")
                .unwrap(),
            "kill him"
        );
        assert!(tree.floor(&"another death star").is_none());
        assert_eq!(*tree.floor(&"you fool").unwrap(), "you fool");
        assert_eq!(
            *tree.floor(&"but i was going to tasche station").unwrap(),
            "back away...I will deal with this jedi slime myself"
        );
        assert_eq!(
            *tree.floor(&"you underestimate my power").unwrap(),
            "you fool"
        );
        assert_eq!(*tree.floor(&"your new empire").unwrap(), "your move");
        assert_eq!(*tree.ceil(&"hello there").unwrap(), "hello there");
        assert_eq!(
            *tree
                .ceil(&"these are not the droids you're looking for")
                .unwrap(),
            "you are a bold one"
        );
        assert_eq!(
            *tree.ceil(&"another death star").unwrap(),
            "back away...I will deal with this jedi slime myself"
        );
        assert_eq!(*tree.ceil(&"you fool").unwrap(), "you fool");
        assert_eq!(
            *tree.ceil(&"but i was going to tasche station").unwrap(),
            "general kenobi"
        );
        assert_eq!(
            *tree.ceil(&"you underestimate my power").unwrap(),
            "your move"
        );
        assert!(tree.ceil(&"your new empire").is_none());
    }

    #[test]
    fn test_iterator() {
        let tree = prequel_memes_tree();
        let mut iter = tree.iter();
        assert_eq!(
            iter.next().unwrap(),
            &"back away...I will deal with this jedi slime myself"
        );
        assert_eq!(iter.next().unwrap(), &"general kenobi");
        assert_eq!(iter.next().unwrap(), &"hello there");
        assert_eq!(iter.next().unwrap(), &"kill him");
        assert_eq!(iter.next().unwrap(), &"you are a bold one");
        assert_eq!(iter.next().unwrap(), &"you fool");
        assert_eq!(iter.next().unwrap(), &"your move");
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}
