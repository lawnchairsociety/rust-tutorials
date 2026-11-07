# Lesson 01 — Written answers

Write your answers below each question. Short is fine — explain in your own words.

## Q1. What does `cargo check` do, and why is it faster than `cargo build`?

Answer:

## Q2. Why doesn't this compile? Give two different ways to fix it.

```rust
let total = 0;
total = total + 5;
```

Answer:

## Q3. Predict the output (then run it to check).

```rust
let x = 2;
let x = x * 3;
{
    let x = "six";
    println!("{x}");
}
println!("{x}");
```

My prediction:

Actual output:

## Q4. What happens with `let a: u8 = 255; let b = a + 1;` in a debug build? In a release build?

Answer:
