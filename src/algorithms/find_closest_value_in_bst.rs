use std::ops::{Deref, Sub};

use crate::data_structures::{BinarySearchTree, Child};

pub fn find_closest_value_in_bst_1<'a, T, F, UT>(
    tree: BinarySearchTree<T>,
    data: T,
    max: T,
    abs_diff_calculator: F,
) -> Option<T>
where
    T: Ord + Clone + Copy,
    F: Fn(T, T) -> UT,
    UT: Ord,
{
    let node = &Some(Box::from(tree));
    find_closest_value_in_bst_1_helper(node, data, max, abs_diff_calculator)
}

fn find_closest_value_in_bst_1_helper<T, F, UT>(
    node: &Option<Box<BinarySearchTree<T>>>,
    target: T,
    closest: T,
    abs_diff_calculator: F,
) -> Option<T>
where
    T: Ord + Clone + Copy,
    F: Fn(T, T) -> UT,
    UT: Ord,
{
    let mut closest = closest;

    match node {
        Some(ref node) => {
            if let Some(value) = &node.get_value() {
                let value = value.clone();

                if abs_diff_calculator(target, closest) > abs_diff_calculator(target, value) {
                    closest = value;
                }

                if target < value {
                    return find_closest_value_in_bst_1_helper(
                        node.get_left(),
                        target,
                        closest,
                        abs_diff_calculator,
                    );
                } else if target > value {
                    return find_closest_value_in_bst_1_helper(
                        node.get_right(),
                        target,
                        closest,
                        abs_diff_calculator,
                    );
                }
            }
            return Some(closest);
        }
        None => Some(closest),
    }
}

pub fn find_closest_value_in_bst_2<T, F, UT>(
    tree: BinarySearchTree<T>,
    data: T,
    max: T,
    abs_diff_calculator: F,
) -> Option<T>
where
    T: Ord + Clone + Copy,
    F: Fn(T, T) -> UT,
    UT: Ord,
{
    find_closest_value_in_bst_2_helper(&Some(Box::from(tree)), data, max, abs_diff_calculator)
}

fn find_closest_value_in_bst_2_helper<T, F, UT>(
    node: &Child<T>,
    target: T,
    closest: T,
    abs_diff_calculator: F,
) -> Option<T>
where
    T: Ord + Clone + Copy,
    F: Fn(T, T) -> UT,
    UT: Ord,
{
    let mut current_node = node;
    let mut closest = closest;

    loop {
        if let Some(node) = current_node {
            if let Some(value) = node.get_value() {
                if abs_diff_calculator(target, closest) > abs_diff_calculator(target, value) {
                    closest = value;
                }

                if target < value {
                    current_node = node.get_left();
                } else if target > value {
                    current_node = node.get_right();
                } else {
                    return Some(closest);
                }
            }
        } else {
            return Some(closest);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_full_bst() -> BinarySearchTree<i32> {
        let mut bst: BinarySearchTree<i32> = BinarySearchTree::new();

        bst.insert(7)
            .insert(9)
            .insert(15)
            .insert(10)
            .insert(20)
            .insert(25)
            .insert(18);

        bst
    }

    #[test]
    fn bst_find_closest_value_1() {
        assert_eq!(
            find_closest_value_in_bst_1(get_full_bst(), 16, i32::MAX, i32::abs_diff),
            Some(15)
        );

        assert_eq!(
            find_closest_value_in_bst_1(get_full_bst(), 14, i32::MAX, i32::abs_diff),
            Some(15)
        );

        assert_eq!(
            find_closest_value_in_bst_1(get_full_bst(), 12, i32::MAX, i32::abs_diff),
            Some(10)
        );

        assert_eq!(
            find_closest_value_in_bst_1(get_full_bst(), 17, i32::MAX, i32::abs_diff),
            Some(18)
        );
    }

    #[test]
    fn bst_find_closest_value_2() {
        assert_eq!(
            find_closest_value_in_bst_2(get_full_bst(), 16, i32::MAX, i32::abs_diff),
            Some(15)
        );

        assert_eq!(
            find_closest_value_in_bst_2(get_full_bst(), 14, i32::MAX, i32::abs_diff),
            Some(15)
        );

        assert_eq!(
            find_closest_value_in_bst_2(get_full_bst(), 12, i32::MAX, i32::abs_diff),
            Some(10)
        );

        assert_eq!(
            find_closest_value_in_bst_2(get_full_bst(), 17, i32::MAX, i32::abs_diff),
            Some(18)
        );
    }
}
