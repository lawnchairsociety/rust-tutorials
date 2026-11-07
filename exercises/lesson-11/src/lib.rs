//! Lesson 11 — Closures & iterators practice problems.
//!
//! Replace each `todo!()` with a working implementation, then run:
//!
//! ```text
//! cd exercises && cargo test -p lesson-11
//! ```
//!
//! Rule of the game for this lesson: **no `for` or `while` loops** in your
//! solutions. Use iterator adapters and consumers instead.
//! (Unused-variable warnings are expected until you implement each function.)

/// Problem 1: Return the sum of the squares of all **even** numbers in `nums`.
///
/// Example: `sum_of_even_squares(&[1, 2, 3, 4])` == `4 + 16` == `20`.
pub fn sum_of_even_squares(nums: &[i32]) -> i32 {
    todo!()
}

/// Problem 2: Return a new vector containing every word in `words` that is
/// longer than `min_len` characters, converted to UPPERCASE, in the original order.
///
/// Example: `long_words_upper(&["hi", "rust", "is", "great"], 2)` == `["RUST", "GREAT"]`.
pub fn long_words_upper(words: &[&str], min_len: usize) -> Vec<String> {
    todo!()
}

/// Problem 3: Return a closure that adds `n` to its argument.
///
/// Example: `let add5 = make_adder(5); add5(10) == 15`.
///
/// Hint: you will need the `move` keyword.
///
/// (The stub returns a placeholder closure, because a bare `todo!()` can't stand in for an
/// `impl Fn` return type. Replace the whole closure.)
pub fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    |x| todo!()
}

/// Problem 4: Call `f` exactly `times` times and return how many of those calls
/// returned `true`.
///
/// `f` is `FnMut` because the caller's closure may mutate captured state
/// (see the tests).
pub fn count_true(times: usize, mut f: impl FnMut() -> bool) -> usize {
    todo!()
}

/// Problem 5: Return the 0-based index of the first number greater than `limit`,
/// or `None` if there isn't one.
///
/// Example: `first_index_above(&[1, 5, 9], 4)` == `Some(1)`.
pub fn first_index_above(nums: &[i32], limit: i32) -> Option<usize> {
    todo!()
}

/// Problem 6: Return the name with the highest score. If several people tie for the
/// highest score, return the **first** one. Return `None` for an empty slice.
///
/// Example: `top_scorer(&[("ann", 3), ("bob", 7), ("cy", 7)])` == `Some("bob".to_string())`.
///
/// Careful: `max_by_key` returns the *last* maximum on ties. Think about how
/// to get the first one instead (`rev()` is one trick; `fold` is another).
pub fn top_scorer(scores: &[(&str, u32)]) -> Option<String> {
    todo!()
}

/// Problem 7: Turn a slice of lines into `"1: first\n2: second\n..."` —
/// each line prefixed with its 1-based line number and `": "`, joined with `\n`
/// (no trailing newline).
///
/// Example: `number_lines(&["a", "b"])` == `"1: a\n2: b"`.
pub fn number_lines(lines: &[&str]) -> String {
    todo!()
}

/// Problem 8: Remove (in place) every negative number from `nums` and double
/// every remaining number.
///
/// Example: `[3, -1, 4]` becomes `[6, 8]`.
///
/// Hint: `retain` and `iter_mut`.
pub fn drop_negatives_and_double(nums: &mut Vec<i32>) {
    todo!()
}

/// Problem 9: Split `text` into words (on whitespace) and return every distinct
/// lowercase word that appears in **both** `a` and `b`, sorted alphabetically,
/// without duplicates.
///
/// Example: `common_words("The cat sat", "a CAT and the dog")` == `["cat", "the"]`.
pub fn common_words(a: &str, b: &str) -> Vec<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_sum_of_even_squares() {
        assert_eq!(sum_of_even_squares(&[1, 2, 3, 4]), 20);
        assert_eq!(sum_of_even_squares(&[]), 0);
        assert_eq!(sum_of_even_squares(&[1, 3, 5]), 0);
        assert_eq!(sum_of_even_squares(&[-2, 10]), 104);
    }

    #[test]
    fn p2_long_words_upper() {
        assert_eq!(
            long_words_upper(&["hi", "rust", "is", "great"], 2),
            vec!["RUST", "GREAT"]
        );
        assert!(long_words_upper(&["a", "bb"], 5).is_empty());
    }

    #[test]
    fn p3_make_adder() {
        let add5 = make_adder(5);
        assert_eq!(add5(10), 15);
        assert_eq!(add5(-5), 0);
        let sub2 = make_adder(-2);
        assert_eq!(sub2(2), 0);
    }

    #[test]
    fn p4_count_true() {
        let mut calls = 0;
        // true on every third call
        let n = count_true(9, || {
            calls += 1;
            calls % 3 == 0
        });
        assert_eq!(n, 3);
        assert_eq!(calls, 9);
        assert_eq!(count_true(0, || true), 0);
    }

    #[test]
    fn p5_first_index_above() {
        assert_eq!(first_index_above(&[1, 5, 9], 4), Some(1));
        assert_eq!(first_index_above(&[1, 2], 10), None);
        assert_eq!(first_index_above(&[], 0), None);
    }

    #[test]
    fn p6_top_scorer() {
        assert_eq!(
            top_scorer(&[("ann", 3), ("bob", 7), ("cy", 7)]).as_deref(),
            Some("bob")
        );
        assert_eq!(top_scorer(&[("solo", 0)]).as_deref(), Some("solo"));
        assert_eq!(top_scorer(&[]), None);
    }

    #[test]
    fn p7_number_lines() {
        assert_eq!(number_lines(&["a", "b"]), "1: a\n2: b");
        assert_eq!(number_lines(&["only"]), "1: only");
        assert_eq!(number_lines(&[]), "");
    }

    #[test]
    fn p8_drop_negatives_and_double() {
        let mut v = vec![3, -1, 4];
        drop_negatives_and_double(&mut v);
        assert_eq!(v, vec![6, 8]);
        let mut empty: Vec<i32> = vec![-1, -2];
        drop_negatives_and_double(&mut empty);
        assert!(empty.is_empty());
    }

    #[test]
    fn p9_common_words() {
        assert_eq!(
            common_words("The cat sat", "a CAT and the dog"),
            vec!["cat", "the"]
        );
        assert_eq!(common_words("x x x", "x"), vec!["x"]);
        assert!(common_words("one", "two").is_empty());
    }
}
