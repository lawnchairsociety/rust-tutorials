# Lesson 05 — Enums, `match` & `Option`

> Model "one of several possibilities" precisely, and let the compiler make sure you handle
> every case.
> Estimated time: 2.5 hours.

## What you'll learn

- Defining **enums**, including variants that carry data
- `match` in depth: exhaustiveness, destructuring, bindings, `|`, ranges, and **guards**
- `Option<T>` — Rust's replacement for null — and its most useful methods
- `if let`, `let ... else`, and `matches!`
- Writing methods on enums

---

## 1. Defining enums

A struct says "a task has a title **and** a priority **and** ...". An **enum** says "a
priority is `Low` **or** `Medium` **or** `High`":

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
enum Priority {
    Low,
    Medium,
    High,
}

fn main() {
    let p = Priority::High;
    println!("{p:?}");                  // High
    println!("{}", p == Priority::Low); // false
}
```

Each possibility is a **variant**, accessed as `Enum::Variant`. A value of type `Priority` is
*always* exactly one of the three — there's no way to make an invalid one. Compare that to
`priority: u8`, where `0` or `200` would compile just fine.

### Variants can carry data

```rust
#[derive(Debug)]
enum Message {
    Quit,                       // no data
    Move { x: i32, y: i32 },    // named fields, like a struct
    Write(String),              // one value, like a tuple struct
    ChangeColor(u8, u8, u8),    // several values
}

fn main() {
    let msgs = [
        Message::Quit,
        Message::Move { x: 1, y: 2 },
        Message::Write(String::from("hi")),
        Message::ChangeColor(255, 0, 0),
    ];
    println!("{msgs:#?}");
}
```

All four variants are the same type, `Message`, so they can live in one array. This is
perfect for modeling *commands* — each command has different arguments.

### Methods on enums

Enums get `impl` blocks just like structs:

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
enum Priority {
    Low,
    Medium,
    High,
}

impl Priority {
    fn label(&self) -> &str {
        match self {
            Priority::Low => "low",
            Priority::Medium => "med",
            Priority::High => "high",
        }
    }
}

fn main() {
    println!("{}", Priority::Medium.label()); // med
}
```

> Inside an `impl Priority` block you can also write `Self::Low` instead of `Priority::Low`.

---

## 2. `match` in depth

### Exhaustiveness

`match` must handle every variant. Forget one and the compiler refuses:

```rust
enum Coin { Penny, Nickel, Dime, Quarter }

fn value(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // ❌ error[E0004]: non-exhaustive patterns: `Coin::Quarter` not covered
    }
}
```

This is a superpower: when you add a new variant later (say, a new command), the compiler
points to every `match` you need to update.

### Destructuring data out of variants

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn describe(msg: &Message) -> String {
    match msg {
        Message::Quit => String::from("quit"),
        Message::Move { x, y } => format!("move to ({x}, {y})"),
        Message::Write(text) => format!("write {text:?}"),
        Message::ChangeColor(r, g, b) => format!("color #{r:02x}{g:02x}{b:02x}"),
    }
}

fn main() {
    println!("{}", describe(&Message::Move { x: 3, y: -1 }));
    println!("{}", describe(&Message::ChangeColor(255, 128, 0)));
}
```

Matching on a **reference** (`&Message`) gives you **references** to the fields (`text` is
a `&String`), so nothing is moved out. This is called *match ergonomics*.

### Patterns you can use

```rust
fn main() {
    let n = 7;
    let s = match n {
        0 => "zero",
        1 | 2 => "one or two",        // or-patterns
        3..=9 => "small",             // ranges
        _ => "big",                   // wildcard
    };
    println!("{s}");

    let point = (0, 5);
    match point {
        (0, 0) => println!("origin"),
        (0, y) => println!("on the y axis at {y}"),   // bind part of a tuple
        (x, 0) => println!("on the x axis at {x}"),
        (x, y) => println!("({x}, {y})"),
    }
}
```

### Match guards

Add an extra `if` condition to an arm:

```rust
fn classify(n: i32) -> &'static str {
    match n {
        x if x < 0 => "negative",
        0 => "zero",
        x if x % 2 == 0 => "positive even",
        _ => "positive odd",
    }
}

fn main() {
    for n in [-3, 0, 4, 7] {
        println!("{n}: {}", classify(n));
    }
}
```

### Binding with `@`

`name @ pattern` tests against a pattern and keeps the value:

```rust
fn main() {
    let age = 15;
    match age {
        a @ 13..=19 => println!("teenager, {a}"),
        a => println!("{a}"),
    }
}
```

### `match` on tuples of values

Very handy for combining two inputs:

```rust
fn main() {
    let (cmd, arg) = ("done", "3");
    match (cmd, arg) {
        ("list", "") => println!("list all"),
        ("done", id) => println!("complete {id}"),
        (other, _) => println!("unknown: {other}"),
    }
}
```

---

## 3. `Option<T>`: values that might be missing

Rust has **no null**. Instead, the standard library defines:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

(`<T>` means "for any type `T`" — generics are lesson 09.) `Option`, `Some`, and `None`
are always in scope, so you don't need the `Option::` prefix.

```rust
fn find_index(words: &[&str], target: &str) -> Option<usize> {
    for (i, w) in words.iter().enumerate() {
        if *w == target {
            return Some(i);
        }
    }
    None
}

fn main() {
    let words = ["apple", "pear", "plum"];
    match find_index(&words, "pear") {
        Some(i) => println!("found at {i}"),
        None => println!("not found"),
    }
}
```

The key: an `Option<usize>` is **not** a `usize`. You *cannot* accidentally use it as a number
— you must handle the `None` case first. That eliminates null-pointer bugs entirely.

You've already used `Option` without knowing it: `s.find(' ')` and `input.split_once(' ')`
return one.

### Useful `Option` methods

```rust
fn main() {
    let some: Option<i32> = Some(4);
    let none: Option<i32> = None;

    println!("{}", some.is_some());           // true
    println!("{}", none.is_none());           // true
    println!("{}", none.unwrap_or(0));        // 0 — a default
    println!("{:?}", some.map(|x| x * 2));    // Some(8) — transform the inside
    println!("{:?}", none.map(|x| x * 2));    // None
    println!("{:?}", some.filter(|x| *x > 10)); // None

    // and_then: chain operations that might each fail
    let parsed: Option<i32> = Some("42").and_then(|s| s.parse().ok());
    println!("{parsed:?}");                   // Some(42)

    // unwrap / expect: get the value or CRASH. Only use when None is truly impossible.
    println!("{}", some.unwrap());
    println!("{}", some.expect("some should be set"));
}
```

> `|x| x * 2` is a **closure** (an anonymous function). Lesson 11 covers them; for now
> read `|x| x * 2` as "given x, return x * 2".

### `Option<&T>` and `Option<&mut T>`

Returning a reference *to something you might not have* is a very common pattern:

```rust
#[derive(Debug)]
struct Book { id: u32, title: String }

struct Library { books: Vec<Book> }

impl Library {
    fn find(&self, id: u32) -> Option<&Book> {
        for book in &self.books {
            if book.id == id {
                return Some(book);
            }
        }
        None
    }

    fn find_mut(&mut self, id: u32) -> Option<&mut Book> {
        for book in &mut self.books {
            if book.id == id {
                return Some(book);
            }
        }
        None
    }

    fn remove(&mut self, id: u32) -> Option<Book> {
        // find the position, then remove it (Vec::remove returns the removed element)
        let mut index = None;
        for (i, book) in self.books.iter().enumerate() {
            if book.id == id {
                index = Some(i);
                break;
            }
        }
        match index {
            Some(i) => Some(self.books.remove(i)),
            None => None,
        }
    }
}

fn main() {
    let mut lib = Library { books: vec![Book { id: 1, title: "Dune".into() }] };
    if let Some(book) = lib.find_mut(1) {
        book.title.push_str(" Messiah");
    }
    println!("{:?}", lib.find(1));     // Some(Book { id: 1, title: "Dune Messiah" })
    println!("{:?}", lib.remove(1));   // Some(Book { ... }) — we now own it
    println!("{:?}", lib.find(1));     // None
}
```

Note `remove` returns `Option<Book>` (owned) while `find` returns `Option<&Book>` (borrowed).
The signature tells the caller exactly what they get.

The `match index { Some(i) => Some(...), None => None }` shape is so common that it has a
shortcut: `index.map(|i| self.books.remove(i))`.

---

## 4. `if let`, `let ... else`, and `matches!`

When you only care about **one** pattern, a full `match` is noisy.

### `if let`

```rust
fn main() {
    let maybe_name: Option<&str> = Some("Ferris");

    if let Some(name) = maybe_name {
        println!("Hello, {name}");
    } else {
        println!("Hello, stranger");
    }
}
```

### `let ... else` — bind or bail out

Great for "get the value or return early", which keeps the happy path un-indented:

```rust
fn initials(full_name: &str) -> Option<String> {
    let Some((first, last)) = full_name.split_once(' ') else {
        return None;            // the else block must exit (return, break, continue, panic)
    };
    let f = first.chars().next()?;  // `?` on an Option: return None if it's None
    let l = last.chars().next()?;
    Some(format!("{f}{l}"))
}

fn main() {
    println!("{:?}", initials("Ada Lovelace")); // Some("AL")
    println!("{:?}", initials("Plato"));        // None
}
```

The `?` operator here is a sneak peek of lesson 07: inside a function that returns `Option`,
`expr?` means "unwrap this, or return `None` right now".

### `matches!`

A quick boolean test against a pattern:

```rust
#[derive(Debug)]
enum Priority { Low, Medium, High }

fn main() {
    let p = Priority::High;
    let urgent = matches!(p, Priority::High | Priority::Medium);
    println!("{urgent}");
}
```

### `while let`

Loop as long as a pattern matches:

```rust
fn main() {
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {   // pop returns Option<T>
        println!("{top}");
    }
}
```

---

## 5. Parsing text into an enum

Turning user input into an enum is a pattern you'll use in taskr. Parse once into a
structured value, then `match` on the value everywhere else:

```rust
#[derive(Debug, PartialEq)]
enum Direction { North, South, East, West }

fn parse_direction(s: &str) -> Option<Direction> {
    match s {
        "n" | "north" => Some(Direction::North),
        "s" | "south" => Some(Direction::South),
        "e" | "east" => Some(Direction::East),
        "w" | "west" => Some(Direction::West),
        _ => None,
    }
}

#[derive(Debug, PartialEq)]
enum Action {
    Go(Direction),
    Look,
    Say(String),
    Unknown(String),
}

fn parse_action(input: &str) -> Action {
    let input = input.trim();
    let (word, rest) = input.split_once(' ').unwrap_or((input, ""));
    match word {
        "look" => Action::Look,
        "go" => match parse_direction(rest.trim()) {
            Some(dir) => Action::Go(dir),
            None => Action::Unknown(input.to_string()),
        },
        "say" if !rest.is_empty() => Action::Say(rest.trim().to_string()),
        _ => Action::Unknown(input.to_string()),
    }
}

fn main() {
    assert_eq!(parse_action("go north"), Action::Go(Direction::North));
    assert_eq!(parse_action("say hello there"), Action::Say("hello there".to_string()));
    assert_eq!(parse_action("go up"), Action::Unknown("go up".to_string()));
    println!("all good");
}
```

`unwrap_or((input, ""))` is a tidy replacement for the `match input.split_once(' ')` recipe
from lesson 03.

---

## Practice problems

All in `exercises/lesson-05/src/lib.rs`. Run: `cargo test -p lesson-05`.

1. **`Shape::area(&self) -> f64`** for `Circle { radius }`, `Rectangle { width, height }`,
   `Triangle { base, height }`. (Use `std::f64::consts::PI`.)
2. **`Coin::value_in_cents(&self) -> u32`**.
3. **`TrafficLight::next(&self) -> TrafficLight`** (Red → Green → Yellow → Red) and
   **`TrafficLight::seconds(&self) -> u32`** (Red 30, Green 25, Yellow 5).
4. **`safe_divide(a: i32, b: i32) -> Option<i32>`** — `None` when dividing by zero.
5. **`first_even(nums: &[i32]) -> Option<i32>`**.
6. **`parse_direction(s: &str) -> Option<Direction>`** — accept `n/north`, `s/south`,
   `e/east`, `w/west`, case-insensitive.
7. **`classify(n: i32) -> String`** — use **match guards**: `"negative"`, `"zero"`,
   `"small even"` / `"small odd"` for 1–9, `"large"` for 10 and up.
8. **`add_options(a: Option<i32>, b: Option<i32>) -> Option<i32>`** — the sum if both are
   `Some`, otherwise `None`. Try it with a `match` on the tuple `(a, b)`, then try with `?`.
9. **`describe(msg: &Message) -> String`** — see the doc comment for exact formats.
10. **`initials(full_name: &str) -> Option<String>`** — uppercase initials of the first and
    last word, using **`let ... else`**. One word → `None`.

Written questions (`exercises/lesson-05/ANSWERS.md`):

- **Q1.** Why is `Option<T>` safer than a null value? What does the compiler force you to
  do?
- **Q2.** Why is `priority: Priority` (an enum) better than `priority: u8`?
- **Q3.** When would you use `if let` instead of `match`? When would you use `let ... else`?
- **Q4.** What happens when you add a new variant to an enum that is used in several
  `match` expressions without `_` arms? Why is that useful? And why might a `_` arm hide
  bugs?

---

## Homework — taskr step 05

### Starting point

From lesson 04: `Task { id, title, priority: u8, done }` with `new`, `complete`, `summary`;
`TaskList { tasks, next_id }` with `new`, `add`, `complete`, `print`; commands `add`,
`done <id>`, `list`, `help`, `quit` dispatched by matching strings in `main`.

### Requirements

1. **Priority enum** — replace `priority: u8` with:
   ```rust
   #[derive(Debug, Clone, Copy, PartialEq)]
   enum Priority { Low, Medium, High }
   ```
   Add `fn label(&self) -> &str` returning `"low"`, `"med"`, `"high"`.
   `Task::summary` now shows the label: `#3 [ ] (med) Call mom`.
2. **`fn parse_priority(s: &str) -> Option<Priority>`** — accepts `"low"`, `"med"`,
   `"medium"`, `"high"`.
3. **Command enum** — parse input into a value *before* acting on it:
   ```rust
   #[derive(Debug, PartialEq)]
   enum Command {
       Add { title: String, priority: Priority },
       Done(u32),
       Remove(u32),
       Show(u32),
       List,
       Help,
       Quit,
       Unknown(String),
   }
   ```
   and **`fn parse_command(input: &str) -> Command`**. Rules:
   - `add <title>` → priority `Medium`. If the **last word** is `!low`, `!med`, or `!high`,
     it sets the priority and is removed from the title: `add Buy milk !high` →
     `Add { title: "Buy milk", priority: High }`.
   - `done <id>`, `remove <id>`, `show <id>` → the id parsed as `u32`.
   - `list`, `help`, `quit`.
   - Anything that doesn't fit (unknown word, missing/invalid id, `add` with no title) →
     `Unknown(<the trimmed input>)`. (Lesson 07 replaces this with proper error messages.)
4. `main`'s loop becomes: read line → `parse_command` → `match` on the `Command`. Empty input
   still just re-prompts (handle it before parsing, or make it a case in your design).
5. **TaskList** gains:
   - `fn add(&mut self, title: &str, priority: Priority) -> u32`
   - `fn find(&self, id: u32) -> Option<&Task>`
   - `fn find_mut(&mut self, id: u32) -> Option<&mut Task>`
   - `fn remove(&mut self, id: u32) -> Option<Task>`
   - rewrite `complete(&mut self, id: u32) -> bool` using `find_mut`.
6. New commands:
   - `remove <id>` → `Removed task #2: Write report` or `No task with id 2.`
   - `show <id>` → prints the details (see the example) or `No task with id 7.`
7. Start-up tasks: `Buy milk` (high, done), `Write report` (med), `Call mom` (low).
8. Ids are **never reused**: after removing #4, the next added task is still #5.

### Example session

```console
taskr> list
#1 [x] (high) Buy milk
#2 [ ] (med) Write report
#3 [ ] (low) Call mom
3 tasks, 1 done
taskr> add Pay rent !high
Added task #4: Pay rent
taskr> show 4
Task #4
  Title:    Pay rent
  Priority: high
  Status:   pending
taskr> remove 2
Removed task #2: Write report
taskr> remove 2
No task with id 2.
taskr> done abc
Unknown command: done abc (type 'help')
taskr> add Water plants
Added task #5: Water plants
taskr> list
#1 [x] (high) Buy milk
#3 [ ] (low) Call mom
#4 [ ] (high) Pay rent
#5 [ ] (med) Water plants
4 tasks, 1 done
```

### Hints

- To check and strip the last word: `title.rsplit_once(' ')` gives `Some((before, last))`.
  If `last` starts with `'!'` and `parse_priority(&last[1..])` is `Some(p)`, use `before` as
  the title. (`last.strip_prefix('!')` returns an `Option<&str>` too — try both!)
- `Command::Add { title, priority }` in a `match` arm destructures the fields.
- `let Some(task) = list.find(id) else { println!("No task with id {id}."); continue; };`
  is a neat way to write `show`.
- In `remove`, find the *position* first, then call `self.tasks.remove(index)`.
- Write a few `assert_eq!(parse_command("..."), Command::...)` checks at the top of `main`
  (or in a scratch function) to test your parser quickly — `#[derive(PartialEq)]` on
  `Command` makes that possible. Real tests come in lesson 10.

### Stretch goals

- Support the priority marker **anywhere** in the title (`add !high Pay rent`).
  This replaces the last-word rule in requirement 3. Lesson 06 makes it required.
- Add `undone <id>` (variant `Undone(u32)`).
- Add `priority <id> <low|med|high>` (variant `SetPriority(u32, Priority)`).
  This replaces lesson 04's `priority <id> <1-3>` stretch goal, if you did it.

---

## Submitting for review

1. `cargo test -p lesson-05` passes; `ANSWERS.md` filled in; `taskr` works.
2. Commit on `work-05`: `git add -A && git commit -m "Lesson 05 complete"`.
3. Ask Claude: **"Please review lesson 05"**.

---

## Further reading

- The Rust Book, ch. 6 — [Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html)
- The Rust Book, ch. 19 — [Patterns and Matching](https://doc.rust-lang.org/book/ch19-00-patterns.html) (reference for every pattern form)
- Rust by Example — [Enums](https://doc.rust-lang.org/rust-by-example/custom_types/enum.html), [match](https://doc.rust-lang.org/rust-by-example/flow_control/match.html)
- std docs — [`Option`](https://doc.rust-lang.org/std/option/enum.Option.html) (skim the method list!)
