//! Lesson 14 — Concurrency practice problems.
//!
//! Replace each `todo!()` with a working implementation, then run:
//!
//! ```text
//! cd exercises && cargo test -p lesson-14
//! ```
//!
//! Every problem must actually use threads, as described in its doc comment. The tests can't
//! check that, but your reviewer will!
//! (Unused-variable / unused-import warnings are expected until you implement each function.)

use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

/// Problem 1: Sum `nums` using `workers` threads created with `thread::spawn`.
///
/// Split the vector into `workers` roughly equal chunks, spawn one thread per chunk, each
/// returning its partial sum, then `join` all handles and add up the results.
///
/// `thread::spawn` requires `'static` data, so each thread must **own** its chunk
/// (e.g. `chunk.to_vec()` moved into the closure).
///
/// `workers` is at least 1. `nums` may be empty.
pub fn spawn_sum(nums: Vec<i64>, workers: usize) -> i64 {
    todo!()
}

/// Problem 2: Same as problem 1, but use `thread::scope` so threads can **borrow** `nums`
/// directly. No copying!
pub fn scoped_sum(nums: &[i64], workers: usize) -> i64 {
    todo!()
}

/// Problem 3: Spawn `n` threads. Thread `i` (0-based) sends the message `"worker i"` over an
/// `mpsc` channel. Collect all `n` messages in the main thread and return them **sorted**,
/// because the arrival order is not deterministic.
///
/// Hint: clone the `Sender` for each thread, and `drop` the original so the receiving loop ends.
pub fn collect_messages(n: usize) -> Vec<String> {
    todo!()
}

/// Problem 4: Spawn `threads` threads that each increment a shared counter `increments` times.
/// Use `Arc<Mutex<u64>>`. Return the final value.
///
/// Example: `shared_counter(4, 1000)` == `4000`.
pub fn shared_counter(threads: usize, increments: u64) -> u64 {
    todo!()
}

/// Problem 5: Count words (split on whitespace, lowercased) across all `texts`, processing
/// each text in its own thread. Merge into one `HashMap<String, usize>`.
///
/// You may either share an `Arc<Mutex<HashMap<..>>>` between threads, **or** have each thread
/// return its own map from `join()` and merge them in the main thread. Try both if you like!
/// (The second approach avoids lock contention. Why?)
pub fn parallel_word_count(texts: Vec<String>) -> HashMap<String, usize> {
    todo!()
}

/// Problem 6: A two-stage pipeline connected by channels.
///
/// - Stage 1 thread: receives numbers from the main thread, sends `n * n` onwards.
/// - Stage 2 thread: receives squares, sends `sq + 1` onwards.
/// - Main thread: sends every input into stage 1, then drops its sender, then collects all
///   results from stage 2 **in order** (a single chain of channels preserves order).
///
/// Example: `pipeline(vec![1, 2, 3])` == `vec![2, 5, 10]`.
pub fn pipeline(inputs: Vec<i64>) -> Vec<i64> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_spawn_sum() {
        let v: Vec<i64> = (1..=100).collect();
        assert_eq!(spawn_sum(v.clone(), 1), 5050);
        assert_eq!(spawn_sum(v.clone(), 3), 5050);
        assert_eq!(spawn_sum(v, 200), 5050);
        assert_eq!(spawn_sum(vec![], 4), 0);
    }

    #[test]
    fn p2_scoped_sum() {
        let v: Vec<i64> = (1..=1000).collect();
        assert_eq!(scoped_sum(&v, 1), 500_500);
        assert_eq!(scoped_sum(&v, 7), 500_500);
        assert_eq!(scoped_sum(&[], 2), 0);
        assert_eq!(v.len(), 1000); // still ours: we only lent it out
    }

    #[test]
    fn p3_collect_messages() {
        let msgs = collect_messages(3);
        assert_eq!(msgs, vec!["worker 0", "worker 1", "worker 2"]);
        assert!(collect_messages(0).is_empty());
    }

    #[test]
    fn p4_shared_counter() {
        assert_eq!(shared_counter(4, 1000), 4000);
        assert_eq!(shared_counter(1, 5), 5);
        assert_eq!(shared_counter(0, 5), 0);
    }

    #[test]
    fn p5_parallel_word_count() {
        let texts = vec![
            "the cat".to_string(),
            "The dog".to_string(),
            "a cat and THE bird".to_string(),
        ];
        let counts = parallel_word_count(texts);
        assert_eq!(counts["the"], 3);
        assert_eq!(counts["cat"], 2);
        assert_eq!(counts["bird"], 1);
        assert_eq!(counts.len(), 6);
        assert!(parallel_word_count(vec![]).is_empty());
    }

    #[test]
    fn p6_pipeline() {
        assert_eq!(pipeline(vec![1, 2, 3]), vec![2, 5, 10]);
        assert_eq!(pipeline(vec![]), Vec::<i64>::new());
        let big: Vec<i64> = (0..1000).collect();
        let out = pipeline(big.clone());
        assert_eq!(out.len(), 1000);
        assert!(out.iter().zip(&big).all(|(o, i)| *o == i * i + 1));
    }
}
