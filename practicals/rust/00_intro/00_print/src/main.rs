// 🦀 Rust Exercise: Printing
//
// Complete the functions below using `println!`.
// Run `px` to check your answers.

fn print_hello() {
    // TODO: print "Hello, Rust!" to the console
    unimplemented!()
}

fn print_sum(a: i32, b: i32) {
    // TODO: print "Sum is: X" where X is the sum of a and b
    unimplemented!()
}

// ----------------- Tests -----------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Write};
    use std::str;

    // Helper function to capture printed output
    fn capture_output<F: FnOnce()>(func: F) -> String {
        let mut buffer = Vec::new();
        {
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            let original = std::mem::replace(&mut handle, io::sink());
            func();
            std::mem::replace(&mut handle, original);
        }
        String::from_utf8(buffer).unwrap_or_default()
    }

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

