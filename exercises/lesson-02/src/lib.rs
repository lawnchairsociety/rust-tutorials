//! Lesson 02 practice problems.
//!
//! Replace each `todo!()` with your implementation, then run:
//!
//! ```text
//! cd exercises
//! cargo test -p lesson-02
//! ```

/// Problem 1: classic FizzBuzz for a single number.
///
/// - multiple of 3 and 5 → `"FizzBuzz"`
/// - multiple of 3 → `"Fizz"`
/// - multiple of 5 → `"Buzz"`
/// - otherwise → the number itself, e.g. `"7"` (use `n.to_string()`)
pub fn fizzbuzz(n: u32) -> String {
    todo!()
}

/// Problem 2: n! computed with a loop (no recursion). `factorial(0)` is `1`.
///
/// Example: `factorial(5)` → `120`
pub fn factorial(n: u64) -> u64 {
    todo!()
}

/// Problem 3: is `n` a prime number? (`0` and `1` are not.)
///
/// Example: `is_prime(13)` → `true`, `is_prime(15)` → `false`
pub fn is_prime(n: u32) -> bool {
    todo!()
}

/// Problem 4: the sum of all numbers strictly *below* `limit` that are multiples of 3 or 5.
///
/// Example: `sum_of_multiples(10)` → `3 + 5 + 6 + 9 = 23`
pub fn sum_of_multiples(limit: u32) -> u32 {
    todo!()
}

/// Problem 5: number of Collatz steps to reach 1.
///
/// Each step: if `n` is even, `n = n / 2`; otherwise `n = 3 * n + 1`.
///
/// Example: `collatz_steps(6)` → `8` (6, 3, 10, 5, 16, 8, 4, 2, 1)
pub fn collatz_steps(n: u64) -> u32 {
    todo!()
}

/// Problem 6: letter grade using `match` with ranges.
///
/// 90..=100 → 'A', 80..=89 → 'B', 70..=79 → 'C', 60..=69 → 'D', 0..=59 → 'F',
/// anything above 100 → '?'
pub fn letter_grade(score: u32) -> char {
    todo!()
}

/// Problem 7: day number to name using `match`.
///
/// 1 → "Monday", 2 → "Tuesday", ..., 7 → "Sunday", anything else → "Invalid"
pub fn day_name(n: u8) -> String {
    todo!()
}

/// Problem 8: count decimal digits using a loop (no string conversion!).
///
/// Examples: `count_digits(0)` → `1`, `count_digits(12345)` → `5`
pub fn count_digits(n: u64) -> u32 {
    todo!()
}

/// Problem 9: the largest element, found with a `for` loop.
///
/// Example: `max_of_array([3, -1, 9, 4, 9, 0])` → `9`
pub fn max_of_array(nums: [i32; 6]) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fizzbuzz_values() {
        assert_eq!(fizzbuzz(1), "1");
        assert_eq!(fizzbuzz(3), "Fizz");
        assert_eq!(fizzbuzz(5), "Buzz");
        assert_eq!(fizzbuzz(15), "FizzBuzz");
        assert_eq!(fizzbuzz(98), "98");
        assert_eq!(fizzbuzz(99), "Fizz");
        assert_eq!(fizzbuzz(100), "Buzz");
    }

    #[test]
    fn factorials() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(20), 2_432_902_008_176_640_000);
    }

    #[test]
    fn primes() {
        let primes = [2, 3, 5, 7, 11, 13, 97, 7919];
        let not_primes = [0, 1, 4, 9, 15, 25, 100, 7917];
        for p in primes {
            assert!(is_prime(p), "{p} should be prime");
        }
        for n in not_primes {
            assert!(!is_prime(n), "{n} should not be prime");
        }
    }

    #[test]
    fn multiples() {
        assert_eq!(sum_of_multiples(0), 0);
        assert_eq!(sum_of_multiples(10), 23);
        assert_eq!(sum_of_multiples(16), 60);
        assert_eq!(sum_of_multiples(1000), 233_168);
    }

    #[test]
    fn collatz() {
        assert_eq!(collatz_steps(1), 0);
        assert_eq!(collatz_steps(2), 1);
        assert_eq!(collatz_steps(6), 8);
        assert_eq!(collatz_steps(27), 111);
    }

    #[test]
    fn grades() {
        assert_eq!(letter_grade(100), 'A');
        assert_eq!(letter_grade(90), 'A');
        assert_eq!(letter_grade(89), 'B');
        assert_eq!(letter_grade(75), 'C');
        assert_eq!(letter_grade(60), 'D');
        assert_eq!(letter_grade(59), 'F');
        assert_eq!(letter_grade(0), 'F');
        assert_eq!(letter_grade(101), '?');
    }

    #[test]
    fn days() {
        assert_eq!(day_name(1), "Monday");
        assert_eq!(day_name(3), "Wednesday");
        assert_eq!(day_name(7), "Sunday");
        assert_eq!(day_name(0), "Invalid");
        assert_eq!(day_name(8), "Invalid");
    }

    #[test]
    fn digits() {
        assert_eq!(count_digits(0), 1);
        assert_eq!(count_digits(7), 1);
        assert_eq!(count_digits(10), 2);
        assert_eq!(count_digits(12345), 5);
        assert_eq!(count_digits(u64::MAX), 20);
    }

    #[test]
    fn max_value() {
        assert_eq!(max_of_array([3, -1, 9, 4, 9, 0]), 9);
        assert_eq!(max_of_array([-5, -4, -3, -2, -1, -6]), -1);
        assert_eq!(max_of_array([0, 0, 0, 0, 0, 1]), 1);
    }
}
