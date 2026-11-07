# Lesson 13 — Written answers

Answer each question in your own words below the question.

---

**Q1. Elision.** `first_word(s: &str) -> &str` has no lifetime annotations. Which elision rule(s)
let the compiler fill them in, and what is the full signature it infers?
Then explain why `fn longer(a: &str, b: &str) -> &str` (no annotations) is rejected.

Answer:

---

**Q2. Fix the compile error.** Explain the error in your own words, then fix it *without* changing
`longer`.

```rust
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if b.len() > a.len() { b } else { a }
}

fn main() {
    let result;
    let a = String::from("a long string");
    {
        let b = String::from("short");
        result = longer(&a, &b);
    }
    println!("{result}");
}
```

Answer:

---

**Q3. `Box<dyn Trait>` vs generics.** Compare these two functions. When can you use the first,
when must you use the second, and what's the runtime cost difference?

```rust
fn total_generic<S: Shape>(shapes: &[S]) -> f64 { ... }
fn total_dyn(shapes: &[Box<dyn Shape>]) -> f64 { ... }
```

Answer:

---

**Q4. Rc and RefCell.** (a) What problem does `Rc<T>` solve that `Box<T>` doesn't?
(b) What does `RefCell<T>` do at runtime that the borrow checker normally does at compile time?
(c) What happens if you call `borrow_mut()` twice on the same `RefCell` while the first borrow is still alive?

Answer:

---

**Q5. Report<'a>.** In the homework, `Report<'a>` holds `Vec<&'a Task>`. Why can't you call
`list.add(...)` while a `Report` built from `list` is still in use? Why is that a *good* thing?

Answer:
