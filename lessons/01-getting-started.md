# Lesson 01 — Getting Started

> Install-check, Cargo, your first program, variables, mutability, the basic types, and printing.
> Estimated time: 1.5–2 hours.

## What you'll learn

- How to create, build, and run a project with **Cargo**
- What `fn main`, `println!`, and macros are
- Variables, **immutability by default**, `mut`, constants, and **shadowing**
- Scalar types (integers, floats, `bool`, `char`) and compound types (tuples, arrays)
- Formatting text with `println!` and `format!`
- How to run this course's practice problems with `cargo test`

---

## 1. Your toolchain

Rust ships with three tools you'll use constantly:

| Tool | What it does |
|------|--------------|
| `rustup` | Installs and updates Rust toolchains (`rustup update`) |
| `rustc` | The compiler. You rarely call it directly. |
| `cargo` | Build tool + package manager. You'll use this for everything. |

Check that everything works:

```console
$ rustc --version
$ cargo --version
```

### Creating a project

Make your experiments in the **`playground/`** folder at the root of this repo. It's listed in
`.gitignore`, so anything you create there stays on your machine and never ends up in a commit.
Use it for every example in this course that you want to try out. The one exception is the
homework project, `taskr`, which goes in the repo root.

```console
$ mkdir -p playground && cd playground
$ cargo new hello
    Creating binary (application) `hello` package
$ cd hello
$ tree
.
├── Cargo.toml
└── src
    └── main.rs
```

`Cargo.toml` is the project manifest (name, version, edition, dependencies):

```toml
[package]
name = "hello"
version = "0.1.0"
edition = "2024"

[dependencies]
```

The **edition** (`2024`) selects a version of the language's surface rules. Editions are
opt-in and every edition can use crates written in any other edition.

### The commands you'll use most

| Command | Meaning |
|---------|---------|
| `cargo run` | Compile (if needed) and run the program |
| `cargo build` | Compile only; output goes to `target/debug/` |
| `cargo check` | Type-check without producing a binary — fastest feedback loop |
| `cargo test` | Compile and run tests |
| `cargo build --release` | Optimized build in `target/release/` |

---

## 2. Hello, world — line by line

```rust
fn main() {
    println!("Hello, world!");
}
```

- `fn main()` — every binary starts executing in `main`.
- `println!` — the `!` means this is a **macro**, not a function. Macros can do things
  functions can't, like accept a variable number of arguments and check your format
  string *at compile time*.
- Statements end with `;`.
- Indentation is 4 spaces by convention. Run `cargo fmt` to format automatically.

---

## 3. Variables and mutability

Variables are declared with `let` and are **immutable by default**:

```rust
fn main() {
    let x = 5;
    println!("x is {x}");
    // x = 6;   // ❌ error[E0384]: cannot assign twice to immutable variable `x`
}
```

The compiler error is very helpful — read it carefully:

```text
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         - first assignment to `x`
3 |     println!("x is {x}");
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut x = 5;
  |         +++
```

To allow change, opt in with `mut`:

```rust
fn main() {
    let mut count = 0;
    count += 1;
    count += 1;
    println!("count = {count}"); // count = 2
}
```

**Why immutable by default?** When you read `let total = ...` you *know* `total` will never
change below. This makes code easier to reason about, and the compiler uses it to catch bugs.

### Constants

```rust
const MAX_TASKS: u32 = 100;
const APP_NAME: &str = "taskr";

fn main() {
    println!("{APP_NAME} can hold {MAX_TASKS} tasks");
}
```

Constants:
- always need a **type annotation**,
- are named in `SCREAMING_SNAKE_CASE`,
- can be declared at the top level (outside any function),
- must be computable at compile time.

### Shadowing

You can declare a *new* variable with the same name as an old one. The new one
**shadows** the old:

```rust
fn main() {
    let spaces = "   ";        // a &str
    let spaces = spaces.len(); // now a usize — a *different* variable
    println!("{spaces}");      // 3

    let x = 5;
    {
        let x = x * 2;          // shadows only inside this block
        println!("inner x = {x}"); // 10
    }
    println!("outer x = {x}");  // 5
}
```

Shadowing vs `mut`: shadowing creates a new variable (and may change its type);
`mut` changes the value of the same variable (type stays fixed).

---

## 4. Scalar types

Rust is **statically typed**: every value has a type known at compile time. Usually the
compiler *infers* it, but you can annotate with `: Type`.

### Integers

| Signed | Unsigned | Size |
|--------|----------|------|
| `i8` | `u8` | 8 bits |
| `i16` | `u16` | 16 bits |
| `i32` | `u32` | 32 bits (default for integer literals is `i32`) |
| `i64` | `u64` | 64 bits |
| `i128` | `u128` | 128 bits |
| `isize` | `usize` | pointer-sized (used for indexing and lengths) |

```rust
fn main() {
    let a: u8 = 255;
    let b = 1_000_000;      // underscores for readability; inferred i32
    let c = 0xff;           // hex
    let d = 0b1010;         // binary
    let e = 42u64;          // suffix sets the type
    println!("{a} {b} {c} {d} {e}");
}
```

**Overflow:** in a debug build, `255u8 + 1` *panics* (crashes with a message). In a
release build it wraps around to 0. If you want specific behavior, say so explicitly:
`a.wrapping_add(1)`, `a.checked_add(1)`, `a.saturating_add(1)`.

### Floats, booleans, and characters

```rust
fn main() {
    let price = 9.99;        // f64 (the default float)
    let half: f32 = 0.5;
    let is_done = false;     // bool
    let letter = 'R';        // char — single quotes, 4 bytes, any Unicode scalar value
    let crab = '🦀';
    println!("{price} {half} {is_done} {letter} {crab}");
}
```

> **Tip:** need π or *e*? Use the exact constants in the standard library, such as
> `std::f64::consts::PI`, instead of typing `3.14159`. Clippy (the linter your editor runs)
> reports a hand-typed approximation like that as an error (`approx_constant`).

### Arithmetic and conversions

```rust
fn main() {
    let sum = 5 + 10;
    let quotient = 7 / 2;        // 3  — integer division truncates
    let remainder = 7 % 2;       // 1
    let precise = 7.0 / 2.0;     // 3.5

    // Rust never converts between numeric types implicitly:
    let small: u8 = 10;
    let big: u32 = 1000;
    // let total = small + big;  // ❌ mismatched types
    let total = small as u32 + big; // ✅ explicit cast with `as`

    // That includes floats and integers: no mixing them either.
    let count = 2;
    // let half = 7.0 / count;   // ❌ cannot divide `{float}` by `{integer}`
    let half = 7.0 / count as f64;  // ✅ 3.5: convert the integer to a float
    let whole = 3.9_f64 as i32;     // 3: float → int with `as` cuts off the fraction
    println!("{sum} {quotient} {remainder} {precise} {total} {half} {whole}");
}
```

Many languages quietly turn `7.0 / 2` into `7.0 / 2.0` for you. Rust doesn't. Both sides of an
arithmetic operator must have the **same** type, so a float and an integer can't be combined:

```text
error[E0277]: cannot divide `{float}` by `{integer}`
 --> src/main.rs:2:17
  |
2 |     let x = 7.0 / 2;
  |                 ^ no implementation for `{float} / {integer}`
```

The fix depends on what you have:

- **Two literals:** write both as floats, `7.0 / 2.0`, or both as integers, `7 / 2` (which gives `3`).
- **A variable:** convert it with `as`, e.g. `7.0 / count as f64`. `as` binds tighter than `/`, so
  this converts `count` before dividing.
- **Going the other way:** `some_float as i32` drops the fractional part (`3.9` becomes `3`). It
  doesn't round; use `.round()` first if you want rounding.

The same rule applies to `+`, `-`, `*` and `%`, and to mixing integer types like `u8` and `u32`.

---

## 5. Compound types: tuples and arrays

### Tuples — a fixed group of values of possibly different types

```rust
fn main() {
    let task: (&str, u8, bool) = ("Buy milk", 1, false);

    // Access by position:
    println!("title = {}", task.0);

    // Or destructure:
    let (title, priority, done) = task;
    println!("{title} / {priority} / {done}");
}
```

The empty tuple `()` is called **unit**. It's the "nothing" value — functions that don't
return anything actually return `()`.

### Arrays — a fixed number of values of the *same* type

```rust
fn main() {
    let days = ["Mon", "Tue", "Wed", "Thu", "Fri"];
    let zeros = [0; 8];             // [0, 0, 0, 0, 0, 0, 0, 0]
    let nums: [i32; 3] = [10, 20, 30];

    println!("first day: {}", days[0]);
    println!("length: {}", nums.len());
    println!("{:?}", zeros);        // {:?} = debug formatting, prints the whole array
}
```

The array's length is part of its type: `[i32; 3]` and `[i32; 4]` are different types.
Indexing past the end (`nums[10]`) panics at runtime — Rust always checks bounds.

Arrays can hold tuples, which is how we'll store tasks this lesson:

```rust
fn main() {
    let tasks = [("Buy milk", 1, true), ("Write report", 2, false)];
    let (title, _, _) = tasks[1];   // `_` ignores a value
    println!("{title}");
}
```

---

## 6. Printing and formatting

`println!` prints with a newline, `print!` without. `format!` builds a `String` instead of
printing.

```rust
fn main() {
    let name = "Ferris";
    let age = 7;

    println!("{name} is {age}");              // inline variable names
    println!("{} is {}", name, age);          // positional
    println!("{0} and {0} again", name);      // reuse by index
    println!("{:?}", (1, "two", 3.0));        // Debug format
    println!("{:#?}", (1, "two"));            // pretty Debug (multi-line)

    // Width, alignment, precision
    println!("[{:>6}]", "right");             // [ right]
    println!("[{:<6}]", "left");              // [left  ]
    println!("[{:^7}]", "mid");               // [  mid  ]
    println!("{:.2}", 1.23456);               // 1.23
    println!("{:03}", 7);                     // 007

    let line = format!("{name} ({age})");     // a String
    println!("{line}");
}
```

> **Note:** inline names like `{name}` only work with plain variable names, not expressions.
> `println!("{task.0}")` won't compile; use `println!("{}", task.0)`.

---

## 7. Comments

```rust
// A line comment.

/* A block comment. Rarely used. */

/// A doc comment — documents the item that follows (we'll use these in lesson 15).
fn main() {}
```

---

## 8. How the practice problems work

Each lesson has a small library crate in `exercises/lesson-NN/`. Open
`exercises/lesson-NN/src/lib.rs`: each problem is a function whose body is `todo!()`.
`todo!()` is a macro that compiles but panics when run, saying "not yet implemented".

Your job: replace each `todo!()` with a working implementation. At the bottom of the file
are **tests** that check your work. Run them:

```console
$ cd exercises
$ cargo test -p lesson-01
```

At first every test fails. As you solve problems, they turn green. You can run one test by
name: `cargo test -p lesson-01 swap`.

You don't need to know how to *write* functions or tests yet (lessons 02 and 10). For now,
just know:

```rust
pub fn add_one(x: i32) -> i32 {
    x + 1      // the last expression (no semicolon!) is the return value
}
```

Written questions go in `exercises/lesson-NN/ANSWERS.md`.

> Unused-variable warnings while functions are still `todo!()` are expected — they go away
> as you implement things.

---

## Practice problems

All in `exercises/lesson-01/src/lib.rs` unless noted.

1. **`celsius_to_fahrenheit(c: f64) -> f64`** — `F = C × 9/5 + 32`. Watch out: `9 / 5` is
   integer division if you write integer literals!
2. **`seconds_in_days(days: u64) -> u64`** — how many seconds in `days` days? Use constants
   for the number of seconds in a minute/hour/day.
3. **`swap_pair(pair: (i32, i32)) -> (i32, i32)`** — return the tuple with elements swapped.
   Try destructuring.
4. **`sum_of_five(nums: [i32; 5]) -> i32`** — add up the five elements using indexing (loops
   come next lesson).
5. **`describe(name: &str, age: u32) -> String`** — return e.g. `"Alice is 30 years old"`.
   Use `format!`.
6. **`quotient_and_remainder(a: i32, b: i32) -> (i32, i32)`** — return both results of
   integer division as a tuple.
7. **`average(a: f64, b: f64, c: f64) -> f64`**
8. **`padded_score(name: &str, score: u32) -> String`** — name left-aligned in 8 columns,
   then score right-aligned in 4 columns, wrapped in pipes: `"|Ann     |  42|"`.
9. **`to_u8_saturating(x: u32) -> u8`** — convert to `u8`, but values above 255 become 255.
   Hint: `x as u8` alone isn't enough. When a value doesn't fit, `as` keeps only the lowest
   8 bits, so `256 as u8` is `0` and `1000 as u8` is `232`. Instead, cap `x` *while it's still a
   `u32`*, then convert. Every number type has a `.min()` method that returns the smaller of
   two values of the same type: `5000_u32.min(100)` is `100`, and `40_u32.min(100)` is `40`.

Written questions (`exercises/lesson-01/ANSWERS.md`):

- **Q1.** What does `cargo check` do, and why is it faster than `cargo build`?
- **Q2.** This doesn't compile. Why, and what are *two* different ways to fix it?
  ```rust
  let total = 0;
  total = total + 5;
  ```
- **Q3.** Predict the output, then run it to check:
  ```rust
  let x = 2;
  let x = x * 3;
  {
      let x = "six";
      println!("{x}");
  }
  println!("{x}");
  ```
- **Q4.** What happens when you run `let a: u8 = 255; let b = a + 1;` in a debug build? In a
  release build? (Hint: the compiler may even catch the literal case — try
  `let b = a + std::hint::black_box(1);` to see the runtime behavior.)

---

## Homework — taskr step 01

Over this course you'll build **taskr**, a command-line task tracker. Each lesson's
homework adds to the previous one, so by lesson 15 you'll have a real, tested,
persistent, multi-threaded CLI app.

### Starting point

Nothing yet — you're creating the project.

### Requirements

1. From the **repository root**, run `cargo new taskr`. (Your project lives at `taskr/`.)
2. Define two constants at the top of `taskr/src/main.rs`:
   - `APP_NAME: &str = "taskr"`
   - `VERSION: &str = "0.1.0"`
3. `main` prints a banner using the constants, e.g. `=== taskr v0.1.0 ===`.
4. Store three hardcoded tasks as an **array of tuples** `(&str, u8, bool)` meaning
   `(title, priority, done)`. Priority 1 is highest, 3 is lowest.
5. Print each task on its own line in this format (done tasks get `[x]`, pending get `[ ]`):
   ```text
   1. [x] (P1) Buy milk
   ```
   You don't know loops yet — it's fine to print each one with its own `println!`. For the
   `[x]`/`[ ]` mark you may use `if done { "x" } else { " " }` (a preview of lesson 02).
6. Print a summary line: `3 tasks, 1 done`. Compute the "done" count from the data (not
   hardcoded!). Hint: `done as u32` converts `true`/`false` to `1`/`0`.
7. Use **shadowing** at least once in a meaningful way (e.g., `let done_count = ...;` then
   `let done_count = format!(...)`), and **no unnecessary `mut`**.

### Example output

```console
$ cd taskr
$ cargo run
=== taskr v0.1.0 ===
1. [x] (P1) Buy milk
2. [ ] (P2) Write report
3. [ ] (P3) Call mom
3 tasks, 1 done
```

### Hints

- `let (title, priority, done) = tasks[0];` destructures one task.
- `cargo run` warns about unused things — read the warnings; they're often helpful.
- Run `cargo fmt` before committing.

### Stretch goals

- Align titles in a column using width formatting (`{:<20}`).
- Print the percentage of tasks done with one decimal place: `33.3% done`.

---

## Submitting for review

1. Make sure `cargo test -p lesson-01` passes (from `exercises/`) and `cargo run` works in
   `taskr/`.
2. Commit on your `work-01` branch:
   ```console
   $ git add -A
   $ git commit -m "Lesson 01 complete"
   ```
3. Ask Claude: **"Please review lesson 01"**. Claude will look at `exercises/lesson-01`,
   your `ANSWERS.md`, and your new `taskr/` project.

---

## Further reading

- The Rust Book, ch. 1 — [Getting Started](https://doc.rust-lang.org/book/ch01-00-getting-started.html)
- The Rust Book, ch. 3.1–3.2 — [Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html), [Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- Rust by Example — [Formatted print](https://doc.rust-lang.org/rust-by-example/hello/print.html), [Primitives](https://doc.rust-lang.org/rust-by-example/primitives.html)
- std docs — [`std::fmt`](https://doc.rust-lang.org/std/fmt/index.html) (all the formatting options)
