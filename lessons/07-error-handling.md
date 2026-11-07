# Lesson 07 — Error Handling

> Make failure a normal, typed value instead of a crash. Estimated time: 2–3 hours.

## What you'll learn

- Recoverable errors (`Result`) vs unrecoverable bugs (`panic!`)
- `unwrap` and `expect`, and when they're acceptable
- Handling `Result` with `match`, `if let` and helper methods
- The `?` operator for passing errors up to the caller
- Converting between `Option` and `Result`, and between error types with `map_err`
- Designing your own error enums and giving them readable messages with `Display`

Rust has no exceptions. A function that can fail says so **in its return type**, and the
compiler makes sure the caller deals with that possibility.

---

## 1. Two kinds of failure

**Unrecoverable: `panic!`.** Something happened that should be impossible if the program
is correct, such as an index past the end of an array or a broken invariant. The program
prints a message and the thread unwinds.

```rust
fn main() {
    let v = vec![1, 2, 3];
    let x = v[99]; // thread 'main' panicked at src/main.rs:3:14: index out of bounds
    println!("{x}");
}
```

**Recoverable: `Result<T, E>`.** Something might legitimately go wrong, like a user typing
`"abc"` where you wanted a number, or a file that doesn't exist. The caller decides what to
do about it.

```rust
enum Result<T, E> {   // already defined in the standard library
    Ok(T),
    Err(E),
}
```

A good rule of thumb: **input from the outside world should never be able to cause a
panic.** In `taskr`, nothing the user types should crash the program.

---

## 2. Handling a `Result`

`str::parse` returns a `Result`, so it's a good one to experiment with:

```rust
fn main() {
    let input = "42";

    // 1. match — the most explicit
    match input.parse::<u32>() {
        Ok(n) => println!("got {n}"),
        Err(e) => println!("bad number: {e}"), // e is a std::num::ParseIntError
    }

    // 2. if let — when you only care about one case
    if let Ok(n) = "7".parse::<u32>() {
        println!("seven = {n}");
    }

    // 3. let-else — bail out early if it doesn't match
    let Ok(n) = "13".parse::<u32>() else {
        println!("not a number");
        return;
    };
    println!("{n}");

    // 4. helpers
    let a = "x".parse::<u32>().unwrap_or(0);        // default on error
    let b = "5".parse::<u32>().is_ok();             // true
    let c = "5".parse::<u32>().ok();                // Result -> Option: Some(5)
    println!("{a} {b} {c:?}");
}
```

### `unwrap` and `expect`

```rust
let n: u32 = "42".parse().unwrap();                       // panics on Err
let n: u32 = "42".parse().expect("hard-coded literal is valid"); // panics with your message
```

These are fine in quick experiments, in tests, and when you **know** the value can't be an
error (and `expect` lets you write down *why* you know). They are not fine for user input.

---

## 3. The `?` operator

Here is a function that has to handle two failure points:

```rust
use std::num::ParseIntError;

fn add_strings_verbose(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x = match a.parse::<i32>() {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    let y = match b.parse::<i32>() {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    Ok(x + y)
}
```

That "unwrap the `Ok` or return the `Err`" pattern is so common that it gets its own
operator:

```rust
use std::num::ParseIntError;

fn add_strings(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x = a.parse::<i32>()?;
    let y = b.parse::<i32>()?;
    Ok(x + y)
}

fn main() {
    println!("{:?}", add_strings("2", "3"));   // Ok(5)
    println!("{:?}", add_strings("2", "x"));   // Err(ParseIntError { kind: InvalidDigit })
}
```

Rules for `?`:

- You can only use it in a function that itself returns a `Result` (or an `Option`; then
  `?` on an `Option` returns `None` early).
- The error type must match the function's error type, or be convertible into it via the
  `From` trait. More on `From` in lesson 9; for now, convert by hand with `map_err`.

---

## 4. Converting errors and options

### `map_err`: change the error type

```rust
fn parse_id(s: &str) -> Result<u32, String> {
    s.parse::<u32>().map_err(|_| format!("invalid id: {s}"))
}
```

The `|_| ...` part is a **closure**, a small anonymous function. Here it takes the original
error (which we ignore with `_`) and returns the new one. Closures get a full treatment in
lesson 11; for now, read `|e| expr` as "a function of `e` that returns `expr`".

### `Option` ⇄ `Result`

```rust
fn main() {
    let words = vec!["alpha", "beta"];

    // Option -> Result: provide the error for the None case
    let first: Result<&str, String> = words.first().copied().ok_or("empty list".to_string());

    // Result -> Option: throw the error away
    let n: Option<u32> = "12".parse().ok();

    println!("{first:?} {n:?}");
}
```

`ok_or(err)` builds the error value every time, even when it isn't used.
`ok_or_else(|| make_err())` only builds it when needed, which helps when the error is
expensive to create.

---

## 5. Your own error types

A `String` error is easy to make, but the caller can't **match** on it in any useful way.
An enum gives each kind of failure its own name, and can carry data about it:

```rust
#[derive(Debug, PartialEq)]
enum ParseError {
    Empty,
    UnknownCommand(String),
    InvalidId(String),
}

fn parse_id(s: &str) -> Result<u32, ParseError> {
    s.parse::<u32>().map_err(|_| ParseError::InvalidId(s.to_string()))
}

fn parse(input: &str) -> Result<(String, u32), ParseError> {
    let input = input.trim();
    if input.is_empty() {
        return Err(ParseError::Empty);
    }
    let (cmd, arg) = input.split_once(' ').unwrap_or((input, ""));
    match cmd {
        "done" => Ok(("done".to_string(), parse_id(arg)?)),
        other => Err(ParseError::UnknownCommand(other.to_string())),
    }
}

fn main() {
    println!("{:?}", parse("done 3"));   // Ok(("done", 3))
    println!("{:?}", parse("done x"));   // Err(InvalidId("x"))
    println!("{:?}", parse("dance"));    // Err(UnknownCommand("dance"))
}
```

### Readable messages with `Display`

`{:?}` (Debug) output is for programmers. To print an error **for users** with `{}`, the
type needs to implement the `Display` trait. Traits are lesson 9's topic, so for now treat
this as a recipe to copy: one `match` arm per variant, each one `write!`-ing a message.

```rust
use std::fmt;

#[derive(Debug)]
enum TaskError {
    NotFound(u32),
    AlreadyDone(u32),
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TaskError::NotFound(id) => write!(f, "no task with id {id}"),
            TaskError::AlreadyDone(id) => write!(f, "task {id} is already done"),
        }
    }
}

fn main() {
    let e = TaskError::NotFound(9);
    println!("Error: {e}");             // Error: no task with id 9
    let msg: String = e.to_string();    // Display also gives you .to_string() for free
    println!("{msg}");
}
```

`write!` works like `format!`, except it writes into the formatter `f`.

### Designing good error enums

- Use one variant per thing the **caller** might want to handle differently.
- Carry the data needed to explain the problem (the bad id, the unknown word).
- Derive `Debug` (and `PartialEq` too, so tests can `assert_eq!` on errors).
- Keep error messages lowercase without a trailing period. That's the Rust convention,
  and it reads well after a prefix like `Error: `.

---

## 6. Where to handle errors

Errors should travel **up** to the code that knows what to do about them. In a REPL, that's
the main loop: it prints the message and asks for the next command.

```rust
loop {
    // ... read `line` ...
    match parse_command(&line) {
        Ok(cmd) => { /* run it; that may also produce an error to print */ }
        Err(e) => println!("Error: {e}"),
    }
}
```

The parsing and task-list code should **return** errors and never print them. That keeps
them easy to test (lesson 10) and easy to reuse.

---

## Practice problems

Code problems live in `exercises/lesson-07/src/lib.rs`. **No `unwrap`, `expect` or
`panic!` allowed in your solutions.**

```bash
cd exercises && cargo test -p lesson-07
```

1. **`parse_age`**: parse with validation and `String` errors.
2. **`sum_strings`**: use `?` inside a loop.
3. **`first_even`**: turn an `Option` into a `Result` with `ok_or`.
4. **`safe_divide`**: return a custom `MathError` enum and use `checked_div`.
5. **`impl Display for PointError`**: write the `fmt` method.
6. **`parse_point`**: use `split_once`, `ok_or`, `map_err` and `?`, with no nested matches.
7. **`Account::withdraw`**: return an error enum with struct-like variant data.
8. **`transfer`**: pass errors along with `?` and leave the accounts unchanged on failure.

Written questions in `exercises/lesson-07/ANSWERS.md`:

9. When is `panic!` the right choice and when is `Result`? Give one example of each from `taskr`.
10. What exactly does `x?` expand to (roughly) when `x: Result<T, E>`?
11. Why is `enum ParseError` better than `String` as an error type for `parse_command`?
12. Why doesn't this compile, and how would you fix it?
    ```rust
    fn main() {
        let n: i32 = "5".parse()?;
        println!("{n}");
    }
    ```

---

## Homework — taskr step 07: real errors

### Starting point

After lesson 06, `taskr` supports `add` (with `#tags` and `!priority`), `done`, `remove`,
`show`, `list` (optionally `list #tag`), `tags`, `stats`, `help` and `quit`.
`parse_command` returns a `Command`, and anything it doesn't understand becomes
`Command::Unknown(String)`. `TaskList` methods return `bool` or `Option`.

### Requirements

1. **`ParseError`.** Create
   ```rust
   #[derive(Debug, PartialEq)]
   enum ParseError {
       Empty,                    // blank line
       UnknownCommand(String),   // the unrecognised word
       MissingArgument(String),  // name of what's missing, e.g. "id" or "title"
       InvalidId(String),        // the text that wasn't a valid id
       InvalidPriority(String),  // e.g. "!urgent"
   }
   ```
   and implement `Display` for it with clear messages.
2. **`parse_command` returns `Result<Command, ParseError>`.** Remove `Command::Unknown`.
   - `done`, `remove` and `show` with no argument → `MissingArgument("id")`. A
     non-numeric argument (or `0`, if you treat ids as starting at 1) → `InvalidId(..)`.
   - `add` with an empty title → `MissingArgument("title")`.
   - `add Buy milk !urgent` → `InvalidPriority("!urgent")`. Before, it probably became
     part of the title or got ignored.
   - Write `fn parse_id(s: &str) -> Result<u32, ParseError>` and use `?` with it.
   - Make `parse_priority` return `Result<Priority, ParseError>`, or keep it as an
     `Option` and convert with `ok_or_else` at the call site.
3. **`TaskError`.**
   ```rust
   #[derive(Debug, PartialEq)]
   enum TaskError {
       NotFound(u32),
       AlreadyDone(u32),
   }
   ```
   with `Display`. Change the `TaskList` methods:
   - `complete(&mut self, id) -> Result<(), TaskError>`: completing a done task gives `AlreadyDone`
   - `remove(&mut self, id) -> Result<Task, TaskError>`
   - add `get(&self, id) -> Result<&Task, TaskError>` (build it on `find` with `ok_or`) and use it for `show`
4. **The main loop handles everything.** Print parse and task errors as `Error: <message>`.
   A blank line (`ParseError::Empty`) just shows the prompt again with no error message.
5. **No panics on user input.** Search your code for `unwrap()`, `expect(` and `[` indexing
   on user-provided numbers. Each one must be impossible to trigger from the keyboard, or
   be replaced.

### Example session

```text
taskr> add Buy milk #errand !high
Added task #1
taskr> done
Error: missing argument: id
taskr> done abc
Error: invalid id: abc
taskr> done 7
Error: no task with id 7
taskr> done 1
Completed task #1
taskr> done 1
Error: task 1 is already done
taskr> add !high
Error: missing argument: title
taskr> add Call mom !urgent
Error: invalid priority: !urgent (use !low, !med or !high)
taskr> dance
Error: unknown command: dance (type 'help')
taskr>
taskr> quit
```

### Hints

- `input.split_once(' ')` splits off the command word. If there is no space, the whole
  input is the command and the argument is empty: `.unwrap_or((input, ""))`. That
  `unwrap_or` is safe because it never panics.
- In the main loop, nest it: `match parse_command(&line) { Ok(cmd) => match cmd { ... }, Err(ParseError::Empty) => {}, Err(e) => println!("Error: {e}") }`.
- For a command like `done`, you can write `match list.complete(id) { Ok(()) => ..., Err(e) => println!("Error: {e}") }`.
  If several commands repeat this, consider a function `fn run(list: &mut TaskList, cmd: Command) -> Result<String, TaskError>`
  that returns the message to print, so the loop handles errors in **one** place.

### Stretch goals

- Write that `run` function returning `Result<String, TaskError>` so the main loop only
  has a single error-printing arm.
- Make unknown-command errors suggest the closest command (e.g. `lsit` → "did you mean
  `list`?") by comparing letters.

---

## Submitting for review

1. `cd exercises && cargo test -p lesson-07` passes; `taskr` builds and survives every bad
   input you can think of.
2. Commit on `work-07`:
   ```bash
   git add -A && git commit -m "Lesson 07: error handling"
   ```
3. Ask Claude: **"Please review lesson 07"**.

## Further reading

- The Rust Book, ch. 9 — [Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- Rust by Example — [Error handling](https://doc.rust-lang.org/rust-by-example/error.html)
- std docs — [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html), [`Option`](https://doc.rust-lang.org/std/option/enum.Option.html), [`std::fmt::Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html)
