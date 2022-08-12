use std::{fmt::Display, mem::swap};

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct Node<T> {
    pub data: T,
    pub left: Child<T>,
    pub right: Child<T>,
}

pub type Child<T> = Option<Box<Node<T>>>;

impl<T: PartialOrd + Clone + Display> Node<T> {
    pub fn new(data: T) -> Node<T> {
        Self {
            data: data,
            left: None,
            right: None,
        }
    }

    pub fn is_leaf(&self) -> bool {
        if let (None, None) = (&self.left, &self.right) {
            return true;
        }

        return false;
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
