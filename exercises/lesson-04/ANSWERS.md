# Lesson 04 — Written answers

## Q1. What's the difference between `fn area(&self)`, `fn scale(&mut self)`, and `fn into_square(self)`? When would you use each?

Answer:

## Q2. Why does this fail, and how would you fix it?

```rust
struct User { name: String }
let u = User { name: String::from("Ann") };
u.name.push_str("ie");
```

Answer:

## Q3. What does `#[derive(Debug, PartialEq)]` do? Why can't you `#[derive(Copy)]` on a struct with a `String` field?

Answer:

## Q4. Why is a stable `id` (from a `next_id` counter) better than a list position for identifying records that can be deleted?

Answer:
