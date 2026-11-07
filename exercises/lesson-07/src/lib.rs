//! Lesson 07 practice problems: error handling with Result, `?` and custom errors.
//!
//! Replace every `todo!()` with a real implementation, then run:
//!
//! ```text
//! cd exercises && cargo test -p lesson-07
//! ```
//!
//! Rule for this lesson: none of your solutions may call `unwrap()`, `expect()` or `panic!`.

use std::fmt;
use std::num::ParseIntError;

/// Problem 1: Parse an age from text. Surrounding whitespace is allowed.
///
/// - not a whole non-negative number -> `Err("not a number: <input>")` (use the trimmed input)
/// - a number greater than 150       -> `Err("unrealistic age: <n>")`
/// - otherwise                       -> `Ok(age)`
///
/// ```text
/// parse_age(" 42 ") == Ok(42)
/// parse_age("abc")  == Err("not a number: abc".to_string())
/// parse_age("200")  == Err("unrealistic age: 200".to_string())
/// ```
pub fn parse_age(input: &str) -> Result<u8, String> {
    todo!()
}

/// Problem 2: Parse every string as an `i32` and return their sum. Stop at
/// the first string that fails and return its `ParseIntError`.
/// Use the `?` operator.
///
/// ```text
/// sum_strings(&["1", "2", "3"]) == Ok(6)
/// sum_strings(&["1", "x"]).is_err()
/// ```
pub fn sum_strings(items: &[&str]) -> Result<i32, ParseIntError> {
    todo!()
}

/// Problem 3: Return the first even number, or an error message
/// `"no even numbers"` if there is none.
/// Hint: find it as an `Option` first, then convert with `.ok_or(...)`.
pub fn first_even(numbers: &[i32]) -> Result<i32, String> {
    todo!()
}

/// Errors for [`safe_divide`].
#[derive(Debug, PartialEq)]
pub enum MathError {
    DivisionByZero,
    Overflow,
}

/// Problem 4: Divide `a` by `b`, returning `MathError::DivisionByZero` when
/// `b == 0` and `MathError::Overflow` when the result does not fit in an
/// `i32` (this happens for exactly one case: `i32::MIN / -1`).
/// Hint: look up `i32::checked_div`.
pub fn safe_divide(a: i32, b: i32) -> Result<i32, MathError> {
    todo!()
}

/// Errors for [`parse_point`].
#[derive(Debug, PartialEq)]
pub enum PointError {
    /// The input had no comma in it.
    MissingComma,
    /// One coordinate was not an integer; holds that coordinate's trimmed text.
    BadNumber(String),
}

/// Problem 5: Implement `Display` so errors read nicely:
///
/// - `MissingComma`      -> `expected "x,y"`
/// - `BadNumber("abc")`  -> `not a number: abc`
impl fmt::Display for PointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

/// Problem 6: Parse `"x,y"` into a pair of integers. Whitespace around each
/// part is allowed: `" 3 , -4 "` -> `Ok((3, -4))`.
/// Use a helper function plus `?` and `.map_err(...)`. Don't write nested matches.
pub fn parse_point(input: &str) -> Result<(i32, i32), PointError> {
    todo!()
}

/// A tiny bank account used by problems 7 and 8.
#[derive(Debug)]
pub struct Account {
    pub owner: String,
    pub balance: u64,
}

/// Errors for [`Account::withdraw`] and [`transfer`].
#[derive(Debug, PartialEq)]
pub enum BankError {
    InsufficientFunds { needed: u64, available: u64 },
    ZeroAmount,
}

impl fmt::Display for BankError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BankError::InsufficientFunds { needed, available } => {
                write!(f, "insufficient funds: needed {needed}, available {available}")
            }
            BankError::ZeroAmount => write!(f, "amount must be greater than zero"),
        }
    }
}

impl Account {
    /// Problem 7: Withdraw `amount`. Return the new balance on success.
    /// Leave the balance untouched on error.
    /// - `amount == 0`       -> `BankError::ZeroAmount`
    /// - `amount > balance`  -> `BankError::InsufficientFunds { needed: amount, available: balance }`
    pub fn withdraw(&mut self, amount: u64) -> Result<u64, BankError> {
        todo!()
    }
}

/// Problem 8: Move `amount` from `from` to `to`, using `?` on `withdraw`.
/// If the withdrawal fails, neither account changes.
pub fn transfer(from: &mut Account, to: &mut Account, amount: u64) -> Result<(), BankError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_parse_age() {
        assert_eq!(parse_age(" 42 "), Ok(42));
        assert_eq!(parse_age("0"), Ok(0));
        assert_eq!(parse_age("150"), Ok(150));
        assert_eq!(parse_age("abc"), Err("not a number: abc".to_string()));
        assert_eq!(parse_age("-3"), Err("not a number: -3".to_string()));
        assert_eq!(parse_age("200"), Err("unrealistic age: 200".to_string()));
        assert_eq!(parse_age("151"), Err("unrealistic age: 151".to_string()));
    }

    #[test]
    fn p2_sum_strings() {
        assert_eq!(sum_strings(&["1", "2", "3"]), Ok(6));
        assert_eq!(sum_strings(&[]), Ok(0));
        assert!(sum_strings(&["1", "x", "3"]).is_err());
    }

    #[test]
    fn p3_first_even() {
        assert_eq!(first_even(&[1, 3, 4, 6]), Ok(4));
        assert_eq!(first_even(&[1, 3]), Err("no even numbers".to_string()));
        assert_eq!(first_even(&[]), Err("no even numbers".to_string()));
    }

    #[test]
    fn p4_safe_divide() {
        assert_eq!(safe_divide(10, 3), Ok(3));
        assert_eq!(safe_divide(1, 0), Err(MathError::DivisionByZero));
        assert_eq!(safe_divide(i32::MIN, -1), Err(MathError::Overflow));
    }

    #[test]
    fn p5_point_error_display() {
        assert_eq!(PointError::MissingComma.to_string(), "expected \"x,y\"");
        assert_eq!(PointError::BadNumber("abc".into()).to_string(), "not a number: abc");
    }

    #[test]
    fn p6_parse_point() {
        assert_eq!(parse_point("3,4"), Ok((3, 4)));
        assert_eq!(parse_point(" 3 , -4 "), Ok((3, -4)));
        assert_eq!(parse_point("34"), Err(PointError::MissingComma));
        assert_eq!(parse_point("3,abc"), Err(PointError::BadNumber("abc".into())));
        assert_eq!(parse_point(" x ,4"), Err(PointError::BadNumber("x".into())));
    }

    fn account(owner: &str, balance: u64) -> Account {
        Account { owner: owner.to_string(), balance }
    }

    #[test]
    fn p7_withdraw() {
        let mut a = account("ann", 100);
        assert_eq!(a.withdraw(30), Ok(70));
        assert_eq!(a.withdraw(0), Err(BankError::ZeroAmount));
        assert_eq!(
            a.withdraw(500),
            Err(BankError::InsufficientFunds { needed: 500, available: 70 })
        );
        assert_eq!(a.balance, 70);
    }

    #[test]
    fn p8_transfer() {
        let mut a = account("ann", 100);
        let mut b = account("bob", 5);
        assert_eq!(transfer(&mut a, &mut b, 40), Ok(()));
        assert_eq!((a.balance, b.balance), (60, 45));
        assert!(transfer(&mut a, &mut b, 1000).is_err());
        assert_eq!((a.balance, b.balance), (60, 45));
    }
}
