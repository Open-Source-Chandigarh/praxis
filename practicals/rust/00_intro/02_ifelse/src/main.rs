// 🦀 Rust Exercise: If-Else Statements
//
// Complete the functions below using `if` and `else`.
// Run `px` to check your answers.

fn is_positive(n: i32) -> bool {
    // TODO: return true if n is positive, false otherwise
    unimplemented!()
}

fn max_of_two(a: i32, b: i32) -> i32 {
    // TODO: return the bigger of a and b using if-else
    unimplemented!()
}

fn grade(score: u32) -> &'static str {
    // TODO: return a grade based on score:
    // "A" if score >= 90
    // "B" if score >= 75
    // "C" if score >= 50
    // "F" otherwise
    unimplemented!()
}

// ----------------- Tests -----------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_positive() {
        assert!(is_positive(10));
        assert!(!is_positive(0));
        assert!(!is_positive(-5));
    }

    #[test]
    fn test_max_of_two() {
        assert_eq!(max_of_two(5, 10), 10);
        assert_eq!(max_of_two(20, 3), 20);
        assert_eq!(max_of_two(7, 7), 7);
    }

    #[test]
    fn test_grade() {
        assert_eq!(grade(95), "A");
        assert_eq!(grade(80), "B");
        assert_eq!(grade(60), "C");
        assert_eq!(grade(40), "F");
    }
}

