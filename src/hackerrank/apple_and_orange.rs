pub fn count_apples_and_oranges(
    s: i32,
    t: i32,
    a: i32,
    b: i32,
    apples: &[i32],
    oranges: &[i32],
) -> (i32, i32) {
    let apple_count = apples
        .iter()
        .map(|&d| a + d)
        .filter(|&pos| pos >= s && pos <= t)
        .count() as i32;

    let orange_count = oranges
        .iter()
        .map(|&d| b + d)
        .filter(|&pos| pos >= s && pos <= t)
        .count() as i32;

    (apple_count, orange_count)
}

pub fn solution(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let (apples_on_house, oranges_on_house) = count_apples_and_oranges(s, t, a, b, apples, oranges);
    println!("{}", apples_on_house);
    println!("{}", oranges_on_house);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_apples_and_oranges() {
        let s = 7;
        let t = 11;
        let a = 5;
        let b = 15;
        let apples = vec![-2, 2, 1];
        let oranges = vec![5, -6];

        let result = count_apples_and_oranges(s, t, a, b, &apples, &oranges);
        
        assert_eq!(result, (1, 1));
    }

    #[test]
    fn test_no_fruit_on_house() {
        let s = 10;
        let t = 20;
        let a = 5;
        let b = 25;
        let apples = vec![1, 2]; // 6, 7 (поза межами 10-20)
        let oranges = vec![-1, -2]; // 24, 23 (поза межами 10-20)

        let result = count_apples_and_oranges(s, t, a, b, &apples, &oranges);
        assert_eq!(result, (0, 0));
    }
}