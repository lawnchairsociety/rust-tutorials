# Lesson 03 — Ownership & Borrowing

> Rust's defining feature: how it manages memory safely *without* a garbage collector.
> Estimated time: 2.5–3 hours. Take your time here — everything else builds on it.

## What you'll learn

- The three **ownership rules**
- **Moves**, `clone`, and `Copy` types
- **References** (`&T`) and **mutable references** (`&mut T`), and the borrowing rules
- `String` vs `&str`, and **slices**
- A first look at `Vec<T>`, Rust's growable list

---

## 1. Why ownership?

Every program has to manage memory. Languages usually pick one of:

- **Manual** (C): you call `malloc`/`free`. Fast, but easy to leak memory, free twice, or use
  freed memory.
- **Garbage collected** (Java, Python, Go): a runtime cleans up for you. Safe, but costs
  runtime overhead and pauses.

Rust takes a third path: a set of **ownership rules checked at compile time**. If your
program breaks them, it doesn't compile. There's no runtime cost.

### Stack vs. heap (the short version)

- **Stack**: fast, fixed-size values known at compile time (`i32`, `bool`, `(u8, char)`,
  `[i32; 3]`). Freed automatically when a function returns.
- **Heap**: for data whose size can change or isn't known at compile time (a `String` the
  user types, a list that grows). You get a pointer to it. *Someone* has to free it.

Ownership is Rust's answer to "who frees heap memory, and when?"

---

## 2. The ownership rules

1. Each value has an **owner** (a variable).
2. There can only be **one owner at a time**.
3. When the owner goes **out of scope**, the value is **dropped** (its memory is freed).

```rust
fn main() {
    {
        let s = String::from("hello"); // s owns a heap-allocated String
        println!("{s}");
    } // <- s goes out of scope here; the String is freed automatically
}
```

### `String` vs. string literals

```rust
fn main() {
    let literal = "hello";             // &str: fixed text baked into the program
    let mut owned = String::from("hi"); // String: owned, growable, lives on the heap
    owned.push_str(", there");
    owned.push('!');
    println!("{literal} / {owned}");    // hello / hi, there!
}
```

Ways to make a `String`: `String::from("x")`, `"x".to_string()`, `"x".to_owned()`,
`format!(...)`, `String::new()` (empty).

---

## 3. Moves

Assigning a heap-owning value to another variable **moves** ownership:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;            // ownership moves from s1 to s2
    // println!("{s1}");    // ❌ error[E0382]: borrow of moved value: `s1`
    println!("{s2}");
}
```

Why? If both `s1` and `s2` owned the same heap data, both would try to free it at the end
of `main` — a "double free" bug. Rust prevents that by making `s1` unusable after the move.

```text
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:4:15
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |     println!("{s1}");
  |               ^^^^ value borrowed here after move
  |
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let s2 = s1.clone();
  |                ++++++++
```

**Passing a value to a function also moves it:**

```rust
fn consume(s: String) {
    println!("consumed {s}");
} // s is dropped here

fn main() {
    let s = String::from("bye");
    consume(s);
    // println!("{s}"); // ❌ s was moved into `consume`
}
```

And **returning a value moves it out** to the caller:

```rust
fn make_greeting(name: &str) -> String {
    let g = format!("Hello, {name}!");
    g // ownership moves to the caller
}

fn main() {
    let greeting = make_greeting("Ferris"); // main now owns it
    println!("{greeting}");
}
```

### `clone` — an explicit deep copy

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();   // copies the heap data; both are valid
    println!("{s1} {s2}");
}
```

`clone` is explicit on purpose: it can be expensive, so you should see it in the code.

### `Copy` types

Simple stack-only types are **copied** instead of moved: all integers, floats, `bool`,
`char`, and tuples/arrays made only of `Copy` types.

```rust
fn main() {
    let x = 5;
    let y = x;           // copy, not move
    println!("{x} {y}"); // both fine

    let t = (1, true);
    let u = t;
    println!("{t:?} {u:?}"); // fine — (i32, bool) is Copy
}
```

`&str` is also `Copy` (copying a reference doesn't copy the text it points to).

---

## 4. References and borrowing

Moving values in and out of every function would be tedious. Instead you can **borrow** a
value with a **reference**, written `&`:

```rust
fn length(s: &String) -> usize {  // s is a reference to a String
    s.len()
} // s goes out of scope, but it doesn't own anything, so nothing is dropped

fn main() {
    let s = String::from("hello");
    let n = length(&s);          // lend s to the function
    println!("{s} has {n} bytes"); // s is still ours
}
```

### Mutable references

References are immutable by default. To let a function modify a borrowed value, use
`&mut`:

```rust
fn shout(s: &mut String) {
    s.push_str("!!!");
}

fn main() {
    let mut s = String::from("hey"); // the variable itself must be `mut`
    shout(&mut s);
    println!("{s}"); // hey!!!
}
```

### The borrowing rules

At any given time, you can have **either**:
- any number of **immutable** references (`&T`), **or**
- exactly **one mutable** reference (`&mut T`).

And references must always be **valid** (no dangling references).

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{r1} {r2}");   // ✅ many readers are fine
    // r1 and r2 are not used after this point...

    let r3 = &mut s;         // ✅ ...so a writer is now allowed
    r3.push('!');
    println!("{r3}");
}
```

But this fails, because a reader is still in use while we try to write:

```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &mut s;        // ❌ error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    r2.push('!');
    println!("{r1}");
}
```

A borrow lasts from where it's created to where it's **last used**, not to the end of the
block. That's why the first example compiles.

**Why these rules?** They make *data races* and a whole family of bugs impossible. Example:
if you hold a reference to an element of a list and someone pushes to the list, the list
might reallocate and your reference would point to freed memory. In Rust, that doesn't
compile:

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    v.push(4);              // ❌ cannot borrow `v` as mutable because it is also borrowed as immutable
    println!("{first}");
}
```

### No dangling references

```rust
fn dangle() -> &String {     // ❌ missing lifetime specifier
    let s = String::from("hi");
    &s                       // s is dropped at the end of this function!
}
```

The fix is to return the `String` itself (move ownership out) instead of a reference.

### Dereferencing with `*`

`*` follows a reference to the value it points to. You need it mostly when assigning
through a `&mut` or comparing to non-references:

```rust
fn add_ten(n: &mut i32) {
    *n += 10;
}

fn main() {
    let mut x = 5;
    add_ten(&mut x);
    let r = &x;
    println!("{}", *r == 15); // true
}
```

Method calls auto-dereference, so `s.len()` works whether `s` is a `String` or `&String`.

---

## 5. Slices and `&str`

A **slice** is a reference to a contiguous part of a collection.

```rust
fn main() {
    let s = String::from("hello world");
    let hello: &str = &s[0..5];   // "hello"
    let world: &str = &s[6..];    // "world"
    let whole: &str = &s[..];     // "hello world"
    println!("{hello}|{world}|{whole}");

    let nums = [10, 20, 30, 40];
    let middle: &[i32] = &nums[1..3]; // [20, 30]
    println!("{middle:?}");
}
```

> String slice indices are **byte** positions. Slicing in the middle of a multi-byte
> character (like `é` or `🦀`) panics. Lesson 06 covers UTF-8 properly.

`&str` is a "string slice" — a borrowed view into some string data. String literals are
`&str` slices pointing into the program binary.

### Prefer `&str` and `&[T]` in parameters

```rust
fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}

fn main() {
    let owned = String::from("buy milk");
    println!("{}", first_word(&owned));     // &String coerces to &str automatically
    println!("{}", first_word("call mom")); // literals are already &str
}
```

A function taking `&str` accepts both `&String` and literals. A function taking `&String`
accepts only the former. Same idea: take `&[T]` instead of `&Vec<T>`.

(`Some`/`None` is the `Option` type — lesson 05. Here it means "found at position i" or
"not found".)

### Useful `&str` methods

```rust
fn main() {
    let line = "  add Buy milk  ";
    let t = line.trim();                    // "add Buy milk"
    println!("{}", t.starts_with("add"));   // true
    println!("{}", t.len());                // 12
    println!("{}", t.to_uppercase());       // "ADD BUY MILK" (a new String)
    println!("{}", t.contains("milk"));     // true
    println!("{}", t.replace("milk", "eggs")); // "add Buy eggs" (a new String)
}
```

---

## 6. A first look at `Vec<T>`

A `Vec<T>` ("vector") is an owned, growable list of `T` stored on the heap. We'll use it
lightly now and cover it fully in lesson 06.

```rust
fn main() {
    let mut names: Vec<String> = Vec::new();
    names.push(String::from("Ann"));
    names.push("Bob".to_string());

    let nums = vec![1, 2, 3];          // vec! macro builds one with values

    println!("{} names, first = {}", names.len(), names[0]);
    for n in &nums {                   // iterate by reference: nums is still usable after
        print!("{n} ");
    }
    println!();

    names[1].push_str("by");           // modify an element in place
    println!("{names:?}");             // ["Ann", "Bobby"]
}
```

Ownership applies to the elements, too:

```rust
fn main() {
    let names = vec![String::from("Ann")];
    // let first = names[0];          // ❌ cannot move out of index of `Vec<String>`
    let first = &names[0];            // ✅ borrow it
    let copy = names[0].clone();      // ✅ or clone it
    println!("{first} {copy}");
}
```

Passing a `Vec` to functions — choose the weakest access you need:

```rust
fn print_all(items: &[String]) {           // read-only: take a slice
    for item in items {
        println!("- {item}");
    }
}

fn add_item(items: &mut Vec<String>, item: &str) { // need to grow it: &mut Vec
    items.push(item.to_string());
}

fn rename_first(items: &mut [String], name: &str) { // modify elements, no growing: &mut [T]
    items[0] = name.to_string();
}

fn main() {
    let mut items = vec![String::from("a")];
    add_item(&mut items, "b");
    rename_first(&mut items, "A");
    print_all(&items);
}
```

Iterating over a `Vec` of tuples, with mutation:

```rust
fn main() {
    let mut tasks: Vec<(String, u8, bool)> = vec![(String::from("Buy milk"), 1, false)];

    for (title, priority, done) in &tasks {     // each field is a reference
        println!("{title} {priority} {done}");
    }

    for task in &mut tasks {                    // task: &mut (String, u8, bool)
        task.2 = true;                          // mark all done
    }

    let n = 1;                                  // 1-based position from the user
    if n >= 1 && n <= tasks.len() {
        tasks[n - 1].1 = 3;                     // change priority of task n
    }
    println!("{tasks:?}");
}
```

---

## Practice problems

All in `exercises/lesson-03/src/lib.rs` unless noted. Run: `cargo test -p lesson-03`.

1. **`first_word(s: &str) -> &str`** — the text before the first space, or the whole string.
   Try to do it with a `for` loop over `s.char_indices()` or `s.bytes()` rather than `find`.
2. **`make_greeting(name: &str) -> String`** — `"Hello, <name>!"`.
3. **`exclaim(s: &mut String)`** — append `"!"` to `s` in place.
4. **`total_len(words: &[String]) -> usize`** — sum of the lengths of all words.
5. **`reversed(v: Vec<i32>) -> Vec<i32>`** — take ownership of `v` and return a new vector
   with the elements in reverse order. (Do it with a loop; `.rev()` on a range of indices
   is allowed.)
6. **`count_vowels(s: &str) -> usize`** — count `a e i o u` (either case). Use
   `for c in s.chars()`.
7. **`push_up_to(v: &mut Vec<i32>, n: i32)`** — push `1, 2, ..., n` onto the end of `v`.
8. **`double_all(nums: &mut [i32])`** — double every element in place.
9. **`index_of_longest(words: &[String]) -> usize`** — the index of the longest word (first
   one on ties). Assume the slice isn't empty. (Why return an index instead of the word
   itself? Returning a `&String` borrowed from `words` is possible too — lesson 13 explains
   the *lifetime* rules that make it work.)

Written questions (`exercises/lesson-03/ANSWERS.md`) — for each snippet: does it compile?
If not, *why* (in terms of the ownership/borrowing rules) and how would you fix it?

- **Q1.**
  ```rust
  let a = String::from("x");
  let b = a;
  println!("{a}");
  ```
- **Q2.**
  ```rust
  let mut s = String::from("x");
  let r1 = &mut s;
  let r2 = &mut s;
  r1.push('y');
  r2.push('z');
  ```
- **Q3.**
  ```rust
  let n = 5;
  let m = n;
  println!("{n} {m}");
  ```
- **Q4.**
  ```rust
  fn longest_name(names: Vec<String>) -> usize { /* ... */ 0 }
  let names = vec![String::from("Ann")];
  let n = longest_name(names);
  println!("{} names, longest is {n}", names.len());
  ```
  Also: what would be a better signature for `longest_name`?
- **Q5.** In your own words: why can't you have a `&mut` reference and a `&` reference to
  the same value at the same time?

---

## Homework — taskr step 03

### Starting point

From lesson 02: a REPL with `list`, `help`, `quit`; `format_task`, `print_tasks`, and
`print_help` functions; tasks hardcoded as an array of `(&str, u8, bool)`.

### Requirements

1. Tasks become a **`Vec<(String, u8, bool)>`** (owned titles!) built at startup with the
   same three tasks. The REPL can now **change** the list.
2. Change/add these functions:
   - `fn print_tasks(tasks: &[(String, u8, bool)])` — note the **slice** parameter.
     If the list is empty, print `No tasks.`
   - `fn add_task(tasks: &mut Vec<(String, u8, bool)>, title: &str)` — appends a new
     pending task with default priority `2`.
   - `fn mark_done(tasks: &mut [(String, u8, bool)], n: usize) -> bool` — marks task number
     `n` (**1-based**, as shown by `list`) done. Returns `false` if `n` is out of range.
   - Keep `format_task` taking `&str` for the title.
3. New commands:
   - `add <title>` — e.g. `add Buy eggs` → `Added task 4: Buy eggs`.
     `add` with no title → `Usage: add <title>`.
   - `done <n>` — `done 2` → `Marked task 2 as done.` If `n` isn't a number or is out of
     range: `No task number <n>.`
4. `list`, `help` (update it!), and `quit` still work.

### Parsing commands with arguments (recipe)

`match` on whole strings can't handle "add *anything*". Split the input into the first
word and the rest:

```rust
let (cmd, rest) = match input.split_once(' ') {
    Some((cmd, rest)) => (cmd, rest.trim()),
    None => (input, ""),
};

match cmd {
    "add" => { /* use rest */ }
    "done" => {
        match rest.parse::<usize>() {
            Ok(n) => { /* n is a usize */ }
            Err(_) => println!("No task number {rest}."),
        }
    }
    // ...
    _ => println!("Unknown command: {cmd} (type 'help')"),
}
```

`split_once` returns an `Option` and `parse` returns a `Result`. You'll learn exactly what
those are in lessons 05 and 07 — for now, this `match` shape is all you need.

### Example session

```console
taskr> list
1. [x] (P1) Buy milk
2. [ ] (P2) Write report
3. [ ] (P3) Call mom
3 tasks, 1 done
taskr> add Buy eggs
Added task 4: Buy eggs
taskr> done 2
Marked task 2 as done.
taskr> done 9
No task number 9.
taskr> done two
No task number two.
taskr> add
Usage: add <title>
taskr> list
1. [x] (P1) Buy milk
2. [x] (P2) Write report
3. [ ] (P3) Call mom
4. [ ] (P2) Buy eggs
4 tasks, 2 done
taskr> quit
Bye!
```

### Hints

- Converting a `&str` to an owned `String` for storage: `title.to_string()`.
- Inside `print_tasks`, `for (i, (title, priority, done)) in tasks.iter().enumerate()` gives
  you `title: &String` — pass it straight to `format_task`, it coerces to `&str`.
- Watch the 1-based → 0-based conversion in `mark_done`, and remember `n` could be `0`!
- If you're fighting the borrow checker, read the *whole* error message — it usually tells
  you exactly what's borrowed where.

### Stretch goals

Stretch goals may change the structure above. You can rename or replace a required function
if the commands still behave as specified.

- `undone <n>` to mark a task pending again (share code with `mark_done`).
- `priority <n> <1-3>` to change a task's priority.
- `done` on an already-done task prints `Task 2 is already done.`

---

## Submitting for review

1. `cargo test -p lesson-03` passes; your `ANSWERS.md` is filled in; `taskr` works.
2. Commit on `work-03`: `git add -A && git commit -m "Lesson 03 complete"`.
3. Ask Claude: **"Please review lesson 03"**.

---

## Further reading

- The Rust Book, ch. 4 — [Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html) (read all three sections)
- The Rust Book, ch. 8.1 — [Storing Lists of Values with Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html) (preview)
- Rust by Example — [Scoping rules](https://doc.rust-lang.org/rust-by-example/scope.html)
- std docs — [`str`](https://doc.rust-lang.org/std/primitive.str.html), [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
