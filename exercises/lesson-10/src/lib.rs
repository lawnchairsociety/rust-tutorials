//! Lesson 10 practice problems: writing tests.
//!
//! This time the functions are (mostly) written for you, and the **tests**
//! are the stubs. Each test below has a `todo!()` describing what it should
//! check. Replace it with real assertions.
//!
//! WARNING: this code contains **two planted bugs**. Good tests will find
//! them. When a test you wrote fails, decide whether the test or the code
//! is wrong, then fix the code (not the test!) if it's the code.
//!
//! ```text
//! cd exercises && cargo test -p lesson-10
//! ```

/// Returns true if `year` is a leap year in the Gregorian calendar:
/// divisible by 4, except years divisible by 100, unless also divisible by 400.
pub fn is_leap_year(year: u32) -> bool {
    year % 4 == 0 && year % 100 != 0
}

/// Converts a score in 0..=100 to a letter grade:
/// 90 and above is 'A', 80–89 'B', 70–79 'C', 60–69 'D', below 60 'F'.
///
/// # Panics
///
/// Panics if `score > 100`.
pub fn grade(score: u32) -> char {
    if score > 100 {
        panic!("score out of range: {score}");
    }
    if score > 90 {
        'A'
    } else if score >= 80 {
        'B'
    } else if score >= 70 {
        'C'
    } else if score >= 60 {
        'D'
    } else {
        'F'
    }
}

/// Parses a duration like "1h30m", "45m" or "2h" into total minutes.
/// Each part is a number followed by `h` or `m`; at least one part is required.
///
/// Errors (as a `String` message) for an empty input, a number with no unit,
/// a unit with no number, or an unknown unit.
pub fn parse_duration(s: &str) -> Result<u32, String> {
    let s = s.trim();
    if s.is_empty() {
        return Err("empty duration".to_string());
    }
    let mut total = 0;
    let mut digits = String::new();
    for c in s.chars() {
        if c.is_ascii_digit() {
            digits.push(c);
        } else {
            let minutes_per_unit = unit_minutes(c).ok_or(format!("unknown unit: {c}"))?;
            if digits.is_empty() {
                return Err(format!("missing number before '{c}'"));
            }
            let n: u32 = digits.parse().map_err(|_| format!("number too large: {digits}"))?;
            total += n * minutes_per_unit;
            digits.clear();
        }
    }
    if !digits.is_empty() {
        return Err(format!("missing unit after {digits}"));
    }
    Ok(total)
}

/// A private helper. Unit tests in this file can still test it, because the
/// `tests` module is a child of this module.
fn unit_minutes(unit: char) -> Option<u32> {
    match unit {
        'h' => Some(60),
        'm' => Some(1),
        _ => None,
    }
}

/// Integer division that panics with the message "division by zero" if `b == 0`.
pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("division by zero");
    }
    a / b
}

/// A simple last-in, first-out stack.
#[derive(Debug, Default)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { items: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    /// Problem 9 (doc test): add an `# Examples` section with a code block
    /// to THIS doc comment, showing push followed by peek. `cargo test` runs it.
    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

/// Problem 10 (TDD): convert 1..=3999 to Roman numerals, e.g. 1994 -> "MCMXCIV".
/// Write the tests in `roman_numerals` FIRST, watch them fail, then implement.
pub fn to_roman(n: u32) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Problem 1
    #[test]
    fn leap_years_divisible_by_four() {
        todo!("assert 2024 and 1996 are leap years, and 2023 is not")
    }

    // Problem 2
    #[test]
    fn centuries_are_not_leap_years() {
        todo!("assert 1900 and 2100 are not leap years")
    }

    // Problem 3
    #[test]
    fn every_400_years_is_a_leap_year() {
        todo!("assert 2000 and 1600 are leap years")
    }

    // Problem 4: test the boundaries, not just the middle of each range.
    #[test]
    fn grade_boundaries() {
        todo!("check 100, 90, 89, 80, 79, 70, 69, 60, 59 and 0")
    }

    // Problem 5
    #[test]
    #[should_panic(expected = "score out of range")]
    fn grade_rejects_scores_over_100() {
        todo!("call grade(101)")
    }

    // Problem 6: a test can return Result, which lets you use `?` inside it.
    #[test]
    fn parse_duration_valid() -> Result<(), String> {
        todo!("use `?` to check \"1h30m\" == 90, \"45m\" == 45, \"2h\" == 120, then return Ok(())")
    }

    // Problem 7: check each error case returns Err (use assert!(x.is_err())
    // or match on the message).
    #[test]
    fn parse_duration_errors() {
        todo!("empty string, \"30\", \"h\", \"5x\"")
    }

    // Problem 8: test the private helper directly.
    #[test]
    fn unit_minutes_known_and_unknown() {
        todo!("'h' -> Some(60), 'm' -> Some(1), 's' -> None")
    }

    // Problem 8b
    #[test]
    #[should_panic(expected = "division by zero")]
    fn divide_by_zero_panics() {
        todo!("call divide(1, 0)")
    }

    // Problem 10 (TDD)
    #[test]
    fn roman_numerals() {
        todo!("write assertions for 1, 4, 9, 14, 40, 90, 400, 1994, 3999 BEFORE implementing to_roman")
    }
}
