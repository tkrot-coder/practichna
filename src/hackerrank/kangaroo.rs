pub fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    // Якщо перший кенгуру стартує там само або позаду, 
    // але стрибає повільніше або так само як другий, він ніколи його не наздожене.
    if v1 <= v2 {
        return "NO".to_string();
    }

    // Кенгуру зустрінуться, якщо різниця відстаней ділиться на різницю швидкостей
    // (x1 + n*v1 = x2 + n*v2) => n = (x2 - x1) / (v1 - v2)
    if (x2 - x1) % (v1 - v2) == 0 {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kangaroo_meets() {
    
        assert_eq!(kangaroo(0, 3, 4, 2), "YES");
    }

    #[test]
    fn test_kangaroo_never_meets() {

        assert_eq!(kangaroo(0, 2, 5, 3), "NO");
    }

    #[test]
    fn test_kangaroo_same_speed_different_start() {
        assert_eq!(kangaroo(0, 2, 4, 2), "NO");
    }
}