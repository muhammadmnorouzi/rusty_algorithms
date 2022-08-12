use crate::data_structures::{BinarySearchTree, Node};

pub fn find_closest_value_in_bst_1(tree: &BinarySearchTree<i32>, data: i32) -> Option<i32>
where
{
    find_closest_value_in_bst_1_helper(&tree.root, data, std::i32::MAX)
}

fn find_closest_value_in_bst_1_helper(
    node: &Option<Box<Node<i32>>>,
    target: i32,
    closest: i32,
) -> Option<i32> {
    let mut closest = closest;

    match node {
        Some(ref node) => {
            if i32::abs(target - closest) > i32::abs(target - node.data) {
                closest = node.data;
            }

            if target < node.data {
                return find_closest_value_in_bst_1_helper(&node.left, target, closest);
            } else if target > node.data {
                return find_closest_value_in_bst_1_helper(&node.right, target, closest);
            }

            return Some(closest);
        }
        None => Some(closest),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bst_find_closest_value_1() {
        let mut bst: BinarySearchTree<i32> = BinarySearchTree::new();
        bst.insert(7)
            .insert(9)
            .insert(15)
            .insert(10)
            .insert(20)
            .insert(25)
            .insert(18);

        assert_eq!(find_closest_value_in_bst_1(&bst, 16), Some(15));
        assert_eq!(find_closest_value_in_bst_1(&bst, 14), Some(15));
        assert_eq!(find_closest_value_in_bst_1(&bst, 12), Some(10));
        assert_eq!(find_closest_value_in_bst_1(&bst, 17), Some(18));
    }
}
