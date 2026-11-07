# Lesson 02 — Functions & Control Flow

> Functions, expressions vs. statements, `if`, loops, ranges, a first look at `match`,
> and reading user input.
> Estimated time: 2 hours.

## What you'll learn

- How to define functions with parameters and return values
- The difference between **expressions** and **statements** (this matters a lot in Rust)
- `if`/`else` — including `if` as an expression
- `loop`, `while`, `for`, ranges, `break`/`continue`, loop labels, and values from `loop`
- A first look at `match` on numbers and strings
- Reading a line from the terminal

---

## 1. Functions

```rust
fn main() {
    greet("Ferris");
    let total = add(2, 3);
    println!("2 + 3 = {total}");
}

fn greet(name: &str) {
    println!("Hello, {name}!");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

- Function names use `snake_case`.
- Every parameter **must** have a type. Rust never infers parameter types — function
  signatures are contracts.
- `-> i32` declares the return type. No arrow means the function returns `()` (unit).
- Functions can be defined in any order; `main` can call `add` even though it's below.

### Returning values

The **last expression** in a function body is its return value. Note: *no semicolon*.

```rust
fn square(x: i32) -> i32 {
    x * x          // returned
}

fn absolute(x: i32) -> i32 {
    if x < 0 {
        return -x; // early return with the `return` keyword
    }
    x
}
```

---

## 2. Expressions vs. statements

This is the single most important idea in this lesson.

- An **expression** evaluates to a value: `5`, `a + b`, `square(3)`, `if c { 1 } else { 2 }`,
  and even a block `{ ... }`.
- A **statement** performs an action and has no value: `let x = 5;`, or any expression
  followed by `;`.

Adding a semicolon turns an expression into a statement and **throws its value away**:

```rust
fn plus_one(x: i32) -> i32 {
    x + 1;   // ❌ the `;` discards the value; the function now "returns" ()
}
```

```text
error[E0308]: mismatched types
 --> src/main.rs:1:24
  |
1 | fn plus_one(x: i32) -> i32 {
  |    --------            ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
2 |     x + 1;
  |          - help: remove this semicolon to return this value
```

Blocks are expressions too:

```rust
fn main() {
    let y = {
        let x = 3;
        x * 2 + 1      // value of the block
    };
    println!("{y}");   // 7
}
```

---

## 3. `if` / `else`

```rust
fn main() {
    let n = 7;

    if n % 2 == 0 {
        println!("even");
    } else if n % 3 == 0 {
        println!("divisible by 3");
    } else {
        println!("something else");
    }
}
```

- The condition must be a `bool`. `if n { ... }` with an integer is an error — Rust has no
  "truthy" values.
- No parentheses needed around the condition; braces are always required.

### `if` is an expression

```rust
fn main() {
    let done = true;
    let mark = if done { "x" } else { " " };
    println!("[{mark}]");
}
```

Both branches must produce the **same type**: `if c { 1 } else { "one" }` doesn't compile.

---

## 4. Loops

### `loop` — repeat forever (until `break`)

```rust
fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // `break` can carry a value out of the loop
        }
    };
    println!("{result}"); // 20
}
```

### `while` — repeat while a condition holds

```rust
fn main() {
    let mut n = 3;
    while n > 0 {
        println!("{n}...");
        n -= 1;
    }
    println!("liftoff!");
}
```

### `for` — iterate over a collection or range

```rust
fn main() {
    let tasks = ["Buy milk", "Write report", "Call mom"];
    for task in tasks {
        println!("- {task}");
    }

    for i in 0..3 {        // 0, 1, 2      (end exclusive)
        print!("{i} ");
    }
    for i in 1..=3 {       // 1, 2, 3      (end inclusive)
        print!("{i} ");
    }
    for i in (1..=3).rev() { // 3, 2, 1
        print!("{i} ");
    }
    println!();
}
```

`for` is the loop you'll use most. It's safer than indexing with `while`, because you can't
go out of bounds.

### Getting the index too: `enumerate`

```rust
fn main() {
    let tasks = ["Buy milk", "Write report"];
    for (i, task) in tasks.iter().enumerate() {
        println!("{}. {task}", i + 1);  // i starts at 0
    }
}
```

(We'll explain `.iter()` properly in lesson 11. For now, treat `.iter().enumerate()` as
"give me `(index, item)` pairs".)

### `break`, `continue`, and labels

```rust
fn main() {
    for n in 1..=10 {
        if n % 2 == 0 {
            continue;         // skip to the next iteration
        }
        if n > 7 {
            break;            // leave the loop
        }
        print!("{n} ");       // 1 3 5 7
    }
    println!();

    'outer: for x in 0..5 {
        for y in 0..5 {
            if x * y == 6 {
                println!("found {x} * {y} = 6");
                break 'outer; // break out of the *outer* loop
            }
        }
    }
}
```

---

## 5. A first look at `match`

`match` compares a value against **patterns** and runs the first arm that matches. It is
much more powerful than a `switch` — we'll go deep in lesson 05.

```rust
fn describe(n: u32) -> &'static str {
    match n {
        0 => "zero",
        1 | 2 | 3 => "a few",       // multiple patterns
        4..=9 => "several",          // inclusive range
        _ => "lots",                 // `_` matches anything else
    }
}

fn main() {
    for n in [0, 2, 5, 100] {
        println!("{n}: {}", describe(n));
    }
}
```

> Don't worry about `&'static str` yet — it means "a string literal baked into the
> program". It's explained in lesson 13.

`match` must be **exhaustive** — every possible value must be covered. Delete the `_` arm
above and the compiler tells you which values you've missed.

You can match on string slices too, which is perfect for command parsing:

```rust
fn main() {
    let command = "help";
    match command {
        "list" => println!("listing..."),
        "help" => println!("helping..."),
        "quit" | "exit" => println!("bye"),
        other => println!("Unknown command: {other}"), // a name binds the value
    }
}
```

`match` is an expression too, so it can produce a value:

```rust
fn main() {
    let priority = 2;
    let label = match priority {
        1 => "high",
        2 => "medium",
        _ => "low",
    };
    println!("{label}");
}
```

---

## 6. Reading input from the terminal (recipe)

You'll understand every piece of this by lesson 07. For now, use it as a recipe:

```rust
use std::io::{self, Write};

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();    // print! doesn't flush; make the prompt appear

        let mut input = String::new();
        let bytes_read = io::stdin().read_line(&mut input).unwrap();
        if bytes_read == 0 {
            // 0 bytes means end-of-file (Ctrl-D on Linux/macOS, Ctrl-Z then Enter on Windows)
            println!();
            break;
        }

        let input = input.trim();  // remove the trailing newline (and surrounding spaces)
        match input {
            "quit" => break,
            "" => continue,        // ignore empty lines
            other => println!("you typed: {other}"),
        }
    }
}
```

Things to notice:
- `use std::io::{self, Write};` brings the `io` module and the `Write` trait (needed for
  `.flush()`) into scope.
- `String::new()` creates an empty, growable string; `read_line` appends to it.
- `.trim()` returns a view of the string without surrounding whitespace. We *shadow*
  `input` with it.
- `.unwrap()` means "crash if this failed". Reading stdin basically never fails, so this is
  fine for now. Lesson 07 covers proper error handling.

You can test a REPL without typing by piping input:

```console
$ printf 'help\nlist\nquit\n' | cargo run
```

---

## Practice problems

All in `exercises/lesson-02/src/lib.rs` unless noted. Run: `cargo test -p lesson-02`.

1. **`fizzbuzz(n: u32) -> String`** — `"Fizz"` for multiples of 3, `"Buzz"` for multiples of
   5, `"FizzBuzz"` for both, otherwise the number as a string (`n.to_string()`).
2. **`factorial(n: u64) -> u64`** — use a loop (not recursion). `factorial(0)` is `1`.
3. **`is_prime(n: u32) -> bool`** — `0` and `1` are not prime.
4. **`sum_of_multiples(limit: u32) -> u32`** — sum of all numbers *below* `limit` that are
   multiples of 3 or 5.
5. **`collatz_steps(n: u64) -> u32`** — how many steps to reach 1, where each step does:
   even → `n / 2`, odd → `3n + 1`. `collatz_steps(1)` is `0`.
6. **`letter_grade(score: u32) -> char`** — use `match` with ranges: 90–100 `'A'`, 80–89 `'B'`,
   70–79 `'C'`, 60–69 `'D'`, 0–59 `'F'`, anything above 100 `'?'`.
7. **`day_name(n: u8) -> String`** — 1 → `"Monday"` … 7 → `"Sunday"`, anything else →
   `"Invalid"`. Use `match`. (Converting a literal to a `String`: `"Monday".to_string()`.)
8. **`count_digits(n: u64) -> u32`** — number of decimal digits; `count_digits(0)` is `1`.
   Use `while` (or `loop`), not string conversion.
9. **`max_of_array(nums: [i32; 6]) -> i32`** — the largest element, using a `for` loop.

Written questions (`exercises/lesson-02/ANSWERS.md`):

- **Q1.** Explain why this function doesn't compile and how to fix it with a one-character
  change:
  ```rust
  fn double(x: i32) -> i32 {
      x * 2;
  }
  ```
- **Q2.** What value does `result` hold?
  ```rust
  let mut i = 1;
  let result = loop {
      i *= 3;
      if i > 50 { break i - 1; }
  };
  ```
- **Q3.** Why doesn't `let x = if true { 5 } else { "five" };` compile?
- **Q4.** When would you choose `loop` over `while true`? (Hint: try returning a value from
  each, and see what the compiler says about `while true`.)

---

## Homework — taskr step 02

### Starting point

From lesson 01: `taskr/src/main.rs` prints a banner from constants, three hardcoded
tasks stored as an array of `(&str, u8, bool)` tuples, and a summary line.

### Requirements

1. Write **`fn format_task(n: usize, title: &str, priority: u8, done: bool) -> String`** that
   returns one line like `1. [x] (P1) Buy milk`. Use `if` as an expression for the mark.
2. Write **`fn print_tasks(tasks: [(&str, u8, bool); 3])`** that prints every task using a
   `for` loop and `format_task`, followed by the summary line (`3 tasks, 1 done`). Count
   the done tasks in the loop.
3. Write **`fn print_help()`** that lists the available commands.
4. Turn `main` into an **interactive REPL**:
   - print the banner once,
   - then repeatedly show the prompt `taskr> ` and read a line,
   - `list` → print the tasks,
   - `help` → print the help,
   - `quit` → print `Bye!` and exit,
   - an empty line → just show the prompt again,
   - end-of-file (Ctrl-D) → exit like `quit`,
   - anything else → `Unknown command: <what they typed> (type 'help')`.
5. Use `match` on the trimmed input to dispatch commands.
6. Tasks are still the hardcoded array of tuples.

### Example session

```console
$ cargo run
=== taskr v0.1.0 ===
Type 'help' for a list of commands.
taskr> help
Commands:
  list   Show all tasks
  help   Show this help
  quit   Exit taskr
taskr> list
1. [x] (P1) Buy milk
2. [ ] (P2) Write report
3. [ ] (P3) Call mom
3 tasks, 1 done
taskr> dance
Unknown command: dance (type 'help')
taskr> quit
Bye!
```

### Hints

- To loop over an array and get positions: `for (i, (title, priority, done)) in tasks.iter().enumerate()`.
  Because `.iter()` gives *references* to the tuples, the fields come out as references
  too: `title: &&str`, `priority: &u8`, `done: &bool`. You can pass `*priority` and
  `*done` (the `*` "follows" the reference; lesson 03 explains). Alternatively, use a
  plain `for task in tasks` with a separate counter.
- Keep `main` short — let functions do the work.
- Test without typing: `printf 'list\nfoo\nquit\n' | cargo run`.

### Stretch goals

- Accept `exit` and `q` as synonyms for `quit`, and `?` as a synonym for `help`.
- Make commands case-insensitive (look up `to_lowercase` — note it returns a `String`, so
  you'll match on `input.to_lowercase().as_str()`).

---

## Submitting for review

1. Check `cargo test -p lesson-02` (from `exercises/`) and try your REPL with `cargo run`.
2. Commit on your `work-02` branch: `git add -A && git commit -m "Lesson 02 complete"`.
3. Ask Claude: **"Please review lesson 02"**. Claude will look at `exercises/lesson-02`,
   your `ANSWERS.md`, and the `taskr` diff against `work-01`.

---

## Further reading

- The Rust Book, ch. 3.3 — [Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
- The Rust Book, ch. 3.5 — [Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
- The Rust Book, ch. 2 — [Programming a Guessing Game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html) (uses stdin, a nice warm-up)
- Rust by Example — [Flow of Control](https://doc.rust-lang.org/rust-by-example/flow_control.html)
