mod hackerrank;
fn main() {
    let s: i32 = 7;
    let t: i32 = 11;
    let a: i32 = 5;
    let b: i32 = 15;
    let apples = vec![-2, 2, 1];
    let oranges = vec![5, -6];

    println!("--- Результати задачі Apple and Orange ---");
    
    // 2. Викликаємо функцію (ЗВЕРНІТЬ УВАГУ: після назви функції solution НЕМАЄ порожніх дужок)
    hackerrank::apple_and_orange::solution(
        s, 
        t, 
        a, 
        b, 
        &apples, 
        &oranges
    );

    println!("------------------------------------------");
    println!("Усі тести та розрахунки виконано успішно!");
}