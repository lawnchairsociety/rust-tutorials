//! Private helpers. This module is declared WITHOUT `pub` in lib.rs, so code
//! outside this crate can never see it. Inside the crate, `pub(crate)` items
//! here are usable from anywhere.

/// Problem 2: Trim surrounding whitespace and lowercase the name.
/// `normalize_name("  Green Apple ") == "green apple"`
pub(crate) fn normalize_name(name: &str) -> String {
    todo!()
}
