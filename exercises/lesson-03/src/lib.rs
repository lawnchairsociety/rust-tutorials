//! Lesson 03 practice problems: ownership, borrowing, slices, and a little `Vec`.
//!
//! Replace each `todo!()` with your implementation, then run:
//!
//! ```text
//! cd exercises
//! cargo test -p lesson-03
//! ```

/// Problem 1: the text before the first space, or the whole string if there's no space.
///
/// Try a `for` loop over `s.char_indices()` (gives `(byte_index, char)` pairs).
///
/// Example: `first_word("buy milk")` → `"buy"`
pub fn first_word(s: &str) -> &str {
    todo!()
}

/// Problem 2: return a new owned greeting.
///
/// Example: `make_greeting("Ferris")` → `"Hello, Ferris!"`
pub fn make_greeting(name: &str) -> String {
    todo!()
}

/// Problem 3: append `"!"` to the string in place.
///
/// Example: `"hi"` becomes `"hi!"`
pub fn exclaim(s: &mut String) {
    todo!()
}

/// Problem 4: total length (in bytes, i.e. `.len()`) of all the words.
///
/// Example: `["ab", "cde"]` → `5`
pub fn total_len(words: &[String]) -> usize {
    todo!()
}

/// Problem 5: take ownership of `v` and return a vector with the elements reversed.
///
/// Use a loop (e.g. over `(0..v.len()).rev()`), not `v.reverse()`.
///
/// Example: `vec![1, 2, 3]` → `vec![3, 2, 1]`
pub fn reversed(v: Vec<i32>) -> Vec<i32> {
    todo!()
}

/// Problem 6: count the vowels a, e, i, o, u (upper or lower case).
///
/// Example: `count_vowels("Hello World")` → `3`
pub fn count_vowels(s: &str) -> usize {
    todo!()
}

/// Problem 7: push the numbers `1..=n` onto the end of `v`.
///
/// Example: `v = vec![0]`, `n = 3` → `v == vec![0, 1, 2, 3]`
pub fn push_up_to(v: &mut Vec<i32>, n: i32) {
    todo!()
}

/// Problem 8: double every element in place.
///
/// Example: `[1, 2, 3]` → `[2, 4, 6]`
pub fn double_all(nums: &mut [i32]) {
    todo!()
}

/// Problem 9: index of the longest word (the first one if several tie).
/// You may assume `words` is not empty.
///
/// Example: `["a", "abc", "ab", "xyz"]` → `1`
pub fn index_of_longest(words: &[String]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(words: &[&str]) -> Vec<String> {
        let mut v = Vec::new();
        for w in words {
            v.push(w.to_string());
        }
        v
    }

    #[test]
    fn first_word_cases() {
        assert_eq!(first_word("buy milk"), "buy");
        assert_eq!(first_word("hello"), "hello");
        assert_eq!(first_word(""), "");
        assert_eq!(first_word("héllo wörld"), "héllo");
        let owned = String::from("call mom today");
        assert_eq!(first_word(&owned), "call");
    }

    #[test]
    fn greeting() {
        assert_eq!(make_greeting("Ferris"), "Hello, Ferris!");
        assert_eq!(make_greeting(""), "Hello, !");
    }

    #[test]
    fn exclaim_in_place() {
        let mut s = String::from("hi");
        exclaim(&mut s);
        assert_eq!(s, "hi!");
        exclaim(&mut s);
        assert_eq!(s, "hi!!");
    }

    #[test]
    fn total_length() {
        assert_eq!(total_len(&strings(&["ab", "cde"])), 5);
        assert_eq!(total_len(&[]), 0);
        // A slice of a Vec works too:
        let words = strings(&["a", "bb", "ccc"]);
        assert_eq!(total_len(&words[1..]), 5);
    }

    #[test]
    fn reverse_vec() {
        assert_eq!(reversed(vec![1, 2, 3]), vec![3, 2, 1]);
        assert_eq!(reversed(vec![]), Vec::<i32>::new());
        assert_eq!(reversed(vec![7]), vec![7]);
    }

    #[test]
    fn vowels() {
        assert_eq!(count_vowels("Hello World"), 3);
        assert_eq!(count_vowels("AEIOU aeiou"), 10);
        assert_eq!(count_vowels("rhythm"), 0);
        assert_eq!(count_vowels(""), 0);
    }

    #[test]
    fn push_numbers() {
        let mut v = vec![0];
        push_up_to(&mut v, 3);
        assert_eq!(v, vec![0, 1, 2, 3]);

        let mut empty = Vec::new();
        push_up_to(&mut empty, 0);
        assert!(empty.is_empty());
    }

    #[test]
    fn doubling() {
        let mut nums = [1, 2, 3];
        double_all(&mut nums);
        assert_eq!(nums, [2, 4, 6]);

        let mut v = vec![5, -5, 0, 10];
        double_all(&mut v[1..3]); // only the middle two
        assert_eq!(v, vec![5, -10, 0, 10]);
    }

    #[test]
    fn longest_index() {
        assert_eq!(index_of_longest(&strings(&["a", "abc", "ab", "xyz"])), 1);
        assert_eq!(index_of_longest(&strings(&["only"])), 0);
        assert_eq!(index_of_longest(&strings(&["aa", "b", "cccc"])), 2);
    }
}
