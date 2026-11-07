# Lesson 07 — Written answers

## 9. When is `panic!` the right choice and when is `Result`? Give one example of each from `taskr`.

Answer:

## 10. Roughly what does `x?` expand to when `x: Result<T, E>`?

Answer:

## 11. Why is an `enum ParseError` better than `String` as the error type for `parse_command`?

Answer:

## 12. Why doesn't this compile, and how would you fix it?

```rust
fn main() {
    let n: i32 = "5".parse()?;
    println!("{n}");
}
```

Answer:
