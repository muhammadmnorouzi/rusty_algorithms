// use std::collections::binary_heap;

use std::ops::{Add, AddAssign};

use crate::data_structures::{BinarySearchTree, Child};

pub fn branch_sum<T>(tree: &BinarySearchTree<T>, acc: T) -> Vec<T>
where
    T: Ord + Eq + PartialEq + PartialOrd + Add + AddAssign,
{
    let mut sums: Vec<T> = Vec::new();
    calculate_branch_sums(&tree, acc, &mut sums);
    return sums;
}

fn calculate_branch_sums<T>(node: &Child<T>, acc: T, sums: &mut Vec<i32>)
where
    T: Ord + Eq + PartialEq + PartialOrd + Add + AddAssign,
{
    if let Some(ref node) = node {
        let temp_sum = temp_sum + node.;

        if node.is_leaf() {
            sums.push(temp_sum);
            return;
        }

        calculate_branch_sums(&node.left, temp_sum, sums);
        calculate_branch_sums(&node.right, temp_sum, sums);
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn branch_sums_test() {
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

        let function_result = branch_sums(&tree);

        assert_eq!(expected, function_result);
    }
}
