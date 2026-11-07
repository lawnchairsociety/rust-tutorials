//! Problem 11: an integration test. This file is compiled as a SEPARATE
//! crate that depends on `lesson_10`, so it can only use the public API,
//! exactly like any other user of the library.

use lesson_10::Stack;

#[test]
fn stack_is_last_in_first_out() {
    let mut s: Stack<i32> = Stack::new();
    todo!("push 1, 2, 3; check len() == 3; pop() returns 3, 2, 1 then None; is_empty()")
}

#[test]
fn peek_does_not_remove() {
    todo!("push \"a\" onto a new stack; peek twice returns Some(&\"a\") both times; len() stays 1")
}
