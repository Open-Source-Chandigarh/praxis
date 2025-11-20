// 🦀 Rust Exercise: Printing
//
// Complete the functions below using `println!`.
// Run `px` to check your answers.

fn print_hello() {
    // TODO: print "Hello, Rust!" to the console
}

fn print_sum(a: i32, b: i32) {
    // TODO: print "Sum is: X" where X is the sum of a and b
}

// ----------------- Tests -----------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_hello() {
        // This test just ensures the function runs
        print_hello();
    }

    #[test]
    fn test_print_sum() {
        print_sum(3, 4);
        // Function runs without panic
    }
}

