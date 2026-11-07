//! Lesson 06 practice problems: Vec, String and HashMap.
//!
//! Replace every `todo!()` with a real implementation, then run:
//!
//! ```text
//! cd exercises && cargo test -p lesson-06
//! ```
//!
//! Warnings about unused variables are expected until you implement a function.

use std::collections::HashMap;

/// Problem 1: Return the items with duplicates removed, keeping the
/// FIRST occurrence of each value and preserving the original order.
///
/// ```text
/// unique_in_order(&[3, 1, 3, 2, 1]) == vec![3, 1, 2]
/// ```
pub fn unique_in_order(items: &[i32]) -> Vec<i32> {
    todo!()
}

/// Problem 2: Return `(number_of_bytes, number_of_chars)` for `s`.
///
/// ```text
/// byte_and_char_len("héllo") == (6, 5)   // 'é' takes two bytes in UTF-8
/// ```
pub fn byte_and_char_len(s: &str) -> (usize, usize) {
    todo!()
}

/// Problem 3: Capitalize the first letter of every word. Words are separated
/// by whitespace; the result joins them with a single space. The rest of each
/// word is left unchanged.
///
/// ```text
/// capitalize_words("hello  wide world") == "Hello Wide World"
/// capitalize_words("élan vital")       == "Élan Vital"
/// ```
pub fn capitalize_words(s: &str) -> String {
    todo!()
}

/// Problem 4: Count how often each word appears. Words are separated by
/// whitespace and compared case-insensitively (store them lowercased).
///
/// ```text
/// word_counts("the cat The dog")  ->  {"the": 2, "cat": 1, "dog": 1}
/// ```
pub fn word_counts(text: &str) -> HashMap<String, usize> {
    todo!()
}

/// Problem 5: Return the most common word in `words` (exact, case-sensitive
/// comparison). If several words tie, return the alphabetically smallest one.
/// Return `None` for an empty slice.
///
/// ```text
/// most_common(&["b", "a", "b", "a", "c"]) == Some("a".to_string())
/// ```
pub fn most_common(words: &[&str]) -> Option<String> {
    todo!()
}

/// Problem 6: Group words by their length in characters. Within each group,
/// keep the words in the order they appeared.
///
/// ```text
/// group_by_length(&["hi", "yo", "hey"]) -> {2: ["hi", "yo"], 3: ["hey"]}
/// ```
pub fn group_by_length(words: &[&str]) -> HashMap<usize, Vec<String>> {
    todo!()
}

/// Problem 7: Convert one lowercase ASCII word to pig latin:
/// - if it starts with a vowel (a, e, i, o, u), add "-hay":   "apple" -> "apple-hay"
/// - otherwise move the first letter to the end and add "ay": "first" -> "irst-fay"
/// An empty string returns an empty string.
pub fn pig_latin(word: &str) -> String {
    todo!()
}

/// Problem 8: Return the median of `values`, or `None` if it is empty.
/// You may sort the vector in place. For an even number of values, the
/// median is the average of the two middle values.
///
/// ```text
/// median(&mut vec![5, 1, 3])     == Some(3.0)
/// median(&mut vec![4, 1, 3, 2])  == Some(2.5)
/// ```
pub fn median(values: &mut Vec<i32>) -> Option<f64> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_unique_in_order() {
        assert_eq!(unique_in_order(&[3, 1, 3, 2, 1]), vec![3, 1, 2]);
        assert_eq!(unique_in_order(&[]), Vec::<i32>::new());
        assert_eq!(unique_in_order(&[7, 7, 7]), vec![7]);
    }

    #[test]
    fn p2_byte_and_char_len() {
        assert_eq!(byte_and_char_len("hello"), (5, 5));
        assert_eq!(byte_and_char_len("héllo"), (6, 5));
        assert_eq!(byte_and_char_len("🦀"), (4, 1));
        assert_eq!(byte_and_char_len(""), (0, 0));
    }

    #[test]
    fn p3_capitalize_words() {
        assert_eq!(capitalize_words("hello  wide world"), "Hello Wide World");
        assert_eq!(capitalize_words("élan vital"), "Élan Vital");
        assert_eq!(capitalize_words("already Fine"), "Already Fine");
        assert_eq!(capitalize_words(""), "");
    }

    #[test]
    fn p4_word_counts() {
        let counts = word_counts("the cat The dog");
        assert_eq!(counts.len(), 3);
        assert_eq!(counts.get("the"), Some(&2));
        assert_eq!(counts.get("cat"), Some(&1));
        assert_eq!(counts.get("dog"), Some(&1));
        assert!(word_counts("   ").is_empty());
    }

    #[test]
    fn p5_most_common() {
        assert_eq!(most_common(&["b", "a", "b", "a", "c"]), Some("a".to_string()));
        assert_eq!(most_common(&["x", "y", "y"]), Some("y".to_string()));
        assert_eq!(most_common(&[]), None);
    }

    #[test]
    fn p6_group_by_length() {
        let groups = group_by_length(&["hi", "yo", "hey", "é"]);
        assert_eq!(groups.len(), 3);
        assert_eq!(groups[&2], vec!["hi".to_string(), "yo".to_string()]);
        assert_eq!(groups[&3], vec!["hey".to_string()]);
        assert_eq!(groups[&1], vec!["é".to_string()]);
    }

    #[test]
    fn p7_pig_latin() {
        assert_eq!(pig_latin("apple"), "apple-hay");
        assert_eq!(pig_latin("first"), "irst-fay");
        assert_eq!(pig_latin("a"), "a-hay");
        assert_eq!(pig_latin(""), "");
    }

    #[test]
    fn p8_median() {
        assert_eq!(median(&mut vec![5, 1, 3]), Some(3.0));
        assert_eq!(median(&mut vec![4, 1, 3, 2]), Some(2.5));
        assert_eq!(median(&mut vec![]), None);
        assert_eq!(median(&mut vec![-1]), Some(-1.0));
    }
}
