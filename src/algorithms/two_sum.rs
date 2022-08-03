#[allow(unused)]
pub fn two_sum_1(arr: &[i32], expected: i32) -> Vec<(i32, i32)> {
    let mut result: Vec<(i32, i32)> = vec![];

    let len = arr.len();

    for i in 0..len {
        let diff = expected - arr[i];
        for j in (i + 1)..len {
            let second = arr[j];

            if second == diff {
                result.push((expected - second, second))
            }
        }
    }

    result
}

#[allow(unused)]
pub fn two_sum_2(arr: &[i32], expected: i32) -> Vec<(i32, i32)> {
    let mut result: Vec<(i32, i32)> = vec![];
    let mut num_helper: Vec<i32> = vec![];

    for number in arr.iter() {
        let potential_match = expected - number;

        if num_helper.contains(&potential_match) {
            result.push((*number, potential_match));
        } else {
            num_helper.push(*number)
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_sum_1_test() {
        // Given
        let input_array: Vec<i32> = vec![1, 2, 4, 3, -3, 13, 7, 10, 0, 1, -5, 15];
        let inupt_expected = 10;
        let expected_output = vec![(3, 7), (-3, 13), (10, 0), (-5, 15)];

        // When
        let output = two_sum_1(&input_array, inupt_expected);

        // Then
        for item in output.clone() {
            assert_eq!(inupt_expected, item.0 + item.1);
        }

        assert_eq!(expected_output.len(), output.len());

        for i in 0..output.len() {
            assert_eq!(expected_output[i], output[i]);
        }
    }

    #[test]
    fn two_sum_2_test() {
        // Given
        let input_array: Vec<i32> = vec![1, 2, 4, 3, -3, 13, 7, 10, 0, 1, -5, 15];
        let inupt_expected = 10;
        let expected_output = vec![(13, -3), (7, 3), (0, 10), (15, -5)];

        // When
        let output = two_sum_2(&input_array, inupt_expected);

        // Then
        for item in output.clone() {
            assert_eq!(inupt_expected, item.0 + item.1);
        }

        assert_eq!(expected_output.len(), output.len());

        for i in 0..output.len() {
            assert_eq!(expected_output[i], output[i]);
        }
    }
}
