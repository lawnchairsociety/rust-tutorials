# Lesson 10 — Writing Tests

> Prove your code works, and keep it working as it changes. Estimated time: 2–3 hours.

## What you'll learn

- How `cargo test` finds and runs tests
- `#[test]`, `assert!`, `assert_eq!`, `assert_ne!` and custom failure messages
- Testing panics with `#[should_panic]`, and tests that return `Result`
- Unit tests in `#[cfg(test)] mod tests`, including testing private functions
- Integration tests in `tests/`, which only see your public API
- Documentation tests: examples in `///` comments that actually run
- Running a subset of tests, seeing output, and ignoring slow tests
- A test-driven development (TDD) loop

You've been running `cargo test` since lesson 1 on tests someone else wrote. Now you'll
write them yourself.

---

## 1. Anatomy of a test

A test is a function marked `#[test]`. If it returns normally, it passes. If it panics, it
fails.

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]           // compile this module only for `cargo test`
mod tests {
    use super::*;      // bring the parent module's items (add) into scope

    #[test]
    fn adds_two_numbers() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn adding_zero_changes_nothing() {
        assert_eq!(add(7, 0), 7, "x + 0 should be x");  // optional message
    }
}
```

```text
$ cargo test
running 2 tests
test tests::adding_zero_changes_nothing ... ok
test tests::adds_two_numbers ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Assertion macros

| Macro | Passes when | On failure prints |
|---|---|---|
| `assert!(cond)` | `cond` is `true` | the condition's source text |
| `assert_eq!(left, right)` | `left == right` | both values (needs `Debug` + `PartialEq`) |
| `assert_ne!(left, right)` | `left != right` | both values |

All three accept an optional `format!`-style message:
`assert!(list.is_empty(), "expected empty list, got {list:?}")`.

**Your types need `#[derive(Debug, PartialEq)]` to be used in `assert_eq!`.** That's why
lesson 09 asked you to derive them on `Command` and the error types.

### Why `#[cfg(test)]`?

`cfg(test)` is **conditional compilation**: the module only exists when building tests.
Test code and test-only helpers then don't end up in your real binary, and you don't get
"unused function" warnings for test helpers in a normal build.

---

## 2. Testing failure

### Expecting a panic

```rust
pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("division by zero");
    }
    a / b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "division by zero")]  // must panic, and the message must contain this text
    fn divide_by_zero_panics() {
        divide(1, 0);
    }
}
```

Without `expected`, *any* panic passes, including one from a bug elsewhere in the test.
Always add `expected` with part of the message.

### Expecting an `Err`

Most of your error handling uses `Result`, not panics. Test those values directly:

```rust
#[derive(Debug, PartialEq)]
pub enum ParseError { Empty, InvalidId(String) }

pub fn parse_id(s: &str) -> Result<u32, ParseError> {
    if s.is_empty() { return Err(ParseError::Empty); }
    s.parse().map_err(|_| ParseError::InvalidId(s.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_words() {
        assert_eq!(parse_id("abc"), Err(ParseError::InvalidId("abc".to_string())));
    }

    #[test]
    fn rejects_empty() {
        assert!(matches!(parse_id(""), Err(ParseError::Empty)));   // matches! = "does it fit this pattern?"
    }
}
```

`matches!(value, pattern)` returns `true` if the value fits the pattern. It's handy when
you only care about the variant and not the data inside it.

### Tests that return `Result`

A test may return `Result<(), E>`. Returning `Err` fails the test, which means you can use
`?` instead of `unwrap()` throughout:

```rust
#[test]
fn parses_valid_ids() -> Result<(), ParseError> {
    assert_eq!(parse_id("42")?, 42);
    assert_eq!(parse_id("7")?, 7);
    Ok(())
}
```

---

## 3. Unit tests vs integration tests

### Unit tests

Unit tests live **in the same file** as the code, in a `mod tests` child module. Rust's
privacy rule says a child can see its ancestors' private items, so **unit tests can test
private functions**:

```rust
fn internal_helper(x: u32) -> u32 { x * 2 }   // private

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn helper_doubles() {
        assert_eq!(internal_helper(4), 8);    // allowed
    }
}
```

### Integration tests

Integration tests live in a `tests/` directory **next to** `src/`. Each file there is
compiled as its own crate that depends on your library, the same way an outside user
would use it:

```text
taskr/
├── src/
│   ├── lib.rs
│   └── ...
└── tests/
    └── task_list.rs
```

```rust
// tests/task_list.rs
use taskr::TaskList;      // only `pub` items are reachable

#[test]
fn ids_start_at_one() {
    let mut list = TaskList::new();
    // ... use only the public API ...
}
```

- No `#[cfg(test)]` is needed in `tests/`. Cargo only compiles those files for testing.
- Integration tests **only work for library crates**. A `main.rs`-only package has
  nothing for them to `use`. This is why lesson 08 split `taskr` into a library and a
  binary.
- For helper code shared between test files, put it in `tests/common/mod.rs` and write
  `mod common;` in each test file.

Use unit tests for the details of how each piece works. Use integration tests to check
that the public pieces work together.

---

## 4. Documentation tests

Code blocks in `///` doc comments are compiled and run by `cargo test`. Examples in your
docs can't silently go out of date.

````rust
/// Returns the larger of two numbers.
///
/// # Examples
///
/// ```
/// assert_eq!(my_crate::max(3, 9), 9);
/// ```
pub fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}
````

- Doc tests run as an *external* user would, so call items with their full path
  (`my_crate::max`) or add a `use my_crate::max;` line inside the example.
- Lines starting with `# ` are compiled but hidden from the rendered docs. That's useful
  for setup code.
- Mark a block ` ```text ` for output that isn't Rust, or ` ```no_run ` for code that should
  compile but not run (e.g. it reads from stdin).
- Doc tests only run for library crates.

---

## 5. Running tests

```bash
cargo test                       # everything: unit, integration, doc
cargo test parse                 # only tests whose name contains "parse"
cargo test --lib                 # only unit tests
cargo test --test task_list      # only tests/task_list.rs
cargo test --doc                 # only doc tests
cargo test -- --nocapture        # show println! output even from passing tests
cargo test -- --test-threads=1   # run one at a time (tests run in parallel by default!)
```

Because tests run **in parallel**, they must not depend on each other or on shared state
like the same file on disk. Each test should set up everything it needs.

Mark slow tests with `#[ignore]` and run them on purpose with `cargo test -- --ignored`.

### What makes a good test

- **One behavior per test**, with a name that says what it checks:
  `complete_twice_returns_already_done` rather than `test3`.
- **Arrange, act, assert**: set up the state, do the one thing, check the result.
- **Test the edges**: empty input, the first and last valid values, the first invalid
  value, duplicates, Unicode.
- **Test the error paths**. They're the ones that break silently.
- Keep tests readable. A small helper like `fn list_with(titles: &[&str]) -> TaskList`
  removes repetition.

---

## 6. Test-driven development (TDD)

TDD is a short loop:

1. **Red**: write a test for behavior that doesn't exist yet. Run it and watch it fail.
   (If it passes, the test is wrong or the feature already exists.)
2. **Green**: write the *simplest* code that makes it pass.
3. **Refactor**: clean up while the tests stay green.

When the code doesn't compile yet, a "failing" test can just mean a compile error. Add a
stub with `todo!()` so the tests compile, then watch them fail at run time.

---

## Practice problems

The roles are reversed this time: `exercises/lesson-10/src/lib.rs` contains working
functions with **two planted bugs**, and the *tests* are the stubs.

```bash
cd exercises && cargo test -p lesson-10 --no-fail-fast
```

(`--no-fail-fast` keeps going after the unit tests fail, so you also see the integration
and doc test results.)

1–3. **Leap year tests.** One of them will catch a bug. Fix the code.
4. **`grade_boundaries`**: test every boundary. The second bug is here.
5. **`grade_rejects_scores_over_100`**: `#[should_panic(expected = ...)]`.
6. **`parse_duration_valid`**: a test returning `Result`, using `?`.
7. **`parse_duration_errors`**: check every error case.
8. **`unit_minutes_known_and_unknown`**: test a private function. Also
   **`divide_by_zero_panics`**.
9. **Doc test**: add an `# Examples` section to `Stack::peek`'s doc comment. Confirm it runs
   with `cargo test -p lesson-10 --doc`.
10. **TDD `to_roman`**: write the `roman_numerals` assertions first, see them fail, then
    implement `to_roman`.
11. **Integration tests**: fill in `exercises/lesson-10/tests/stack.rs`.

Written questions in `exercises/lesson-10/ANSWERS.md` (12–16).

---

## Homework — taskr step 10: a real test suite + TDD `edit`

### Starting point

After lesson 09, `taskr` is a library + binary with `Display` for `Task` and `Priority`,
`FromStr` for `Priority`, `Command` and `SortKey`, and `Default` for `TaskList`.
`Command`, `ParseError` and `TaskError` derive `Debug` and `PartialEq`. Commands are
`add`, `done`, `remove`, `show`, `list [#tag]`, `tags`, `stats`, `sort <id|priority>`,
`help` and `quit`.

### Requirements

1. **Unit tests in `command.rs`** (`#[cfg(test)] mod tests` at the bottom) covering:
   - every command parses to the expected `Command` (use `assert_eq!` on the whole value)
   - `add` with tags and each priority, in different positions, with mixed case
   - extra whitespace, e.g. `"   done   3  "`
   - **every** `ParseError` variant is produced by at least one input
2. **Unit tests in `task_list.rs`** covering:
   - ids start at 1 and increase by 1 per `add`
   - ids are **not reused** after `remove` (add 1, 2, 3, remove 3, add → id 4)
   - `complete` → `Ok`; completing again → `Err(TaskError::AlreadyDone(id))`
   - `complete`/`remove`/`get` on a missing id → `Err(TaskError::NotFound(id))`
   - `sort` by priority puts High first and breaks ties by id
3. **An integration test file `tests/task_list.rs`** that uses only `taskr::...` public
   items to run a realistic scenario: add several tasks, complete one, remove one, sort,
   and check the result through the public getters.
4. **At least one doc test**, e.g. on `TaskList::add` or `Priority::from_str`, showing
   typical use.
5. **At least 20 tests in total.** `cargo test` reports all three kinds: unit,
   integration, doc.
6. **TDD a new feature: `edit <id> <new text>`.** Follow red → green → refactor, and make
   separate commits so the history shows it:
   - **Red commit**: tests for parsing `edit` (e.g. `edit 2 Buy oat milk #errand !low` →
     `Command::Edit { id: 2, title: "Buy oat milk", priority: Some(Priority::Low), tags: vec!["errand"] }`;
     `edit 2` → `MissingArgument("title")`; `edit x foo` → `InvalidId`) and for
     `TaskList::edit` (changes title and tags; changes priority only if one was given;
     `NotFound` for a bad id). Add stub signatures with `todo!()` so it compiles. Commit
     with the tests failing.
   - **Green commit**: implement it. Reuse the same word-splitting helper that `add` uses.
     Wire it into `main.rs` and `help`.
   - **Refactor commit** (if needed).

### Example session

```text
taskr> add Buy milk #errand !high
Added task #1
taskr> edit 1 Buy oat milk #errand #shop
Edited task #1
taskr> show 1
#1 [ ] Buy oat milk (high) #errand #shop
taskr> edit 1 Buy oat milk !low
Edited task #1
taskr> show 1
#1 [ ] Buy oat milk (low)
taskr> edit 9 Something
Error: no task with id 9
```

```text
$ cargo test
running 18 tests          <- unit tests (src/)
...
running 3 tests           <- tests/task_list.rs
...
running 1 test            <- doc tests
...
```

### Hints

- A small helper inside `mod tests` cuts down repetition:
  `fn parse(s: &str) -> Command { s.parse().expect("should parse") }`. Using `expect` is
  fine **in tests**.
- If `Command::Add` holds a `Vec<String>`, comparing it in tests needs
  `vec!["errand".to_string()]`.
- To test that ids aren't reused, you need `remove` to work and then an `add` to return
  the next id. Because `add` returns the id, this is easy to assert.
- `cargo test edit` runs only the new tests while you work on the feature.

### Stretch goals

- If you did lesson 09's `Display for Command` stretch goal, write a **round-trip** test:
  for a list of inputs, `cmd.to_string().parse::<Command>() == Ok(cmd)`.
- Write a test helper `fn list_with(titles: &[&str]) -> TaskList` in `tests/common/mod.rs`
  and use it from the integration tests.

---

## Submitting for review

1. `cd exercises && cargo test -p lesson-10` passes (both bugs fixed). In `taskr`,
   `cargo test` shows unit, integration and doc tests all passing.
2. Commit on `work-10`. The `edit` feature should appear as separate red/green commits:
   ```bash
   git log --oneline   # e.g. "Add failing tests for edit", "Implement edit command"
   ```
3. Ask Claude: **"Please review lesson 10"**.

## Further reading

- The Rust Book, ch. 11 — [Writing Automated Tests](https://doc.rust-lang.org/book/ch11-00-testing.html)
- Rust by Example — [Testing](https://doc.rust-lang.org/rust-by-example/testing.html)
- The rustdoc book — [Documentation tests](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html)
- The Cargo Book — [`cargo test`](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
