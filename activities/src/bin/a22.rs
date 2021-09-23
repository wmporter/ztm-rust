// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    match b {
        0 => None,
        _ => Some(a / b),
    }
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}

#[cfg(test)]
mod test {

    use crate::*;
    #[test]
    fn test_clamp_in_range() {
        let result = clamp(6, 5, 8);
        assert_eq!(result, 6, "middle value not returned");
    }

    #[test]
    fn test_clamp_n_smaller() {
        let result = clamp(2, 8, 10);
        assert_eq!(result, 8, "middle value not returned");
    }

    #[test]
    fn test_div() {
        assert_eq!(div(5, 2), Some(2), "bad division");
    }

    #[test]
    fn test_div_by_zero() {
        assert_eq!(div(4, 0), None, "bad division");
    }

    #[test]
    fn test_concat() {
        assert_eq!(String::from("catdog"), concat("cat", "dog"), "concat fail");
    }

    #[test]
    fn test_concat_again() {
        assert_eq!(String::from("dog"), concat("", "dog"), "concat fail");
    }
}
