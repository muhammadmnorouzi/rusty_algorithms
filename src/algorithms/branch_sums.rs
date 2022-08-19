// use std::collections::binary_heap;

use std::{clone, ops::Add};

use crate::data_structures::{BinarySearchTree, Child};

pub fn branch_sum<T>(tree: BinarySearchTree<T>, acc: T) -> Vec<T>
where
    T: Ord + Eq + Add + Clone + From<<T as Add>::Output>,
{
    let mut sums: Vec<T> = Vec::new();
    calculate_branch_sums(&Some(Box::from(tree)), acc, &mut sums);
    return sums;
}

fn calculate_branch_sums<T>(node: &Child<T>, acc: T, sums: &mut Vec<T>)
where
    T: Ord + Eq + Add + Clone + From<<T as Add>::Output>,
{
    if let Some(ref node) = node {
        let acc = acc + node.get_value().unwrap();
        let acc = acc.into();

        if node.is_leaf() {
            sums.push(acc);
            return;
        }

        calculate_branch_sums(node.get_left(), acc.clone(), sums);
        calculate_branch_sums(node.get_right(), acc.clone(), sums);
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_branch_sums() {
        let mut tree = BinarySearchTree::new();

        tree.insert(10)
            .insert(15)
            .insert(7)
            .insert(9)
            .insert(5)
            .insert(14)
            .insert(18)
            .insert(20);

        let expected = vec![22, 26, 39, 63];
        let function_result = branch_sum(tree, 0);

        assert_eq!(expected, function_result);
    }
}
