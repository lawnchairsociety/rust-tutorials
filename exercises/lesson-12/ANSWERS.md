# Lesson 12 — Written answers

Answer each question in your own words below the question.

---

**Q1. Fix the compile error.** Why doesn't this compile? Give the smallest fix.

```rust
use std::fs;

fn main() {
    let text = fs::read_to_string("tasks.db")?;
    println!("{text}");
}
```

Answer:

---

**Q2. `Box<dyn Error>` vs a custom error enum.** `taskr` uses a custom `StorageError` enum for
its storage module but `main` returns `Result<(), Box<dyn Error>>`. What does each approach give
you, and why is it reasonable to use both in the same program?

Answer:

---

**Q3. Buffered writing.** You write 10,000 short lines to a file with `writeln!(file, ...)`, first
directly on a `File`, then on a `BufWriter<File>`. Which is faster and why? What must you be
careful about with `BufWriter` if an error happens on the final write?

Answer:

---

**Q4. Arguments.** What does `std::env::args().collect::<Vec<String>>()` contain when you run
`cargo run -- --file x.db add Buy milk`? What's element `0`? Why is the `--` needed with `cargo run`?

Answer:
