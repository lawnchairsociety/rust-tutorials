//! Lesson 15 — Polish: clippy, docs & doc tests.
//!
//! There are two parts:
//!
//! - **Part A** (`src/messy.rs`): working but unidiomatic code. Make it clippy-clean.
//! - **Part B** (this file): implement the functions below *and* give each one a
//!   `# Examples` section with at least one doc test.
//!
//! Run everything with:
//!
//! ```text
//! cd exercises
//! cargo test -p lesson-15            # unit tests + doc tests
//! cargo clippy -p lesson-15          # lints
//! cargo doc -p lesson-15 --open      # view the documentation you wrote
//! ```
//!
//! (Unused-variable warnings are expected until you implement each function.)

pub mod messy;

/// Problem 1: Turn a title into a URL-friendly "slug".
///
/// - lowercase everything
/// - keep ASCII letters and digits
/// - turn every run of other characters (spaces, punctuation, ...) into a single `-`
/// - no leading or trailing `-`
///
/// `"Hello, World!"` → `"hello-world"`; `"  Rust  2024 "` → `"rust-2024"`.
///
/// TODO: add an `# Examples` section with a doc test here.
pub fn slugify(title: &str) -> String {
    todo!()
}

/// Problem 2: Shorten `s` to at most `max` **characters** (not bytes!). If it had to be cut,
/// the result ends with `…` (a single character), still within `max` characters in total.
/// If `max` is 0, return an empty string.
///
/// `truncate("hello world", 8)` → `"hello w…"`; `truncate("hi", 8)` → `"hi"`.
///
/// TODO: add an `# Examples` section with a doc test here.
pub fn truncate(s: &str, max: usize) -> String {
    todo!()
}

/// Problem 3: Parse a duration like `"1h30m"`, `"45m"`, `"2h"`, `"90s"` or `"1h2m3s"` into
/// seconds. Units must appear in the order h, m, s, each at most once. Every number must be
/// followed by a unit. Anything else (empty string, unknown unit, missing number, wrong order)
/// returns `None`.
///
/// `parse_duration("1h30m")` → `Some(5400)`.
///
/// TODO: add an `# Examples` section with doc tests for a success **and** a failure case.
pub fn parse_duration(s: &str) -> Option<u64> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_slugify() {
        assert_eq!(slugify("Hello, World!"), "hello-world");
        assert_eq!(slugify("  Rust  2024 "), "rust-2024");
        assert_eq!(slugify("already-a-slug"), "already-a-slug");
        assert_eq!(slugify("!!!"), "");
        assert_eq!(slugify("Café au lait"), "caf-au-lait");
    }

    #[test]
    fn p2_truncate() {
        assert_eq!(truncate("hello world", 8), "hello w…");
        assert_eq!(truncate("hi", 8), "hi");
        assert_eq!(truncate("exact", 5), "exact");
        assert_eq!(truncate("héllo wörld", 6), "héllo…");
        assert_eq!(truncate("abc", 1), "…");
        assert_eq!(truncate("abc", 0), "");
    }

    #[test]
    fn p3_parse_duration() {
        assert_eq!(parse_duration("1h30m"), Some(5400));
        assert_eq!(parse_duration("45m"), Some(2700));
        assert_eq!(parse_duration("2h"), Some(7200));
        assert_eq!(parse_duration("90s"), Some(90));
        assert_eq!(parse_duration("1h2m3s"), Some(3723));
        assert_eq!(parse_duration(""), None);
        assert_eq!(parse_duration("10"), None);
        assert_eq!(parse_duration("5x"), None);
        assert_eq!(parse_duration("m"), None);
        assert_eq!(parse_duration("30m1h"), None);
        assert_eq!(parse_duration("1h1h"), None);
    }
}
