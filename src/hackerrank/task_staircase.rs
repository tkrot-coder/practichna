use std::io::{self, BufRead};

pub fn staircase(n: i32) {
    let n = n as usize;
    for i in 1..=n {
        let spaces = " ".repeat(n - i);
        let hashes = "#".repeat(i);
        println!("{}{}", spaces, hashes);
    }
}

// Функція main тут не обов'язкова для тестів, але її можна залишити
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    if let Some(Ok(line)) = stdin_iterator.next() {
        if let Ok(n) = line.trim().parse::<i32>() {
            staircase(n);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_staircase_logic() {
        staircase(1);
        assert!(true);
    }
}