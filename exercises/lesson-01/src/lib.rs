//! Lesson 01 practice problems.
//!
//! Replace each `todo!()` with your implementation, then run:
//!
//! ```text
//! cd exercises
//! cargo test -p lesson-01
//! ```

/// Problem 1: convert degrees Celsius to degrees Fahrenheit.
///
/// Formula: F = C × 9/5 + 32.
///
/// Example: `celsius_to_fahrenheit(100.0)` → `212.0`
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    todo!()
}

/// Problem 2: how many seconds are in `days` days?
///
/// Define constants (e.g. `SECONDS_PER_MINUTE`) instead of writing magic numbers.
///
/// Example: `seconds_in_days(1)` → `86_400`
pub fn seconds_in_days(days: u64) -> u64 {
    todo!()
}

/// Problem 3: return the pair with its two elements swapped.
///
/// Example: `swap_pair((1, 2))` → `(2, 1)`
pub fn swap_pair(pair: (i32, i32)) -> (i32, i32) {
    todo!()
}

/// Problem 4: add up all five numbers using indexing (`nums[0]`, ...).
///
/// Example: `sum_of_five([1, 2, 3, 4, 5])` → `15`
pub fn sum_of_five(nums: [i32; 5]) -> i32 {
    todo!()
}

/// Problem 5: describe a person using `format!`.
///
/// Example: `describe("Alice", 30)` → `"Alice is 30 years old"`
pub fn describe(name: &str, age: u32) -> String {
    todo!()
}

/// Problem 6: return `(a / b, a % b)`.
///
/// Example: `quotient_and_remainder(17, 5)` → `(3, 2)`
pub fn quotient_and_remainder(a: i32, b: i32) -> (i32, i32) {
    todo!()
}

/// Problem 7: the average of three numbers.
///
/// Example: `average(1.0, 2.0, 6.0)` → `3.0`
pub fn average(a: f64, b: f64, c: f64) -> f64 {
    todo!()
}

/// Problem 8: name left-aligned in a field 8 wide, score right-aligned in a field
/// 4 wide, each field surrounded by `|`.
///
/// Example: `padded_score("Ann", 42)` → `"|Ann     |  42|"`
pub fn padded_score(name: &str, score: u32) -> String {
    todo!()
}

/// Problem 9: convert to `u8`, clamping anything above 255 to 255.
///
/// Examples: `to_u8_saturating(7)` → `7`, `to_u8_saturating(1000)` → `255`
pub fn to_u8_saturating(x: u32) -> u8 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn celsius_boiling_and_freezing() {
        assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
    }

    #[test]
    fn celsius_fractional() {
        // 37 °C = 98.6 °F (compare floats with a tolerance)
        assert!((celsius_to_fahrenheit(37.0) - 98.6).abs() < 1e-9);
        assert_eq!(celsius_to_fahrenheit(-40.0), -40.0);
    }

    #[test]
    fn seconds() {
        assert_eq!(seconds_in_days(0), 0);
        assert_eq!(seconds_in_days(1), 86_400);
        assert_eq!(seconds_in_days(365), 31_536_000);
    }

    #[test]
    fn swap() {
        assert_eq!(swap_pair((1, 2)), (2, 1));
        assert_eq!(swap_pair((-5, 5)), (5, -5));
    }

    #[test]
    fn sum_five() {
        assert_eq!(sum_of_five([1, 2, 3, 4, 5]), 15);
        assert_eq!(sum_of_five([10, -10, 0, 7, -7]), 0);
    }

    #[test]
    fn describe_person() {
        assert_eq!(describe("Alice", 30), "Alice is 30 years old");
        assert_eq!(describe("Ferris", 7), "Ferris is 7 years old");
    }

    #[test]
    fn division() {
        assert_eq!(quotient_and_remainder(17, 5), (3, 2));
        assert_eq!(quotient_and_remainder(10, 2), (5, 0));
        assert_eq!(quotient_and_remainder(-7, 2), (-3, -1));
    }

    #[test]
    fn averages() {
        assert_eq!(average(1.0, 2.0, 6.0), 3.0);
        assert_eq!(average(1.0, 2.0, 3.0), 2.0);
        assert_eq!(average(0.5, 0.5, 0.5), 0.5);
    }

    #[test]
    fn padding() {
        assert_eq!(padded_score("Ann", 42), "|Ann     |  42|");
        assert_eq!(padded_score("Bartholo", 1234), "|Bartholo|1234|");
    }

    #[test]
    fn saturating() {
        assert_eq!(to_u8_saturating(7), 7);
        assert_eq!(to_u8_saturating(255), 255);
        assert_eq!(to_u8_saturating(256), 255);
        assert_eq!(to_u8_saturating(1000), 255);
    }
}
