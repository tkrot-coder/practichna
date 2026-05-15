pub fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if v1 <= v2 {
        return "NO".to_string();
    }

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