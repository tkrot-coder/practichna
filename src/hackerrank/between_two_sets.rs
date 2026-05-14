pub fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let mut count = 0;
    let start = *a.iter().max().unwrap_or(&0);
    let end = *b.iter().min().unwrap_or(&100);

    for i in start..=end {
        let ok_a = a.iter().all(|&x| i % x == 0);
        let ok_b = b.iter().all(|&x| x % i == 0);

        if ok_a && ok_b {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_between_two_sets_example() {
        let a = vec![2, 4];
        let b = vec![16, 32, 96];
        assert_eq!(get_total_x(&a, &b), 3);
    }

    #[test]
    fn test_simple_case() {
        let a = vec![3, 4];
        let b = vec![24, 48];
        assert_eq!(get_total_x(&a, &b), 2);
    }
}