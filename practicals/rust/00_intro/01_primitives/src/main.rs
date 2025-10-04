// 🦀 Rust Exercise: Primitives
//
// Work with integer, float, boolean, and char types.
// Complete the functions below and run `px` to check your work.

fn get_integer() -> i32 {
    // TODO: return any integer value
    unimplemented!()
}

fn get_float() -> f64 {
    // TODO: return any floating point value
    unimplemented!()
}

fn get_boolean() -> bool {
    // TODO: return either true or false
    unimplemented!()
}

fn get_character() -> char {
    // TODO: return any character
    unimplemented!()
}

// ----------------- Tests -----------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_integer() {
        let val = get_integer();
        assert_eq!(val, 42, "Integer should be 42");
    }

    #[test]
    fn test_get_float() {
        let val = get_float();
        assert!((val - 3.14).abs() < f64::EPSILON, "Float should be 3.14");
    }

    #[test]
    fn test_get_boolean() {
        let val = get_boolean();
        assert!(val, "Boolean should be true");
    }

    #[test]
    fn test_get_character() {
        let val = get_character();
        assert_eq!(val, 'R', "Character should be 'R'");
    }
}

