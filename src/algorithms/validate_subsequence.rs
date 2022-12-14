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

pub fn validate_subsequence_2(arr: &[i32], sub: &[i32]) -> bool {
    let len_arr = arr.len();
    let len_sub = sub.len();

    let mut i_arr = 0;
    let mut i_sub = 0;

    while i_arr < len_arr && i_sub < len_sub {
        if arr[i_arr] == sub[i_sub] {
            i_sub += 1;
        }

        i_arr += 1;
    }

    return i_sub == len_sub;
}

pub fn validate_subsequence_3(arr: &[i32], sub: &[i32]) -> bool {
    match (arr.len(), sub.len()) {
        (_, 0) => true,
        (0, _) => false,
        (_, _) => match arr[0] == sub[0] {
            true => validate_subsequence_3(&arr[1..], &sub[1..]),
            false => validate_subsequence_3(&arr[1..], &sub),
        },
    }
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

    #[test]
    fn validate_subsequence_2_should_return_true_if_second_arr_is_sub_of_first() {
        let arr = vec![1, 5, 9, 7, 3, 4, 6, -9, 1, 0, 7, 4, -6, 1];
        let sub = vec![1, 6, -9, 0, 4, -6, 1];

        assert!(validate_subsequence_2(&arr, &sub));
    }

    #[test]
    fn validate_subsequence_2_should_return_false_if_second_arr_is_not_sub_of_first() {
        let arr = vec![1, 5, 9, 7, 3, 4, 6, -9, 1, 0, 7, 4, -6, 1];
        let sub = vec![1, 6, -9, 35, 0, 4, -6, 1];

        assert!(!validate_subsequence_2(&arr, &sub));
    }

    #[test]
    fn validate_subsequence_3_should_return_true_if_second_arr_is_sub_of_first() {
        let arr = vec![1, 5, 9, 7, 3, 4, 6, -9, 1, 0, 7, 4, -6, 1];
        let sub = vec![1, 6, -9, 0, 4, -6, 1];

        assert!(validate_subsequence_3(&arr, &sub));
    }

    #[test]
    fn validate_subsequence_3_should_return_false_if_second_arr_is_not_sub_of_first() {
        let arr = vec![1, 5, 9, 7, 3, 4, 6, -9, 1, 0, 7, 4, -6, 1];
        let sub = vec![1, 6, -9, 35, 0, 4, -6, 1];

        assert!(!validate_subsequence_3(&arr, &sub));
    }
}
