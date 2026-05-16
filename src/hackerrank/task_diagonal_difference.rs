pub fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let n = arr.len();
    let mut primary_diag = 0;
    let mut secondary_diag = 0;

    for i in 0..n {
        primary_diag += arr[i][i];
        secondary_diag += arr[i][n - 1 - i];
    }

    (primary_diag - secondary_diag).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagonal_difference_sample() {
        let arr = vec![
            vec![11, 2, 4],
            vec![4, 5, 6],
            vec![10, 8, -12],
        ];
        assert_eq!(diagonal_difference(&arr), 15);
    }

    #[test]
    fn test_diagonal_difference_square_2x2() {
        let arr = vec![
            vec![1, 2],
            vec![3, 4],
        ];
        assert_eq!(diagonal_difference(&arr), 0);
    }

    #[test]
    fn test_diagonal_difference_single_element() {
        let arr = vec![vec![5]];
        assert_eq!(diagonal_difference(&arr), 0);
    }
}