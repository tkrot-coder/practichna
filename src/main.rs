mod hackerrank;

fn main() {
    // --- Тестування задачі 1: Apple and Orange ---
    println!("--- Apple and Orange ---");
    let s = 7;
    let t = 11;
    let a = 5;
    let b = 15;
    let apples = vec![-2, 2, 1];
    let oranges = vec![5, -6];

    hackerrank::apple_and_orange::solution(s, t, a, b, &apples, &oranges);

    // --- Тестування задачі 2: Kangaroo (Number Line Jumps) ---
    println!("\n--- Kangaroo ---");
    let result = hackerrank::kangaroo::kangaroo(0, 3, 4, 2);
    println!("Чи зустрінуться кенгуру: {}", result);

    println!("\n------------------------------------------");
    println!("Усі модулі завантажено, програма завершена!");
}