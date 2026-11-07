# Lesson 14 — Fearless Concurrency

> Run code on multiple threads, share data safely, and let the compiler catch data races before they happen. Estimated time: 3–4 hours.

## What you'll learn

- Spawning threads with `thread::spawn` and waiting for them with `join`
- `move` closures and why threads need `'static` data
- Scoped threads with `thread::scope`, which can borrow local data
- Message passing with `mpsc` channels
- Shared state with `Arc<T>` and `Mutex<T>` (plus a peek at `RwLock`)
- What the `Send` and `Sync` traits mean
- Deadlocks, and how to avoid them

---

## 1. Spawning threads

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=3 {
            println!("  spawned thread: {i}");
            thread::sleep(Duration::from_millis(10));
        }
        42 // a thread can return a value
    });

    for i in 1..=2 {
        println!("main thread: {i}");
        thread::sleep(Duration::from_millis(10));
    }

    let answer = handle.join().unwrap(); // wait for the thread; get its return value
    println!("spawned thread returned {answer}");
}
```

- `thread::spawn` takes a closure and runs it on a new OS thread. It returns a `JoinHandle<T>`.
- `join()` blocks until the thread finishes. It returns `Result<T, _>`, which is `Err` if the
  thread panicked.
- **When `main` returns, the whole process exits**, killing any threads still running. If you
  care about a thread's work, `join` it.
- The output interleaving varies from run to run. That's concurrency!

### `move` and `'static`

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("sum = {}", v.iter().sum::<i32>()); // v is now owned by the thread
    });
    handle.join().unwrap();
    // println!("{v:?}"); // ERROR: v was moved
}
```

Without `move`, the closure would *borrow* `v`. A spawned thread could outlive `main`'s stack
frame, so the compiler rejects that: "closure may outlive the current function, but it borrows
`v`". Formally, `spawn` requires its closure to be `'static`, meaning it holds no non-static
borrows. `move` gives the thread ownership.

---

## 2. Scoped threads: borrowing is allowed

Often you just want to split work across threads and wait for all of them before continuing.
`thread::scope` guarantees that every thread spawned inside it is joined before the scope ends,
so the threads **can borrow** local data:

```rust
use std::thread;

fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let mut results = vec![0; 2];

    thread::scope(|s| {
        let (left, right) = data.split_at(data.len() / 2);
        let (r0, r1) = results.split_at_mut(1);
        s.spawn(move || r0[0] = left.iter().sum());   // borrows `left`, mutably borrows r0
        s.spawn(move || r1[0] = right.iter().sum());
    }); // all scoped threads are joined here automatically

    println!("{results:?}");          // [10, 26]
    println!("data still usable: {}", data.len());
}
```

The borrow rules still hold across threads. Two threads can't both get `&mut results`, which is
why the example splits it into two disjoint halves with `split_at_mut`. This is the "fearless"
part: **a data race is a compile error**.

(The `move` here moves the *references* `left` and `r0` into each closure, not the data itself.)

---

## 3. Message passing with channels

"Do not communicate by sharing memory; instead, share memory by communicating." A **channel** has
a sending end and a receiving end. `mpsc` means *multiple producer, single consumer*.

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    for id in 0..3 {
        let tx = tx.clone();                       // each producer gets its own Sender
        thread::spawn(move || {
            tx.send(format!("hello from worker {id}")).unwrap();
        });
    }
    drop(tx); // drop the original Sender, or the loop below would wait forever

    for msg in rx {                                // iterates until ALL senders are dropped
        println!("{msg}");
    }
}
```

The key methods:

| Method | Behaviour |
|---|---|
| `tx.send(v)` | sends `v` (moves it!). `Err` if the receiver is gone |
| `rx.recv()` | **blocks** until a message arrives. `Err` once all senders are dropped and the queue is empty |
| `rx.try_recv()` | never blocks: `Ok(v)`, `Err(TryRecvError::Empty)` or `Err(TryRecvError::Disconnected)` |
| `rx.recv_timeout(d)` | blocks for at most `d` |
| `for v in rx` / `rx.iter()` | `recv()` in a loop until disconnected |

`send` **moves** the value into the channel, so you can't accidentally keep using it in the
sending thread. Ownership makes message passing safe.

### Enums make great messages

```rust
use std::sync::mpsc;
use std::thread;

enum Job {
    Print(String),
    Add(i32, i32),
    Stop,
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let worker = thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(Job::Print(s)) => println!("worker: {s}"),
                Ok(Job::Add(a, b)) => println!("worker: {a} + {b} = {}", a + b),
                Ok(Job::Stop) | Err(_) => break,   // explicit stop, or all senders gone
            }
        }
        println!("worker: shutting down");
    });

    tx.send(Job::Print("hi".into())).unwrap();
    tx.send(Job::Add(2, 3)).unwrap();
    tx.send(Job::Stop).unwrap();
    worker.join().unwrap();                        // wait for the worker to finish cleanly
}
```

This "worker thread with a message enum" pattern is exactly what you'll build in the homework.

---

## 4. Shared state: `Arc<Mutex<T>>`

Sometimes several threads really do need to read and modify the same data.

### `Mutex<T>`

A mutex (mutual exclusion) allows only one thread at a time to access the data inside:

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);
    {
        let mut guard = m.lock().unwrap(); // blocks until we have the lock
        *guard += 1;                       // MutexGuard derefs to &mut i32
    }                                      // guard dropped → lock released (Drop, lesson 13!)
    println!("{:?}", m);
}
```

In Rust, the data lives *inside* the mutex. **You can't touch it without locking.** Many other
languages can't promise that.

`lock()` returns `Err` only if another thread panicked while holding the lock ("poisoning").
`.unwrap()` is a common and reasonable response.

### `Arc<T>`

To give a mutex to several threads, each needs an owning handle. `Rc` (lesson 13) won't do,
because its counter isn't thread-safe. Use **`Arc`** (atomically reference counted) instead:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..8 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                *counter.lock().unwrap() += 1;
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("total = {}", *counter.lock().unwrap()); // always 8000
}
```

`Arc<Mutex<T>>` is the thread-safe counterpart of `Rc<RefCell<T>>`.

| Single-threaded | Multi-threaded |
|---|---|
| `Rc<T>` | `Arc<T>` |
| `RefCell<T>` | `Mutex<T>` / `RwLock<T>` |
| `Rc<RefCell<T>>` | `Arc<Mutex<T>>` |

### `RwLock<T>`

`RwLock` allows **many readers or one writer** at a time: `read()` and `write()` instead of
`lock()`. It's useful when reads vastly outnumber writes.

### Keep lock scopes small

```rust
// BAD: holds the lock while doing slow I/O; every other thread waits
let guard = data.lock().unwrap();
std::fs::write("out.txt", format!("{:?}", *guard))?;

// BETTER: copy what you need, release the lock, then do the slow part
let snapshot = data.lock().unwrap().clone();   // guard is a temporary, dropped at `;`
std::fs::write("out.txt", format!("{snapshot:?}"))?;
```

---

## 5. `Send` and `Sync`

How does the compiler know `Rc` can't cross threads? Through two **marker traits** that the
compiler implements automatically:

- **`Send`**: ownership of a value of this type can be transferred to another thread.
  Almost everything is `Send`. `Rc<T>` is not, because two threads could update its count at once.
- **`Sync`**: `&T` can be shared between threads (`T` is `Sync` if `&T` is `Send`).
  `Mutex<T>` is `Sync`. `RefCell<T>` and `Cell<T>` are not.

`thread::spawn`'s signature requires `F: Send + 'static`. Try to send an `Rc` and you get:

```text
error[E0277]: `Rc<i32>` cannot be sent between threads safely
```

You almost never implement these traits yourself. Just know that they're why the compiler can
promise freedom from data races.

---

## 6. Deadlocks: what Rust *can't* save you from

Rust prevents **data races** (unsynchronized access to the same memory). It does **not** prevent
**deadlocks**, where threads wait for each other forever:

```text
Thread 1: lock(A) ... then wants lock(B)
Thread 2: lock(B) ... then wants lock(A)    → both wait forever
```

Simple rules that avoid most deadlocks:

1. **Always acquire multiple locks in the same order** everywhere in your program.
2. Hold locks for as short a time as possible. Never hold one while waiting on a channel or doing I/O.
3. Prefer message passing (one thread owns the data) when you can.

A related trap: locking the *same* `Mutex` twice on one thread also deadlocks (or panics):

```rust
let a = m.lock().unwrap();
let b = m.lock().unwrap(); // waits for `a`, forever
```

Logical race conditions are also still possible, such as two threads doing "check, then act"
non-atomically. Keep each check-then-act inside a single lock.

---

## Practice problems

All problems live in `exercises/lesson-14/`. Run them with:

```bash
cd exercises && cargo test -p lesson-14
```

1. `spawn_sum`: split work across `thread::spawn` threads (owned chunks), then join and combine.
2. `scoped_sum`: the same with `thread::scope`, borrowing instead of copying.
3. `collect_messages`: many producers, one consumer, plus the "drop the original sender" trick.
4. `shared_counter`: `Arc<Mutex<u64>>`.
5. `parallel_word_count`: choose shared state or join-and-merge.
6. `pipeline`: a chain of channels between stage threads.

Written questions in `exercises/lesson-14/ANSWERS.md`:

- **Q1** `move` vs `scope`
- **Q2** `Rc` across threads, and `Send`/`Sync`
- **Q3** deadlocks
- **Q4** channels vs shared state for the saver
- **Q5** why `join` matters

---

## Homework — taskr step 14

### Starting point

After lesson 13, `taskr`:

- loads `tasks.db` (or `--file <path>`) on startup and **saves synchronously after every mutating command**
- has one-shot mode and the REPL
- has `Report<'a>` for `stats`, `Exporter` trait objects for `export md|csv <path>`, and an
  in-session `undo` history of `TaskList` snapshots
- supports `add`, `done`, `remove`, `show`, `edit`, `list` (filters), `search`, `clear`, `tags`,
  `stats`, `sort`, `export`, `undo`, `help`, `quit`

Saving is fast for a small file, but imagine `tasks.db` lived on a slow network drive: every
command would freeze the prompt. Let's move saving onto a background thread.

### Requirements

1. **New module `src/saver.rs`** with:
   ```rust
   pub enum SaveMsg {
       Save(TaskList),   // a snapshot (clone) of the list to write
       Shutdown,
   }

   pub struct Saver {
       tx: mpsc::Sender<SaveMsg>,
       errors: mpsc::Receiver<StorageError>,  // or String, if StorageError isn't Send-friendly for you
       handle: Option<thread::JoinHandle<()>>,
   }

   impl Saver {
       pub fn start(path: PathBuf) -> Saver;              // spawns the saver thread
       pub fn save(&self, list: &TaskList);               // sends SaveMsg::Save(list.clone())
       pub fn take_errors(&self) -> Vec<StorageError>;    // drains errors with try_recv
       pub fn shutdown(mut self);                         // sends Shutdown and joins the thread
   }
   ```
2. **The saver thread** loops on `rx.recv()`:
   - `Save(list)`: write it using your `storage::save`, but make the write **atomic**: write to
     `<path>.tmp`, then `fs::rename` it over `<path>` (see lesson 12, §2). On error, send the
     error back over the errors channel and keep running.
   - `Shutdown`, or the channel disconnecting (`Err` from `recv`): exit the loop.
3. **REPL integration.**
   - Start a `Saver` after loading the list.
   - After each mutating command (including `undo`), call `saver.save(&list)` instead of saving directly.
   - Before printing each prompt, call `take_errors()` and print any errors as
     `Warning: failed to save: ...`.
   - On `quit` **and** on EOF (Ctrl-D), call `saver.shutdown()`, so the last save always lands on disk.
4. **One-shot mode** can keep saving synchronously (it exits right away anyway). Reuse the same
   atomic write function so both paths behave the same.
5. **No data loss on exit.** Run `taskr`, add a task, type `quit` immediately, then run
   `taskr list`. The task must be there. Explain in a comment *why* the `join` guarantees this.
6. **Tests** (unit tests in `saver.rs`, using temp files):
   - start a saver, send two saves, shut down, then load the file: it contains the *second* list
   - an unwritable path (e.g. a directory that doesn't exist) produces an error from `take_errors()`
     after shutdown, and nothing panics
   - the `.tmp` file doesn't linger after a successful save

### Example session

```text
$ cargo run -q
taskr> add Water plants #home
Added task 1.
taskr> add Pay rent !high
Added task 2.
taskr> quit
Bye!
$ cat tasks.db
1|medium|false|home|Water plants
2|high|false||Pay rent
$ cargo run -q -- -f /no/such/dir/tasks.db
taskr> add Will not save
Added task 1.
taskr> list
Warning: failed to save: No such file or directory (os error 2)
  1. [ ] (Medium) Will not save
taskr> quit
Bye!
```

### Hints

- You need to *send* a `TaskList` to another thread, so it must be `Send`. It will be
  automatically, as long as it only contains `Send` types (`String`, `Vec`, your enums...).
  If you see a `Send` error, check for an `Rc` hiding somewhere.
- If you store the errors as `StorageError`, it must also be `Send`. `io::Error` is `Send`, so
  you should be fine.
- `shutdown(mut self)` takes `self` by value, so nobody can use the saver after shutdown. The
  type system enforces it! Use `self.handle.take()` to get the `JoinHandle` out of the `Option`
  so you can `join()` it.
- Bonus safety: implement `Drop for Saver` to send `Shutdown` and join, so even an early
  return or `?` in `main` doesn't lose data. (If you do this, `shutdown` can simply be `drop(self)`.)
- If the REPL loop and command handling live in one big `main`, this is a good time to extract
  `fn run_repl(list: TaskList, saver: &Saver) -> ...`.
- The test for "contains the second list" works because the channel is FIFO and the single
  saver thread processes messages in order.

### Stretch goals

- **Debounce.** If many `Save` messages pile up, only the latest matters. After receiving one
  `Save`, drain the queue with `try_recv()` and keep only the newest list before writing.
  Add a test that sends 100 saves quickly and checks that the final file has the last state.
- **Autosave interval.** Instead of saving on every command, save at most once every 2 seconds
  using `recv_timeout`. Make sure `Shutdown` still flushes a pending save!
- Use `thread::Builder::new().name("taskr-saver".into())` so the thread has a name in panic
  messages and debuggers.

---

## Submitting for review

1. Make sure everything passes:
   ```bash
   (cd exercises && cargo test -p lesson-14)
   (cd taskr && cargo test)
   ```
2. Commit on your `work-14` branch:
   ```bash
   git add -A && git commit -m "Lesson 14: background saver thread"
   ```
3. Ask Claude: **"Please review lesson 14"**.

---

## Further reading

- The Rust Book, ch. 16: [Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- Rust by Example: [Threads](https://doc.rust-lang.org/rust-by-example/std_misc/threads.html), [Channels](https://doc.rust-lang.org/rust-by-example/std_misc/channels.html)
- std docs: [`std::thread`](https://doc.rust-lang.org/std/thread/index.html) (see `scope`), [`std::sync::mpsc`](https://doc.rust-lang.org/std/sync/mpsc/index.html), [`Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html), [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html)
- [Rust Atomics and Locks](https://marabos.nl/atomics/) by Mara Bos, free online, for when you want to go deep
