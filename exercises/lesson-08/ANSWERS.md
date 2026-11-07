# Lesson 08 — Written answers

## 7. What does `pub use temperature::celsius_to_fahrenheit;` in `lib.rs` allow, and why might a library author want it?

Answer:

## 8. Why can the tests call `crate::util::normalize_name` even though `util` is not `pub`? Could an integration test in `tests/` call it?

Answer:

## 9. Why are `Item`'s fields private? What could go wrong if `name` were `pub`?

Answer:

## 10. Predict the compiler error, then give two fixes.

```rust
mod a {
    pub mod b {
        fn hidden() {}
    }
}
fn main() { a::b::hidden(); }
```

Answer:

## 11. What's the difference between `mod task;` and `use task::Task;`?

Answer:
