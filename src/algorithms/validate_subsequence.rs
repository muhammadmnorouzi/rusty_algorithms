#![allow(unused)]

pub fn validate_subsequence_1(arr: &[i32], sub: &[i32]) -> bool {
    let sub_len = sub.len();
    let mut sub_index = 0;

    for &item in arr {
        if item == sub[sub_index] {
            sub_index += 1;

            if sub_index >= sub_len {
                return true;
            }
        }
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_subsequence_1_should_return_true_if_second_arr_is_sub_of_first() {
        let arr = vec![1, 5, 9, 7, 3, 4, 6, -9, 1, 0, 7, 4, -6, 1];
        let sub = vec![1, 6, -9, 0, 4, -6, 1];

        assert!(validate_subsequence_1(&arr, &sub));
    }

    #[test]
    fn validate_subsequence_1_should_return_false_if_second_arr_is_not_sub_of_first() {
        let arr = vec![1, 5, 9, 7, 3, 4, 6, -9, 1, 0, 7, 4, -6, 1];
        let sub = vec![1, 6, -9, 35, 0, 4, -6, 1];

        assert!(!validate_subsequence_1(&arr, &sub));
    }
}
