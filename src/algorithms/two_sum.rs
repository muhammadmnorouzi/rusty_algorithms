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

#[cfg(test)]
mod tests {
    use super::two_sum_1;

    #[test]
    fn two_sum_test() {
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
}
