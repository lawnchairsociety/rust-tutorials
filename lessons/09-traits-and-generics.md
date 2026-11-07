# Lesson 09 — Traits & Generics

> Write code once for many types, and plug your own types into the standard library's machinery. Estimated time: 3 hours.

## What you'll learn

- Generic functions, structs, enums and `impl` blocks
- Defining traits, implementing them, and writing default methods
- Trait bounds: `T: Trait`, `+`, `where` clauses, `impl Trait`
- The standard traits you'll use every day: `Debug`, `Clone`, `Copy`, `PartialEq`/`Eq`,
  `PartialOrd`/`Ord`, `Hash`, `Default`, `Display`, `From`/`Into`, `FromStr`
- What `#[derive(...)]` actually generates, and when to write an impl by hand
- How generics get compiled (monomorphization)

You've been using traits all along: `{:?}` needs `Debug`, `.clone()` needs `Clone`,
HashMap keys need `Hash + Eq`, and in lesson 7 you copied a `Display` recipe. Now you'll
see how they work.

---

## 1. Generics: one function, many types

Without generics you'd write one function per type:

```rust
fn largest_i32(v: &[i32]) -> i32 { /* ... */ 0 }
fn largest_char(v: &[char]) -> char { /* ... */ 'a' }
```

With a **type parameter** `T`, you write it once:

```rust
fn largest<T: PartialOrd + Copy>(items: &[T]) -> T {
    let mut best = items[0];
    for &item in items {
        if item > best {
            best = item;
        }
    }
    best
}

fn main() {
    println!("{}", largest(&[3, 7, 2]));        // 7
    println!("{}", largest(&['x', 'b', 'q']));  // x
}
```

`T: PartialOrd + Copy` is a **trait bound**. It says "any type `T`, as long as it can be
compared with `>` and copied". Without the bound, the compiler refuses `item > best`,
because it can't assume every type supports `>`:

```text
error[E0369]: binary operation `>` cannot be applied to type `T`
help: consider restricting type parameter `T` with trait `PartialOrd`
```

### Generic structs, enums and methods

```rust
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {                    // methods for every Point<T>
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {                     // methods only for Point<f64>
    fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

fn main() {
    let ints = Point { x: 1, y: 2 };
    let floats = Point { x: 3.0, y: 4.0 };
    println!("{} {}", ints.x(), floats.distance_from_origin()); // 1 5
}
```

You already know two generic enums very well:

```rust
enum Option<T> { Some(T), None }
enum Result<T, E> { Ok(T), Err(E) }
```

### Zero cost

The compiler generates a separate copy of a generic function for each concrete type you
use it with (`largest::<i32>`, `largest::<char>`). This is called **monomorphization**.
Generic code runs as fast as hand-written code. The costs are compile time and binary
size.

---

## 2. Traits: shared behavior

A **trait** defines a set of methods that a type can implement. It's similar to an
interface in other languages.

```rust
trait Summary {
    fn summarize_author(&self) -> String;          // required: every implementor writes it

    fn summarize(&self) -> String {                // default: implementors get it for free
        format!("(Read more from {}...)", self.summarize_author())
    }
}

struct Post {
    author: String,
    text: String,
}

struct Tweet {
    username: String,
}

impl Summary for Post {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
    fn summarize(&self) -> String {                // override the default
        format!("{}: {}", self.author, self.text)
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn main() {
    let p = Post { author: "Ann".into(), text: "Traits are neat".into() };
    let t = Tweet { username: "bob".into() };
    println!("{}", p.summarize());   // Ann: Traits are neat
    println!("{}", t.summarize());   // (Read more from @bob...)
}
```

### Taking "anything that implements X"

These three signatures mean almost the same thing:

```rust
fn notify(item: &impl Summary) { println!("Breaking! {}", item.summarize()); }

fn notify2<T: Summary>(item: &T) { println!("Breaking! {}", item.summarize()); }

fn notify3<T>(item: &T)
where
    T: Summary,                     // `where` is easier to read when bounds get long
{
    println!("Breaking! {}", item.summarize());
}
```

The difference shows when there are two parameters. `fn f(a: &impl Summary, b: &impl Summary)`
allows **different** types for `a` and `b`, while `fn f<T: Summary>(a: &T, b: &T)`
requires them to be the **same** type.

You can also **return** `impl Trait`: `fn make() -> impl Summary { Tweet { ... } }`. The
caller only knows "it's some type that implements Summary". This becomes really useful
with iterators in lesson 11.

### Two rules worth knowing

- **To call a trait's methods, the trait must be in scope.** If `x.summarize()` gives
  "method not found", add `use crate::path::Summary;`. That's why lesson 7's code had
  `use std::fmt;`, and why `std::io::Write` must be imported before you can call `flush()`.
- **The orphan rule**: you can implement trait `X` for type `Y` only if your crate
  defines `X` or `Y` (or both). You can implement `Display` for your `Task`, but you
  can't implement `Display` for `Vec<i32>`. This stops two crates from writing
  conflicting impls.

---

## 3. Deriving the standard traits

`#[derive(...)]` asks the compiler to write the obvious implementation for you:

| Trait | Gives you | Derive requires |
|---|---|---|
| `Debug` | `{:?}` and `{:#?}` formatting | all fields `Debug` |
| `Clone` | `.clone()` | all fields `Clone` |
| `Copy` | implicit copies instead of moves | `Clone`, and all fields `Copy` (so no `String`, no `Vec`) |
| `PartialEq` / `Eq` | `==`, `!=` | fields `PartialEq` (`Eq` means "equality is total", so no `f64`) |
| `PartialOrd` / `Ord` | `<`, `>`, `.cmp()`, `.sort()` | fields comparable; `Ord` needs `Eq` |
| `Hash` | usable as a `HashMap`/`HashSet` key | fields `Hash`; should agree with `Eq` |
| `Default` | `T::default()` | all fields `Default` |

For **enums**, the derived `PartialOrd`/`Ord` compares variants **in declaration order**,
earlier < later. That's exactly what we want for priorities:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Priority {
    Low,
    Medium,
    High,
}

fn main() {
    assert!(Priority::Low < Priority::High);
    let mut v = vec![Priority::High, Priority::Low, Priority::Medium];
    v.sort();
    println!("{v:?}"); // [Low, Medium, High]
}
```

For **structs**, derived ordering compares fields in order, like comparing words in a
dictionary: first field first, then the second if the first fields are equal, and so on.

---

## 4. Implementing standard traits by hand

### `Display`: user-facing text

Now you can read the recipe from lesson 7 properly:

```rust
use std::fmt;

struct Temperature {
    celsius: f64,
}

impl fmt::Display for Temperature {
    //     the trait's one required method; `f` is where output goes
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.1}°C", self.celsius)
    }
}

fn main() {
    let t = Temperature { celsius: 21.456 };
    println!("It is {t}");            // It is 21.5°C
    let s: String = t.to_string();    // to_string() comes free with Display
    println!("{s}");
}
```

`Display` can't be derived. Rust can't guess how *you* want your type shown to users.

Inside `fmt`, if you want to honour width and alignment flags like `{:<10}`, use
`f.pad(&text)` instead of `write!`. For most types, `write!` is fine.

### `Default`

```rust
#[derive(Debug)]
struct Settings {
    volume: u8,
    muted: bool,
}

impl Default for Settings {
    fn default() -> Self {        // `Self` means "the type this impl is for"
        Settings { volume: 50, muted: false }
    }
}

fn main() {
    let s = Settings { muted: true, ..Default::default() }; // struct update syntax
    println!("{s:?}");
}
```

When all fields have sensible defaults (zero, empty, false), just `#[derive(Default)]`.
Clippy will even suggest `impl Default` for any type with a `new()` that takes no
arguments.

### `From` and `Into`: conversions

```rust
struct Celsius(f64);
struct Fahrenheit(f64);

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

fn main() {
    let f = Fahrenheit::from(Celsius(100.0));
    let g: Fahrenheit = Celsius(0.0).into();   // implementing From gives you Into for free
    println!("{} {}", f.0, g.0);               // 212 32
}
```

`From` has a special job in error handling: **`?` calls `From::from` on the error.** So if
`impl From<ParseIntError> for MyError` exists, a `ParseIntError` passes through `?` in a
function returning `Result<_, MyError>` and gets converted automatically:

```rust
use std::num::ParseIntError;

#[derive(Debug)]
enum MyError {
    BadNumber(ParseIntError),
}

impl From<ParseIntError> for MyError {
    fn from(e: ParseIntError) -> Self {
        MyError::BadNumber(e)
    }
}

fn double(s: &str) -> Result<i32, MyError> {
    let n: i32 = s.parse()?;     // ParseIntError -> MyError via From
    Ok(n * 2)
}

fn main() {
    println!("{:?}", double("21"));
    println!("{:?}", double("x"));
}
```

### `FromStr`: making `.parse()` work for your type

`"42".parse::<i32>()` works because `i32` implements `FromStr`. Your types can too:

```rust
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Direction { North, South }

impl FromStr for Direction {
    type Err = String;           // an "associated type": the error type for this impl

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "n" | "north" => Ok(Direction::North),
            "s" | "south" => Ok(Direction::South),
            other => Err(format!("unknown direction: {other}")),
        }
    }
}

fn main() {
    let d: Direction = "North".parse().unwrap();
    assert_eq!(d, Direction::North);
    assert!("up".parse::<Direction>().is_err());
}
```

### `Ord` and custom sort orders

`Ord::cmp` returns a `std::cmp::Ordering` (`Less`, `Equal` or `Greater`). To sort by
something other than the natural order, pass a comparison to `sort_by`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Priority { Low, Medium, High }

#[derive(Debug)]
struct Job { id: u32, priority: Priority }

fn main() {
    let mut jobs = vec![
        Job { id: 1, priority: Priority::Low },
        Job { id: 2, priority: Priority::High },
        Job { id: 3, priority: Priority::High },
    ];

    // highest priority first; ties broken by id ascending
    jobs.sort_by(|a, b| b.priority.cmp(&a.priority).then(a.id.cmp(&b.id)));
    println!("{jobs:?}"); // ids in order: 2, 3, 1

    jobs.sort_by_key(|j| j.id); // back to id order
}
```

The `|a, b| ...` is a closure again (lesson 11). `sort_by` calls it to compare two
elements. Swapping `a` and `b` in `b.priority.cmp(&a.priority)` reverses the order.

---

## 5. Trait objects: a sneak peek

Generics work out the concrete type at **compile time**. Sometimes you need a list of
*different* types that share a trait, like `Vec<Box<dyn Shape>>`. That's a **trait
object**, and lesson 13 covers it in depth. For now: `impl Trait`/generics are the
default; reach for `dyn Trait` when the type really is only known at runtime.

---

## Practice problems

In `exercises/lesson-09/src/lib.rs`:

```bash
cd exercises && cargo test -p lesson-09
```

1. **`largest`**: a generic function that returns `Option<T>`.
2. **`Pair<T>`**: generic struct methods, plus a method that only exists when `T: PartialOrd`.
3. **`impl Shape for Circle / Rect`**: required methods, and overriding a default.
4. **`total_area`**: a generic function with a trait bound.
5. **`describe_bigger`**: two `impl Shape` arguments of *different* types.
6. **`Display for Point`**.
7. **`From<(u8, u8, u8)> for Rgb`**: then `.into()` works too.
8. **`FromStr for Rgb`**: parse `#rrggbb`.
9. **`Level`**: choose the right derives, implement `sort_desc`, and **uncomment the test**.
10. **`Default for Config`**: by hand, with non-zero defaults.

Written questions in `exercises/lesson-09/ANSWERS.md`:

11. Why can't you `#[derive(Copy)]` on a struct that contains a `String`?
12. What's the difference between `fn f(a: &impl Shape, b: &impl Shape)` and `fn f<S: Shape>(a: &S, b: &S)`?
13. Explain the orphan rule in your own words. Could you `impl fmt::Display for Vec<Task>` in taskr? What could you do instead?
14. What is monomorphization, and what are its trade-offs?

---

## Homework — taskr step 09: speak Rust's trait language

### Starting point

After lesson 08, `taskr` is a library (`lib.rs`, `task.rs`, `task_list.rs`, `command.rs`,
`error.rs`) plus a thin `main.rs`. `Task` has private fields and getters. Parsing goes
through `parse_command(&str) -> Result<Command, ParseError>` and `parse_priority`.
Errors already implement `Display`.

### Requirements

1. **`Display for Priority`**: prints `low`, `medium` or `high`. You choose the exact words.
2. **`Display for Task`**: prints the one-line form used by `list`, e.g.
   `#2 [ ] Buy milk (high) #errand #home`. Replace your old `summary()` method; `list`
   becomes `println!("{task}")`. Remove `summary()` once nothing uses it.
3. **`FromStr for Priority`** with `type Err = ParseError`. Accept `low`, `med`, `medium`
   and `high`, case-insensitive, **without** the `!`. Strip the `!` in the command parser
   before calling `.parse::<Priority>()`. Remove `parse_priority` or turn it into a thin
   wrapper.
4. **`FromStr for Command`** with `type Err = ParseError`. Move the body of
   `parse_command` into `from_str`, so `main.rs` can write `line.parse::<Command>()`.
   (Keep `parse_command` as a one-line wrapper if you like.)
5. **Derives.** `Priority` derives `PartialOrd, Ord` (plus what it had). Order the variants
   so that `Low < Medium < High`. `Task` and `TaskList` derive `Debug` and `Clone`.
   Errors and `Command` derive `Debug` and `PartialEq` (lesson 10's tests will need it).
6. **`Default for TaskList`**: derive or implement it so `TaskList::default()` is an empty
   list whose first id will be 1. `new()` can just return `Self::default()`.
7. **`sort` command.** `sort priority` reorders the stored list: highest priority first,
   ties broken by id. `sort id` restores id order. Model the argument as
   `enum SortKey { Id, Priority }` with its own `FromStr` (error:
   `MissingArgument("sort key")` or a new `ParseError` variant of your choice). Add
   `Command::Sort(SortKey)` and `TaskList::sort(&mut self, key: SortKey)`.
8. **A generic helper.** In `main.rs`, write
   `fn print_numbered<T: std::fmt::Display>(items: &[T])` that prints `1. item` lines, and
   use it somewhere sensible, e.g. to print the `help` text from an array of strings.
9. *(Optional, but recommended)* If you have code that does
   `.parse::<u32>().map_err(...)` in several places, try `impl From<ParseIntError> for ParseError`
   instead and let `?` do the conversion. Note what you lose: the original text of the bad
   id. Decide which version you prefer and leave a comment explaining why.

### Example session

```text
taskr> add Write report !low #work
Added task #1
taskr> add Buy milk #errand !high
Added task #2
taskr> add Fix bike
Added task #3
taskr> sort priority
Sorted by priority
taskr> list
#2 [ ] Buy milk (high) #errand
#3 [ ] Fix bike (medium)
#1 [ ] Write report (low) #work
taskr> sort id
Sorted by id
taskr> sort colour
Error: unknown sort key: colour (use 'id' or 'priority')
taskr> add Test !HIGH
Added task #4
```

### Hints

- `impl FromStr for Command` lives in `command.rs`; add `use std::str::FromStr;` there.
- Inside `from_str`, `"high".parse::<Priority>()?` returns a `Result<Priority, ParseError>`,
  so `?` works with no conversion needed.
- Since `Task`'s fields are private, `TaskList::sort` must use getters:
  `self.tasks.sort_by(|a, b| b.priority().cmp(&a.priority()).then(a.id().cmp(&b.id())))`.
- `Display for Task` needs to print tags. Loop over them and `write!(f, " #{tag}")?` for
  each one. Note the `?` here: `write!` returns a `fmt::Result`.

### Stretch goals

- `impl Display for Command` that prints a command back in canonical text form. Then
  `cmd.to_string().parse::<Command>()` should give back the same `cmd`, which is a nice
  property to test next lesson.
- A `trait Describe { fn describe(&self) -> String; }` implemented for `Task` and
  `TaskList`, used by `show`.

---

## Submitting for review

1. `cd exercises && cargo test -p lesson-09` passes, including the uncommented `p9` test.
   `taskr` builds without warnings.
2. Commit on `work-09`:
   ```bash
   git add -A && git commit -m "Lesson 09: traits and generics"
   ```
3. Ask Claude: **"Please review lesson 09"**.

## Further reading

- The Rust Book, ch. 10.1–10.2 — [Generic Types, Traits](https://doc.rust-lang.org/book/ch10-00-generics.html)
- The Rust Book, appendix C — [Derivable Traits](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html)
- Rust by Example — [Generics](https://doc.rust-lang.org/rust-by-example/generics.html), [Traits](https://doc.rust-lang.org/rust-by-example/trait.html)
- std docs — [`std::fmt`](https://doc.rust-lang.org/std/fmt/index.html), [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html), [`From`](https://doc.rust-lang.org/std/convert/trait.From.html), [`Ord`](https://doc.rust-lang.org/std/cmp/trait.Ord.html)
