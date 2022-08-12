use std::collections::binary_heap;

use crate::data_structures::{BinarySearchTree, Node};

pub fn branch_sums(tree: &BinarySearchTree<i32>) -> Vec<i32> {
    let mut sums: Vec<i32> = Vec::new();
    calculate_branch_sums(&tree.root, 0, &mut sums);
    return sums;
}

fn calculate_branch_sums(node: &Option<Box<Node<i32>>>, temp_sum: i32, sums: &mut Vec<i32>) {
    if let Some(ref node) = node {
        let temp_sum = temp_sum + node.data;

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
