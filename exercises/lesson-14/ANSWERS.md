# Lesson 14 — Written answers

Answer each question in your own words below the question.

---

**Q1. Fix the compile error.** Explain the error, then fix it two ways: once with `move`, and
once with `thread::scope`.

```rust
use std::thread;

fn main() {
    let names = vec!["ann", "bob"];
    let handle = thread::spawn(|| {
        println!("{names:?}");
    });
    handle.join().unwrap();
}
```

Answer:

---

**Q2. Why not `Rc`?** This doesn't compile. Why not, and what's the fix? What do the `Send` and
`Sync` traits mean?

```rust
use std::rc::Rc;
use std::thread;

fn main() {
    let data = Rc::new(5);
    let d = Rc::clone(&data);
    thread::spawn(move || println!("{d}")).join().unwrap();
}
```

Answer:

---

**Q3. Deadlock.** Describe how two threads and two mutexes can deadlock. Give one simple rule
that prevents it.

Answer:

---

**Q4. Channels vs shared state.** In the homework, why is a channel (message passing) a nicer fit
for the background saver than sharing an `Arc<Mutex<TaskList>>` between the REPL and the saver?

Answer:

---

**Q5. Lost saves.** If `main` returned without joining the saver thread, what could happen to the
user's last change? Why does `join()` fix it?

Answer:
