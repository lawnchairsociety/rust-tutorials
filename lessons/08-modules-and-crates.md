# Lesson 08 — Packages, Crates & Modules

> Organize a growing program into files, control what's public, and understand Cargo's moving parts. Estimated time: 2 hours.

## What you'll learn

- How a **package**, a **crate** and a **module** differ
- Having a library crate (`lib.rs`) and a binary crate (`main.rs`) in one package
- Declaring modules inline and in files, including nested modules
- Privacy: `pub`, `pub(crate)`, `pub(super)`, private fields
- Paths: `crate::`, `super::`, `self::` and `use`
- Re-exporting with `pub use` to shape a clean public API
- How dependencies go into `Cargo.toml`, and what semantic versioning means

Your `taskr/src/main.rs` is getting long. This lesson is about cutting it into pieces that
each have one job.

---

## 1. Packages, crates, modules

| Term | What it is | In `taskr` |
|---|---|---|
| **Package** | A directory with a `Cargo.toml`. It builds one or more crates. | `taskr/` |
| **Crate** | One compilation unit: a library or an executable. | the `taskr` library + the `taskr` binary |
| **Crate root** | The file the compiler starts from. | `src/lib.rs` (library), `src/main.rs` (binary) |
| **Module** | A named namespace inside a crate, forming a tree. | `task`, `task_list`, `command`, `error` |

Cargo's conventions:

- `src/main.rs` → a binary crate with the same name as the package
- `src/lib.rs` → a library crate with the same name as the package (dashes become underscores)
- `src/bin/other.rs` → extra binaries

A package may contain **both** `lib.rs` and `main.rs`. The binary then uses the library
like any external crate: `use taskr::TaskList;`. Many Rust programs are built this way:
almost all the logic sits in the library, and `main.rs` is a thin layer that talks to the
user. The big payoff comes in lesson 10, because integration tests can only test a
**library**.

---

## 2. Declaring modules

### Inline

```rust
mod shapes {
    pub struct Square {
        pub side: f64,
    }

    impl Square {
        pub fn area(&self) -> f64 {
            self.side * self.side
        }
    }

    fn secret() {}           // private: only visible inside `shapes`
}

fn main() {
    let s = shapes::Square { side: 2.0 };
    println!("{}", s.area());
    // shapes::secret();     // error[E0603]: function `secret` is private
}
```

### In files

Usually each module gets its own file. You **declare** the module in its parent with
`mod name;` (a semicolon instead of a body), and the compiler looks for the code in:

```text
src/lib.rs                 mod task;        -> src/task.rs
src/task.rs                (module crate::task)
src/inventory.rs           mod item;        -> src/inventory/item.rs
src/inventory/item.rs      (module crate::inventory::item)
```

Two things trip people up:

1. **Creating `src/task.rs` does nothing by itself.** The compiler only finds files that
   are declared with `mod task;` somewhere in the module tree.
2. **Declare a module only once**, in its parent. Other files bring it into scope with
   `use`, not `mod`.

(You may also see the older layout `src/inventory/mod.rs` instead of `src/inventory.rs`.
Both work.)

---

## 3. Privacy

**Everything is private by default.** A private item is visible in the module where it's
defined and in that module's descendants. You open things up with:

| Visibility | Who can use it |
|---|---|
| *(nothing)* | this module and its children |
| `pub(super)` | the parent module (and its children) |
| `pub(crate)` | anywhere in this crate, but not to other crates |
| `pub` | everyone, provided they can reach the module too |

### Structs vs enums

- Making a struct `pub` does **not** make its fields public. Each field needs its own `pub`.
- A `pub enum` makes all its variants public automatically.

```rust
mod bank {
    pub struct Account {
        pub owner: String,
        balance: u64,          // private: outsiders can't change it directly
    }

    impl Account {
        pub fn new(owner: &str) -> Account {
            Account { owner: owner.to_string(), balance: 0 }
        }
        pub fn balance(&self) -> u64 {   // a read-only "getter"
            self.balance
        }
        pub fn deposit(&mut self, amount: u64) {
            self.balance += amount;
        }
    }
}

fn main() {
    let mut a = bank::Account::new("ann");
    a.deposit(10);
    println!("{} has {}", a.owner, a.balance());
    // a.balance = 1_000_000;   // error[E0616]: field `balance` of struct `Account` is private
    // bank::Account { owner: "x".into(), balance: 5 }; // error: can't build it either
}
```

Private fields enforce **invariants**. If `title` must never be empty, or `tags` must
always be lowercase, keep the fields private and check those rules in the constructor and
methods. Then no other code can break them.

---

## 4. Paths and `use`

```rust
mod garden {
    pub mod vegetables {
        pub fn carrot() -> &'static str { "carrot" }

        pub fn soup() -> String {
            // relative path from here:
            let a = carrot();
            let b = self::carrot();          // `self` = the current module
            let c = super::fruit();          // `super` = the parent module (garden)
            let d = crate::garden::fruit();  // absolute path from the crate root
            format!("{a} {b} {c} {d}")
        }
    }
    pub fn fruit() -> &'static str { "apple" }
}

use garden::vegetables;           // brings the *module* into scope
use garden::vegetables::carrot;   // brings a *function* into scope

fn main() {
    println!("{}", vegetables::soup());
    println!("{}", carrot());
}
```

(`&'static str` is the type of a string literal. You'll learn what `'static` means in
lesson 13.)

### `use` style conventions

- For **functions**, import the parent module and call `module::function()`. That makes
  it clear at the call site where the function comes from.
- For **structs, enums and traits**, import the item itself: `use std::collections::HashMap;`.
- Group imports: `use std::io::{self, Write};` imports `io` itself and `io::Write`.
- Rename on conflict: `use std::fmt::Result as FmtResult;`.
- `use` declarations apply to the module they're written in. A `use` in `lib.rs` does
  **not** affect `task.rs`. Each file has its own imports.

### Re-exports with `pub use`

Internal structure is for your convenience. The public API should be shaped for the
person using it:

```rust
// src/lib.rs
mod task;          // private module...
mod task_list;

pub use task::{Priority, Task};   // ...but its items are re-exported at the top level
pub use task_list::TaskList;
```

Now users write `use taskr::TaskList;` and don't need to know which file it's in. You can
reorganize the files later without breaking anyone.

---

## 5. The library + binary pattern

```text
taskr/
├── Cargo.toml
└── src/
    ├── lib.rs        <- crate root of the library `taskr`
    ├── task.rs
    ├── ...
    └── main.rs       <- crate root of the binary `taskr`
```

Inside `main.rs`, the library is a separate crate called `taskr`:

```rust
// src/main.rs
use taskr::{parse_command, Command, TaskList};   // NOT `mod task;` — the library owns those

fn main() {
    let mut list = TaskList::new();
    // ...
}
```

A common mistake is writing `mod task;` in **both** `lib.rs` and `main.rs`. That compiles
the file twice into two separate crates, and you get confusing errors like "expected
`taskr::Task`, found `Task`". Declare modules in `lib.rs` only.

Note that `main.rs` sees only what the library makes `pub`. That's a good check: if
`main.rs` needs to reach into a private field, maybe the library needs a method instead.

---

## 6. Dependencies and Cargo.toml

You won't add any dependencies this lesson, but you need to know how it works.

```toml
[package]
name = "taskr"
version = "0.1.0"
edition = "2024"

[dependencies]
rand = "0.9"          # means ^0.9 : any 0.9.x, never 0.10
```

- `cargo add rand` edits `Cargo.toml` for you. `cargo remove rand` undoes it.
- Crates come from [crates.io](https://crates.io). Documentation for every crate is on
  [docs.rs](https://docs.rs).
- **Semantic versioning**: for `MAJOR.MINOR.PATCH`, a bump to MAJOR may break your code.
  Before 1.0, a MINOR bump (0.**9** → 0.**10**) counts as breaking. A version like `"1.4"`
  in Cargo.toml means "1.4 or newer, but below 2.0".
- `Cargo.lock` records the exact versions chosen, so builds can be reproduced. Commit it
  for applications such as `taskr`.
- After adding a dependency, use it with `use rand::Rng;`. The crate's name works like a
  top-level path, the same way `std` does.

---

## Practice problems

This exercise crate is spread across several files, like a real project. Start with
`exercises/lesson-08/src/lib.rs`, which shows the module tree.

```bash
cd exercises && cargo test -p lesson-08
```

1. **`temperature.rs`**: implement both conversions.
2. **`util.rs`**: implement `normalize_name`. Note it's `pub(crate)` in a private module.
3. **Re-export**: read the `pub use` line in `lib.rs` and explain it in ANSWERS.md (Q7).
4. **`inventory/item.rs`**: implement `Item::new` and the getters. Use the full path to `normalize_name`.
5. **`inventory.rs`**: add a `use` line for `normalize_name` and implement `Inventory`'s
   methods. You'll need `Item::add_quantity`, which is `pub(super)`.
6. **`Inventory::names`**: return the sorted names.

Written questions in `exercises/lesson-08/ANSWERS.md`:

7. What does `pub use temperature::celsius_to_fahrenheit;` in `lib.rs` allow, and why might a library author want it?
8. The tests can call `crate::util::normalize_name`, even though `util` is not `pub`. Why? Would an integration test in `tests/` be able to call it?
9. `Item`'s fields are private. Why is that useful here? What would break if `name` were `pub`?
10. Predict the compiler error, and give two fixes:
    ```rust
    mod a {
        pub mod b {
            fn hidden() {}
        }
    }
    fn main() { a::b::hidden(); }
    ```
11. What's the difference between `mod task;` and `use task::Task;`?

---

## Homework — taskr step 08: library + binary

### Starting point

After lesson 07, `taskr` works fully but is probably all one `src/main.rs`: `Priority`,
`Task`, `TaskList`, `Command`, `parse_command`, `parse_priority`, `parse_id`, `ParseError`
and `TaskError` (both with `Display`), plus the REPL loop.

### Requirements

Restructure **without changing behavior**:

1. Create these files:
   ```text
   src/lib.rs        declares the modules; re-exports the public API with `pub use`
   src/task.rs       Priority, Task
   src/task_list.rs  TaskList
   src/command.rs    Command, parse_command, parse_priority, parse_id
   src/error.rs      ParseError, TaskError (and their Display impls)
   src/main.rs       only the REPL: reading input, calling the library, printing results
   ```
2. `main.rs` must not contain any `mod` declarations. It uses the library via
   `use taskr::...`.
3. **Minimal public surface.**
   - `Task`'s fields become **private**. Add the getters `main.rs` needs: `id()`,
     `title()`, `priority()`, `is_done()`, `tags()` (returning `&[String]`).
   - `TaskList`'s fields are private (they probably already are).
   - Helpers that only the library uses (e.g. a function that splits `add` arguments into
     title, tags and priority) should be private or `pub(crate)`.
4. Modules refer to each other with `use crate::...`, e.g. `task_list.rs` has
   `use crate::task::{Priority, Task};` and `use crate::error::TaskError;`.
5. Printing: decide whether `TaskList::print` belongs in the library or in `main.rs`, and
   explain your choice in a comment. (Hint: one of those places makes testing easier.)
6. `cargo build` produces **no warnings**. Unused imports and dead code will show up as
   you move things around, so clean them up.
7. Every command from lesson 07 still works exactly as before.

### Hints

- Do it in small steps and run `cargo build` after each one. First move `error.rs` (it
  depends on nothing), then `task.rs`, then `task_list.rs`, then `command.rs`.
- Typical `lib.rs`:
  ```rust
  mod command;
  mod error;
  mod task;
  mod task_list;

  pub use command::{parse_command, Command};
  pub use error::{ParseError, TaskError};
  pub use task::{Priority, Task};
  pub use task_list::TaskList;
  ```
- If you get `error[E0433]: failed to resolve: use of undeclared type`, a `use` line is
  missing in **that** file. Remember that imports apply per file.
- After making fields private, the compiler lists every place that touched them. Work
  through the list.

### Stretch goals

- Run `cargo doc --open` and browse your library's public API. Does it show exactly what
  `main.rs` needs, and nothing more?
- Try `cargo add rand`, use it for a hidden `taskr> random` command that picks a random
  pending task, then decide whether it's worth keeping the dependency.
  This is an exception to "without changing behavior" and requirement 7, so finish the
  restructure first.

---

## Submitting for review

1. `cd exercises && cargo test -p lesson-08` passes; `cd taskr && cargo build` is
   warning-free; the program behaves exactly as in lesson 07.
2. Commit on `work-08`:
   ```bash
   git add -A && git commit -m "Lesson 08: split taskr into library + binary"
   ```
3. Ask Claude: **"Please review lesson 08"**.

## Further reading

- The Rust Book, ch. 7 — [Managing Growing Projects with Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- The Rust Book, ch. 14.1–14.2 — [Cargo and crates.io](https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html)
- Rust by Example — [Modules](https://doc.rust-lang.org/rust-by-example/mod.html)
- The Cargo Book — [Specifying Dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)
