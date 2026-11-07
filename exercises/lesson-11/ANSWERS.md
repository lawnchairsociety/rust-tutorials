# Lesson 11 — Written answers

Answer each question in your own words below the question. A few sentences is plenty.

---

**Q1. Predict the output.** What does this program print, and why?

```rust
fn main() {
    let v = vec![1, 2, 3];
    let doubled = v.iter().map(|x| {
        println!("doubling {x}");
        x * 2
    });
    println!("before collect");
    let result: Vec<i32> = doubled.collect();
    println!("{result:?}");
}
```

Answer:

---

**Q2. Fix the compile error.** This does not compile. Explain the error and give two different fixes.

```rust
use std::thread;

fn main() {
    let name = String::from("taskr");
    let greet = || println!("hello from {name}");
    let handle = thread::spawn(greet);
    handle.join().unwrap();
}
```

(Hint: it isn't about threads specifically — think about *how* the closure captures `name`
and how long the closure might live. Threads are covered properly in lesson 14.)

Answer:

---

**Q3. Fn, FnMut, FnOnce.** For each closure below, say which is the *most permissive* trait
it implements (`Fn`, `FnMut`, or only `FnOnce`) and why.

```rust
let s = String::from("hi");
let a = || println!("{s}");          // (a)
let mut count = 0;
let mut b = || count += 1;           // (b)
let c = move || drop(s);             // (c)
```

Answer:

---

**Q4. iter vs iter_mut vs into_iter.** Given `let v: Vec<String> = ...;`, what is the item type
of `v.iter()`, `v.iter_mut()` (if `v` is `mut`), and `v.into_iter()`? Which of them can you
still use `v` after?

Answer:
