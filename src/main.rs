mod hackerrank;

fn main() {
    let s = 7;
    let t = 11;
    let a_pos = 5;
    let b_pos = 15;
    let apples = vec![-2, 2, 1];
    let oranges = vec![5, -6];

    println!("--- Apple and Orange ---");
    hackerrank::apple_and_orange::solution(s, t, a_pos, b_pos, &apples, &oranges);

    println!("\n--- Kangaroo ---");
    println!("{}", hackerrank::kangaroo::kangaroo(0, 3, 4, 2));

    println!("\n--- Between Two Sets ---");
    let a_set = vec![2, 4];
    let b_set = vec![16, 32, 96];
    println!("{}", hackerrank::between_two_sets::get_total_x(&a_set, &b_set));
}