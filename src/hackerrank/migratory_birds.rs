pub fn migratory_birds(arr: &[i32]) -> i32 {
    let mut counts = [0; 6];
    for &bird_type in arr {
        counts[bird_type as usize] += 1;
    }

    let mut max_count = 0;
    let mut result_type = 0;

    for i in 1..=5 {
        if counts[i] > max_count {
            max_count = counts[i];
            result_type = i as i32;
        }
    }

    result_type
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migratory_birds_sample() {
        let arr = vec![1, 4, 4, 4, 5, 3];
        assert_eq!(migratory_birds(&arr), 4);
    }

    #[test]
    fn test_migratory_birds_tie() {
        let arr = vec![1, 2, 2, 1, 3];
        assert_eq!(migratory_birds(&arr), 1);
    }

    #[test]
    fn test_all_same() {
        let arr = vec![5, 5, 5];
        assert_eq!(migratory_birds(&arr), 5);
    }
}